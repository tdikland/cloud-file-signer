use std::time::Duration;

use azure_storage_blobs::prelude::ClientBuilder;
use cloud_file_signer::{azure::AbfsFileSigner, CloudFileSigner};
use tokio::runtime::Runtime;

struct MockAbfs<'a> {
    rt: &'a Runtime,
    container: String,
    cb: ClientBuilder,
}

impl<'a> MockAbfs<'a> {
    fn setup(rt: &'a Runtime, container_name: impl Into<String> + Clone) -> Self {
        let cb = ClientBuilder::emulator();

        let container_client = cb.clone().container_client(container_name.clone());
        rt.block_on(container_client.create().into_future())
            .unwrap();

        Self {
            rt,
            container: container_name.into(),
            cb,
        }
    }

    fn put_blob(&self, key: &str) {
        let blob_client = self.cb.clone().blob_client(&self.container, key);
        self.rt
            .block_on(blob_client.put_block_blob("hello world").into_future())
            .unwrap();
    }

    fn teardown(&self) {
        let container_client = self.cb.clone().container_client(&self.container);
        self.rt
            .block_on(container_client.delete().into_future())
            .unwrap();
    }
}

impl Drop for MockAbfs<'_> {
    fn drop(&mut self) {
        self.teardown();
    }
}

#[test]
fn test_abfs_signer() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let mock_abfs = MockAbfs::setup(&rt, "mycontainer");
    mock_abfs.put_blob("path/myfile");

    let cb = ClientBuilder::emulator();
    let signer = AbfsFileSigner::from_client_builder("devstoreaccount1", cb);

    let uri = "abfss://mycontainer@devstoreaccount1.dfs.core.windows.net/path/myfile";
    let presigned_url = rt
        .block_on(signer.sign_read_only_starting_now(uri, Duration::from_secs(3600)))
        .unwrap();

    let c = reqwest::blocking::Client::builder().build().unwrap();
    let res = c.get(presigned_url.url()).send().unwrap().bytes().unwrap();

    assert_eq!(res, "hello world");
}
