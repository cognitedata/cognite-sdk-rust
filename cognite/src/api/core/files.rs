use futures::TryStream;

use crate::api::resource::*;
use crate::dto::core::files::*;
use crate::dto::items::Items;
use crate::error::Result;
use crate::PartitionedFilter;
use crate::{Identity, ItemsWithoutCursor, Patch};

/// Files store documents, binary blobs, and other file data and relate it to assets.
pub type Files = Resource<FileMetadata>;

impl WithBasePath for Files {
    const BASE_PATH: &'static str = "files";
}

impl FilterWithRequest<PartitionedFilter<FileFilter>, FileMetadata> for Files {}
impl<'a> SearchItems<'a, FileFilter, FileSearch, FileMetadata> for Files {}
impl RetrieveWithIgnoreUnknownIds<Identity, FileMetadata> for Files {}
impl Delete<Identity> for Files {}
impl Update<Patch<PatchFile>, FileMetadata> for Files {}

impl Files {
    /// Upload a stream to an url, the url is received from `Files::upload`
    ///
    /// # Arguments
    ///
    /// * `mime_type` - Mime type of file to upload. For example `application/pdf`.
    /// * `url` - URL to upload stream to.
    /// * `stream` - Stream to upload.
    /// * `stream_chunked` - Set this to `true` to use chunked streaming. Note that this is not supported for the
    /// azure file backend. If this is set to `false`, the entire file is read into memory before uploading, which may
    /// be very expensive. Use `upload_stream_known_size` if the size of the file is known.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use tokio_util::codec::{BytesCodec, FramedRead};
    ///
    /// let file = tokio::fs::File::open("my-file");
    /// let stream = FramedRead::new(file, BytesCodec::new());
    /// cognite_client.files.upload_stream(&file.mime_type.unwrap(), &file.upload_url, stream, true).await?;
    /// ```
    ///
    /// Note that `stream_chunked` being true is in general more efficient, but it is not supported
    /// for the azure file backend.
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
            .put_stream(url, mime_type, stream, stream_chunked, None)
            .await
    }

    /// Upload a stream to an url, the url is received from `Files::upload`
    /// This method requires that the length of the stream in bytes is known before hand.
    /// If the specified size is wrong, the request may fail or even hang.
    ///
    /// # Arguments
    ///
    /// * `mime_type` - Mime type of file to upload. For example `application/pdf`.
    /// * `url` - URL to upload stream to.
    /// * `stream` - Stream to upload.
    /// * `size` - Known size of stream in bytes. Note: Do not use this method if the size is not
    /// actually known!
    ///
    /// # Example
    ///
    /// ```ignore
    /// use tokio_util::codec::{BytesCodec, FramedRead};
    ///
    /// let size = tokio::fs::metadata("my-file").await?.len();
    /// let file = tokio::fs::File::open("my-file").await?;
    /// let stream = FramedRead::new(file, BytesCodec::new());
    ///
    /// cognite_client.files.upload_stream_known_size(&file.mime_type.unwrap(), &file.upload_url, stream, size).await?;
    /// ```
    ///
    /// Note that this will still stream the data from disk, so it should be as efficient as `upload_stream` with
    /// `upload_chunked`, but not require the target to accept `content-encoding: chunked`.
    pub async fn upload_stream_known_size<S>(
        &self,
        mime_type: &str,
        url: &str,
        stream: S,
        size: u64,
    ) -> Result<()>
    where
        S: futures::TryStream + Send + Sync + 'static,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
        bytes::Bytes: From<S::Ok>,
    {
        self.api_client
            .put_stream(url, mime_type, stream, true, Some(size))
            .await
    }

    /// Upload a binary vector to `url`.
    ///
    /// # Arguments
    ///
    /// * `mime_type` - Mime type of file to upload. For example `application/pdf`.
    /// * `url` - URL to upload blob to.
    /// * `blob` - File to upload, as bytes.
    pub async fn upload_blob(&self, mime_type: &str, url: &str, blob: Vec<u8>) -> Result<()> {
        self.api_client.put_blob(url, mime_type, blob).await
    }

    /// Create a file, optionally overwriting an existing file.
    ///
    /// The result will contain an upload URL that can be used to upload a file.
    ///
    /// # Arguments
    ///
    /// * `overwrite` - Set this to `true` to overwrite existing files with the same `external_id`.
    /// If this is `false`, and a file with the given `external_id` already exists, the request will fail.
    /// * `item` - The file to upload.
    pub async fn upload(&self, overwrite: bool, item: &AddFile) -> Result<FileMetadata> {
        self.api_client
            .post_with_query("files", item, Some(FileUploadQuery::new(overwrite)))
            .await
    }

    /// Get download links for a list of files.
    ///
    /// # Arguments
    ///
    /// * `ids` - List of file IDs or external IDs.
    pub async fn download_link(&self, ids: &[Identity]) -> Result<Vec<FileDownloadUrl>> {
        let items = Items::from(ids);
        let file_links_response: ItemsWithoutCursor<FileDownloadUrl> =
            self.api_client.post("files/downloadlink", &items).await?;
        Ok(file_links_response.items)
    }

    /// Stream a file from `url`.
    ///
    /// # Arguments
    ///
    /// * `url` - URL to download from.
    pub async fn download(
        &self,
        url: &str,
    ) -> Result<impl TryStream<Ok = bytes::Bytes, Error = reqwest::Error>> {
        self.api_client.get_stream(url).await
    }

    /// Stream a file given by `id`.
    ///
    /// # Arguments
    ///
    /// * `id` - ID or external ID of file to download.
    pub async fn download_file(
        &self,
        id: Identity,
    ) -> Result<impl TryStream<Ok = bytes::Bytes, Error = reqwest::Error>> {
        let items = vec![id];
        let links = self.download_link(&items).await?;
        let link = links.first().unwrap();
        self.download(&link.download_url).await
    }
}
