use futures::Stream;

use crate::api::resource::*;
use crate::dto::core::files::*;
use crate::dto::items::Items;
use crate::error::Result;
use crate::{Identity, ItemsWithoutCursor, Patch};

pub type Files = Resource<FileMetadata>;

impl WithBasePath for Files {
    const BASE_PATH: &'static str = "files";
}

impl FilterItems<FileFilter, FileMetadata> for Files {}
impl<'a> SearchItems<'a, FileFilter, FileSearch, FileMetadata> for Files {}
impl RetrieveWithIgnoreUnknownIds<Identity, FileMetadata> for Files {}
impl Delete<Identity> for Files {}
impl Update<Patch<PatchFile>, FileMetadata> for Files {}

impl Files {
    /// Upload a stream to an url, the url is received from `Files::upload`
    /// For example:
    /// ```ignore
    /// use tokio_util::codec::{BytesCodec, FramedRead};
    ///
    /// let file = std::io::File::new("my-file");
    /// let stream = FramedRead::new(file, BytesCodec::new());
    /// cognite_client.upload_stream(&file.mime_type.unwrap(), &file.upload_url, stream, true).await?;
    /// ```
    ///
    /// Note that `stream_chunked` being true is in general more efficient, but it is not supported
    /// for the azure file backend. Setting it to false results in the entire stream being read into memory before uploading.
    pub async fn upload_stream<S>(
        &self,
        mime_type: &str,
        url: &str,
        stream: S,
        stream_chunked: bool,
    ) -> Result<()>
    where
        S: futures::TryStream + Send + Sync + 'static,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
        bytes::Bytes: From<S::Ok>,
    {
        self.api_client
            .put_stream(url, mime_type, stream, stream_chunked)
            .await
    }

    pub async fn upload(&self, overwrite: bool, item: &AddFile) -> Result<FileMetadata> {
        self.api_client
            .post_with_query("files", item, Some(FileUploadQuery::new(overwrite)))
            .await
    }

    pub async fn download_link(&self, ids: &[Identity]) -> Result<Vec<FileDownloadUrl>> {
        let items = Items::from(ids);
        let file_links_response: ItemsWithoutCursor<FileDownloadUrl> =
            self.api_client.post("files/downloadlink", &items).await?;
        Ok(file_links_response.items)
    }

    pub async fn download(
        &self,
        url: &str,
    ) -> Result<impl Stream<Item = std::result::Result<bytes::Bytes, reqwest::Error>>> {
        self.api_client.get_stream(url).await
    }

    pub async fn download_file(
        &self,
        id: Identity,
    ) -> Result<impl Stream<Item = std::result::Result<bytes::Bytes, reqwest::Error>>> {
        let items = vec![id];
        let links = self.download_link(&items).await?;
        let link = links.first().unwrap();
        self.download(&link.download_url).await
    }
}
