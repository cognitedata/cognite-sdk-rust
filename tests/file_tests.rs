use cognite::files::*;
use cognite::prelude::*;
mod common;
use common::*;

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
        .upload_stream("text/plain", &res.upload_url.unwrap(), stream)
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
