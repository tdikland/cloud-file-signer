// use std::time::Duration;

// use google_cloud_storage::client::{Client, ClientConfig};
// use tokio::runtime::Runtime;

// use cloud_file_signer::{CloudFileSigner, Permission};

// struct MockGcs<'a> {
//     rt: &'a Runtime,
//     client: Client,
//     bucket: String,
// }

// impl<'a> MockGcs<'a> {
//     fn setup(async_runtime: &'a Runtime, bucket_name: impl Into<String>) -> Self {
//         let cfg = ClientConfig::default().with_credentials();
//         let client_conf = async_runtime.block_on(cfg).unwrap();
//         let client = Client::new(client_conf);

//         Self {
//             client,
//             rt: async_runtime,
//             bucket: bucket_name.into(),
//         }
//     }

//     fn client(&self) -> Client {
//         self.client.clone()
//     }

//     fn create_bucket(&self) {
//         let req = self.client().create_bucket().bucket(&self.bucket).send();
//         self.rt.block_on(req).unwrap();
//     }

//     fn put_object(&self, key_name: &str) {
//         let req = self
//             .client()
//             .put_object()
//             .bucket(&self.bucket)
//             .key(key_name)
//             .body(ByteStream::from_static("hello world".as_bytes()))
//             .send();
//         self.rt.block_on(req).unwrap();
//     }
// }

// impl Drop for MockS3<'_> {
//     fn drop(&mut self) {
//         let req = self.client().list_objects_v2().bucket(&self.bucket).send();
//         let obj = self.rt.block_on(req).unwrap();
//         let objects_to_delete = obj
//             .contents()
//             .iter()
//             .fold(Delete::builder(), |delete, obj| {
//                 delete.objects(
//                     ObjectIdentifier::builder()
//                         .key(obj.key().unwrap())
//                         .build()
//                         .unwrap(),
//                 )
//             })
//             .build()
//             .unwrap();

//         let req = self
//             .client()
//             .delete_objects()
//             .bucket(&self.bucket)
//             .delete(objects_to_delete)
//             .send();
//         self.rt.block_on(req).unwrap();

//         let req = self.client().delete_bucket().bucket(&self.bucket).send();
//         self.rt.block_on(req).unwrap();
//     }
// }

// #[test]
// fn test_s3_signer() {
//     let rt = tokio::runtime::Builder::new_current_thread()
//         .enable_all()
//         .build()
//         .unwrap();
//     let mock_s3 = MockS3::setup(&rt, "mybucket");
//     mock_s3.create_bucket();
//     mock_s3.put_object("mykey");

//     let s3_url = S3Uri::new("mybucket".to_string(), "mykey".to_string(), None).to_url();
//     let s3_signer = S3FileSigner::new(mock_s3.client());
//     let presigned_url = rt
//         .block_on(s3_signer.sign(&s3_url, Duration::from_secs(3600), Permission::Read))
//         .unwrap();

//     let c = reqwest::blocking::Client::builder().build().unwrap();
//     let res = c.get(presigned_url.url()).send().unwrap().bytes().unwrap();

//     assert_eq!(res, "hello world");
// }
