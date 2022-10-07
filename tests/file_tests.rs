use bytes::Bytes;
use cognite::files::*;
use cognite::prelude::*;
mod common;
use common::*;
use futures::TryStreamExt;

async fn ensure_test_file(client: &CogniteClient) {
    let id = "rust-sdk-test-file".to_string();
    let new_file = AddFile {
        name: "Rust SDK test file".to_string(),
        external_id: Some(id),
        mime_type: Some("text/plain".to_string()),
        ..Default::default()
    };

    let file = match client.files.upload(false, &new_file).await {
        Err(cognite::Error { kind }) => match kind {
            cognite::Kind::Conflict(_) => return,
            e => panic!("{}", e),
        },
        Ok(f) => f,
    };

    let chunks: Vec<Result<_, ::std::io::Error>> = vec![Ok("test "), Ok("file "), Ok("contents")];
    let stream = futures::stream::iter(chunks);

    client
        .files
        .upload_stream("text/plain", &file.upload_url.unwrap(), stream, false)
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
        .upload_stream("text/plain", &res.upload_url.unwrap(), stream, false)
        .await
        .unwrap();

    client
        .files
        .delete(&vec![Identity::Id { id: res.id }])
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

    res.source = Some("New source".to_string());

    let upd_res = client.files.update_from(&vec![res]).await.unwrap();

    let upd_res = upd_res.first().unwrap();

    assert_eq!(Some("New source".to_string()), upd_res.source);

    client
        .files
        .delete(&vec![Identity::ExternalId { external_id: id }])
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

    let data: Vec<u8> = data.into_iter().flat_map(|b| b).collect();
    let contents = String::from_utf8(data).unwrap();

    assert_eq!("test file contents", contents.as_str())
}
