use cloud_file_signer::azure;
use cloud_file_signer::CloudFileSigner;

use azure_core::error::{ErrorKind, ResultExt};
use azure_storage::{prelude::*, shared_access_signature};
use azure_storage_blobs::prelude::*;

fn main() {
    let cb = ClientBuilder::emulator();

    // let container_client = cb.clone().container_client("container1");
    // container_client.create().into_future().await.unwrap();

    // let blob_client = cb.clone().blob_client("container1", "blob2");
    // blob_client.put_block_blob("hello world2").await.unwrap();

    // let signer = azure::AbfsFileSigner::emulator();
    // let presigned_url = signer.sign("container1/blob1").await.unwrap();
    // println!("presigned_url: {:?}", presigned_url);
}

// http://127.0.0.1:10000/devstoreaccount1/container1/blob1?sv=2020-06-12&sp=&sr=b&se=2023-12-09T22%3A10%3A35Z&sig=u4Kp8enrNc6AOyFrbe5nGKkscbXOCZPz1oOu3JWSqz8%3D
// http://127.0.0.1:10000/devstoreaccount1/container1/blob1?sv=2020-06-12&sp=&sr=b&se=2023-12-09T22%3A10%3A35Z&sig=u4Kp8enrNc6AOyFrbe5nGKkscbXOCZPz1oOu3JWSqz8%3D
// http://127.0.0.1:10000/devstoreaccount1/container1/blob1?sv=2020-06-12&sp=&sr=b&se=2023-12-09T22%3A10%3A35Z&sig=u4Kp8enrNc6AOyFrbe5nGKkscbXOCZPz1oOu3JWSqz8%3D

// http://127.0.0.1:10000/devstoreaccount1/container1/blob1?sv=2020-06-12&sp=&sr=b&se=2023-12-09T22%3A10%3A35Z&sig=u4Kp8enrNc6AOyFrbe5nGKkscbXOCZPz1oOu3JWSqz8%3D
// http://127.0.0.1:10000/devstoreaccount1/container1/blob1?sv=2018-03-28&st=2023-12-09T20%3A56%3A21Z&se=2023-12-10T20%3A56%3A21Z&sr=b&sp=r&sig=l36h1wPq7smSS4Ft5QIUGZEDGDhNRUKhfigO%2F%2BZmk2o%3D
// http://127.0.0.1:10000/devstoreaccount1/container1/blob1?sv=2020-06-12&sp=r&sr=b&se=2023-12-10T00%3A43%3A19Z&st=2023-12-09T21%3A43%3A19Z&sig=CjV9Ii97W5RlcsLI4%2BqhB%2BMQA90cw%2B7XvYtXfkHWrkk%3D
// http://127.0.0.1:10000/devstoreaccount1/container1/blob1?sv=2020-06-12&sp=&sr=b&se=2023-12-09T22%3A19%3A41Z&sig=IbzDDVoiq6RSgSOzB2YK6Jot%2F7s2Ok9hrzJM7svl%2Brk%3D
