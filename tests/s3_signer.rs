use std::thread::sleep;
use std::time::Duration;

use aws_config::{BehaviorVersion, Region, SdkConfig};
use aws_sdk_s3::types::{Delete, ObjectIdentifier};
use aws_sdk_s3::{config::Credentials, primitives::ByteStream, Client};
use reqwest::StatusCode;
use tokio::runtime::Runtime;

use cloud_file_signer::aws::S3FileSigner;
use cloud_file_signer::CloudFileSigner;

struct MockS3<'a> {
    rt: &'a Runtime,
    conf: SdkConfig,
    bucket: String,
}

impl<'a> MockS3<'a> {
    fn setup(async_runtime: &'a Runtime) -> Self {
        let bucket_name = format!("my-test-bucket-{}", uuid::Uuid::new_v4());
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
        let this = Self {
            conf,
            rt: async_runtime,
            bucket: bucket_name.into(),
        };
        this.create_bucket();
        this
    }

    fn bucket(&self) -> &str {
        &self.bucket
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
    let mock_s3 = MockS3::setup(&rt);

    // Read an object using a valid presigned URL.
    mock_s3.put_object("my-read-only-key");
    let s3_url = format!("s3://{}/my-read-only-key", mock_s3.bucket());
    let s3_signer = S3FileSigner::new(mock_s3.client());
    let presigned_url = rt
        .block_on(s3_signer.sign_read_only_starting_now(&s3_url, Duration::from_secs(3600)))
        .unwrap();

    let c = reqwest::blocking::Client::builder().build().unwrap();
    let res = c.get(presigned_url.url()).send().unwrap().bytes().unwrap();
    assert_eq!(res, "hello world");
}

#[test]
fn test_s3_signer_expired_url() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let mock_s3 = MockS3::setup(&rt);

    // Read an object using a expired presigned URL.
    mock_s3.put_object("my-expired-key");
    let s3_url = format!("s3://{}/my-expired-key", mock_s3.bucket());
    let s3_signer = S3FileSigner::new(mock_s3.client());
    let presigned_url = rt
        .block_on(s3_signer.sign_read_only_starting_now(&s3_url, Duration::from_secs(1)))
        .unwrap();

    sleep(Duration::from_secs(5));
    let c = reqwest::blocking::Client::builder().build().unwrap();
    let res = c.get(presigned_url.url()).send().unwrap().status();
    assert_eq!(res, StatusCode::FORBIDDEN);
}
