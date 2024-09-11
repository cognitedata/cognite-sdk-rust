#![cfg(feature = "integration_tests")]

#[cfg(test)]
use bytes::Bytes;
use cognite::files::*;
use cognite::prelude::*;
use futures::TryStreamExt;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

mod common;
pub use common::*;

async fn ensure_test_file(client: &CogniteClient) {
    let id = "rust-sdk-test-file".to_string();
    let new_file = AddFile {
        name: "Rust SDK test file".to_string(),
        external_id: Some(id),
        mime_type: Some("text/plain".to_string()),
        ..Default::default()
    };

    let file = match client.files.upload(false, &new_file).await {
        Err(cognite::Error::Conflict(_)) => return,
        Err(e) => panic!("{}", e),
        Ok(f) => f,
    };

    let chunks: Vec<Result<_, ::std::io::Error>> = vec![Ok("test "), Ok("file "), Ok("contents")];
    let stream = futures::stream::iter(chunks);

    client
        .files
        .upload_stream("text/plain", &file.extra.upload_url, stream, false)
        .await
        .unwrap();
}

#[tokio::test]
async fn create_upload_delete_file() {
    let id = format!("{}-file1", PREFIX.as_str());
    let new_file = AddFile {
        name: "File 1".to_string(),
        external_id: Some(id),
        mime_type: Some("text/plain".to_string()),
        ..Default::default()
    };

    let client = get_client();

    let res = client.files.upload(true, &new_file).await.unwrap();

    let chunks: Vec<Result<_, ::std::io::Error>> = vec![Ok("test "), Ok("file "), Ok("contents")];

    let stream = futures::stream::iter(chunks);

    client
        .files
        .upload_stream("text/plain", &res.extra.upload_url, stream, false)
        .await
        .unwrap();

    client
        .files
        .delete(&[Identity::Id {
            id: res.metadata.id,
        }])
        .await
        .unwrap();
}

#[tokio::test]
async fn create_upload_delete_actual_file() {
    let id = format!("{}-file3", PREFIX.as_str());
    let new_file = AddFile {
        name: "File 1".to_string(),
        external_id: Some(id),
        mime_type: Some("text/plain".to_string()),
        ..Default::default()
    };

    let client = get_client();

    let res = client.files.upload(true, &new_file).await.unwrap();

    let size = tokio::fs::metadata("tests/dummyfile.txt")
        .await
        .unwrap()
        .len();
    let file = File::open("tests/dummyfile.txt").await.unwrap();
    let stream = FramedRead::new(file, BytesCodec::new());
    client
        .files
        .upload_stream_known_size("text/plain", &res.extra.upload_url, stream, size)
        .await
        .unwrap();

    client
        .files
        .delete(&[Identity::Id {
            id: res.metadata.id,
        }])
        .await
        .unwrap();
}

#[tokio::test]
async fn create_update_delete_file() {
    let id = format!("{}-file2", PREFIX.as_str());
    let new_file = AddFile {
        name: "File 2".to_string(),
        external_id: Some(id.clone()),
        mime_type: Some("text/plain".to_string()),
        ..Default::default()
    };

    let client = get_client();

    let mut res = client.files.upload(true, &new_file).await.unwrap();

    res.metadata.source = Some("New source".to_string());

    let upd_res = client.files.update_from(&vec![res.metadata]).await.unwrap();

    let upd_res = upd_res.first().unwrap();

    assert_eq!(Some("New source".to_string()), upd_res.source);

    client
        .files
        .delete(&[Identity::ExternalId { external_id: id }])
        .await
        .unwrap();
}

#[tokio::test]
async fn download_test_file() {
    let client = get_client();

    ensure_test_file(&client).await;

    let data: Vec<Bytes> = client
        .files
        .download_file(Identity::ExternalId {
            external_id: "rust-sdk-test-file".to_string(),
        })
        .await
        .unwrap()
        .try_collect()
        .await
        .unwrap();

    let data: Vec<u8> = data.into_iter().flatten().collect();
    let contents = String::from_utf8(data).unwrap();

    assert_eq!("test file contents", contents.as_str())
}

#[tokio::test]
async fn create_multipart_file() {
    let id = format!("{}-multipart", PREFIX.as_str());
    let new_file = AddFile {
        name: "Multipart File".to_owned(),
        external_id: Some(id.clone()),
        mime_type: Some("text/plain".to_owned()),
        ..Default::default()
    };

    let client = get_client();

    let content_1 = "abcde".repeat(1_200_000);
    let content_2 = "fghij";
    let (session, _) = client
        .files
        .multipart_upload(true, 2, &new_file)
        .await
        .unwrap();
    session.upload_part_blob(0, content_1).await.unwrap();
    session.upload_part_blob(1, content_2).await.unwrap();

    session.complete().await.unwrap();

    let data: Vec<Bytes> = client
        .files
        .download_file(Identity::ExternalId {
            external_id: id.to_owned(),
        })
        .await
        .unwrap()
        .try_collect()
        .await
        .unwrap();
    let data: Vec<u8> = data.into_iter().flatten().collect();
    assert_eq!(1_200_000 * 5 + 5, data.len());
    assert!(data.ends_with("fghij".as_bytes()));

    client
        .files
        .delete(&[Identity::ExternalId { external_id: id }])
        .await
        .unwrap();
}
