use std::time::Duration;

use aws_config::{BehaviorVersion, Region, SdkConfig};
use aws_sdk_s3::types::{Delete, ObjectIdentifier};
use aws_sdk_s3::{config::Credentials, primitives::ByteStream, Client};
use tokio::runtime::Runtime;

use cloud_file_signer::aws::S3FileSigner;
use cloud_file_signer::CloudFileSigner;

struct MockS3<'a> {
    rt: &'a Runtime,
    conf: SdkConfig,
    bucket: String,
}

impl<'a> MockS3<'a> {
    fn setup(async_runtime: &'a Runtime, bucket_name: impl Into<String>) -> Self {
        let shared_conf = aws_config::defaults(BehaviorVersion::latest())
            .credentials_provider(Credentials::from_keys(
                "delta-sharing",
                "sharing-is-caring",
                None,
            ))
            .endpoint_url("http://127.0.0.1:4566")
            .region(Region::new("us-east-1"))
            .load();

        let conf = async_runtime.block_on(shared_conf);

        Self {
            conf,
            rt: async_runtime,
            bucket: bucket_name.into(),
        }
    }

    fn client(&self) -> Client {
        let s3_conf_builder = aws_sdk_s3::config::Builder::from(&self.conf);
        aws_sdk_s3::Client::from_conf(s3_conf_builder.build())
    }

    fn create_bucket(&self) {
        let req = self.client().create_bucket().bucket(&self.bucket).send();
        self.rt.block_on(req).unwrap();
    }

    fn put_object(&self, key_name: &str) {
        let req = self
            .client()
            .put_object()
            .bucket(&self.bucket)
            .key(key_name)
            .body(ByteStream::from_static("hello world".as_bytes()))
            .send();
        self.rt.block_on(req).unwrap();
    }
}

impl Drop for MockS3<'_> {
    fn drop(&mut self) {
        let req = self.client().list_objects_v2().bucket(&self.bucket).send();
        let obj = self.rt.block_on(req).unwrap();
        let objects_to_delete = obj
            .contents()
            .iter()
            .fold(Delete::builder(), |delete, obj| {
                delete.objects(
                    ObjectIdentifier::builder()
                        .key(obj.key().unwrap())
                        .build()
                        .unwrap(),
                )
            })
            .build()
            .unwrap();

        let req = self
            .client()
            .delete_objects()
            .bucket(&self.bucket)
            .delete(objects_to_delete)
            .send();
        self.rt.block_on(req).unwrap();

        let req = self.client().delete_bucket().bucket(&self.bucket).send();
        self.rt.block_on(req).unwrap();
    }
}

#[test]
fn test_s3_signer() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let mock_s3 = MockS3::setup(&rt, "mybucket");
    mock_s3.create_bucket();
    mock_s3.put_object("mykey");

    let s3_url = "s3://mybucket/mykey";
    let s3_signer = S3FileSigner::new(mock_s3.client());
    let presigned_url = rt
        .block_on(s3_signer.sign_read_only_starting_now(s3_url, Duration::from_secs(3600)))
        .unwrap();

    let c = reqwest::blocking::Client::builder().build().unwrap();
    let res = c.get(presigned_url.url()).send().unwrap().bytes().unwrap();

    assert_eq!(res, "hello world");
}
