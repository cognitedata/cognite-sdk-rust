use bytes::Bytes;
use futures::TryStream;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::api::resource::*;
use crate::dto::core::files::*;
use crate::dto::items::Items;
use crate::error::Result;
use crate::{Error, IdentityOrInstance, PartitionedFilter};
use crate::{Identity, ItemsVec, Patch};

/// Files store documents, binary blobs, and other file data and relate it to assets.
pub type Files = Resource<FileMetadata>;

impl WithBasePath for Files {
    const BASE_PATH: &'static str = "files";
}

impl FilterWithRequest<PartitionedFilter<FileFilter>, FileMetadata> for Files {}
impl SearchItems<'_, FileFilter, FileSearch, FileMetadata> for Files {}
impl RetrieveWithIgnoreUnknownIds<Identity, FileMetadata> for Files {}
impl RetrieveWithIgnoreUnknownIds<IdentityOrInstance, FileMetadata> for Files {}
impl Delete<Identity> for Files {}
impl Update<Patch<PatchFile>, FileMetadata> for Files {}

/// Utility for uploading files in multiple parts.
pub struct MultipartUploader<'a> {
    resource: &'a Files,
    id: IdentityOrInstance,
    urls: MultiUploadUrls,
}

impl<'a> MultipartUploader<'a> {
    /// Create a new multipart uploader.
    ///
    /// # Arguments
    ///
    /// * `resource` - Files resource.
    /// * `id` - ID of the file to upload.
    /// * `urls` - Upload URLs returned from `init_multipart_upload`.
    pub fn new(resource: &'a Files, id: IdentityOrInstance, urls: MultiUploadUrls) -> Self {
        Self { resource, id, urls }
    }

    /// Upload a part given by part index given by `part_no`. The part number
    /// counts from zero, so with 5 parts you must upload with `part_no` 0, 1, 2, 3, and 4.
    ///
    /// # Arguments
    ///
    /// * `part_no` - Part number.
    /// * `stream` - Stream to upload.
    /// * `size` - Size of stream to upload.
    pub async fn upload_part_stream<S>(&self, part_no: usize, stream: S, size: u64) -> Result<()>
    where
        S: futures::TryStream + Send + 'static,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
        bytes::Bytes: From<S::Ok>,
    {
        if part_no >= self.urls.upload_urls.len() {
            return Err(Error::Other("Part number out of range".to_owned()));
        }

        self.resource
            .upload_stream_known_size("", &self.urls.upload_urls[0], stream, size)
            .await
    }

    /// Upload a part given by part index given by `part_no`. The part number
    /// counts from zero, so with 5 parts you must upload with `part_no` 0, 1, 2, 3, and 4.
    ///
    /// # Arguments
    ///
    /// * `part_no` - Part number.
    /// * `file` - File to upload.
    pub async fn upload_part_file<S>(&self, part_no: usize, file: tokio::fs::File) -> Result<()> {
        let size = file.metadata().await?.len();
        let stream = FramedRead::new(file, BytesCodec::new());

        self.upload_part_stream(part_no, stream, size).await
    }

    /// Upload a part given by part index given by `part_no`. The part number
    /// counts from zero, so with 5 parts you must upload with `part_no` 0, 1, 2, 3, and 4.
    ///
    /// # Arguments
    ///
    /// * `part_no` - Part number.
    /// * `part` - Binary data to upload.
    pub async fn upload_part_blob(&self, part_no: usize, part: impl Into<Bytes>) -> Result<()> {
        if part_no >= self.urls.upload_urls.len() {
            return Err(Error::Other("Part number out of range".to_owned()));
        }
        self.resource
            .upload_blob("", &self.urls.upload_urls[part_no], part)
            .await
    }

    /// Complete the multipart upload process after all parts are uploaded.
    pub async fn complete(self) -> Result<()> {
        self.resource
            .complete_multipart_upload(self.id, self.urls.upload_id)
            .await
    }
}

impl Files {
    /// Upload a stream to a url, the url is received from `Files::upload`
    ///
    /// # Arguments
    ///
    /// * `mime_type` - Mime type of file to upload. For example `application/pdf`.
    /// * `url` - URL to upload stream to.
    /// * `stream` - Stream to upload.
    /// * `stream_chunked` - Set this to `true` to use chunked streaming. Note that this is not supported for the
    ///   azure file backend. If this is set to `false`, the entire file is read into memory before uploading, which may
    ///   be very expensive. Use `upload_stream_known_size` if the size of the file is known.
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
        S: futures::TryStream + Send + 'static,
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
    ///   actually known!
    ///
    /// # Example
    ///
    /// ```ignore
    /// use tokio_util::codec::{BytesCodec, FramedRead};
    ///
    /// let file = tokio::fs::File::open("my-file").await?;
    /// let size = file.metadata().await?.len();
    /// let stream = FramedRead::new(file, BytesCodec::new());
    ///
    /// cognite_client.files.upload_stream_known_size(&file_res.mime_type.unwrap(), &file_res.extra.upload_url, stream, size).await?;
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
        S: futures::TryStream + Send + 'static,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
        bytes::Bytes: From<S::Ok>,
    {
        self.api_client
            .put_stream(url, mime_type, stream, true, Some(size))
            .await
    }

    /// Upload a file as a stream to CDF. `url` should be the upload URL returned from
    /// `upload`.
    ///
    /// # Arguments
    ///
    /// * `mime_type` - Mime type of file to upload. For example `application/pdf`.
    /// * `url` - URL to upload the file to.
    /// * `file` - File to upload.
    pub async fn upload_file(
        &self,
        mime_type: &str,
        url: &str,
        file: tokio::fs::File,
    ) -> Result<()> {
        let size = file.metadata().await?.len();
        let stream = FramedRead::new(file, BytesCodec::new());

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
    pub async fn upload_blob(
        &self,
        mime_type: &str,
        url: &str,
        blob: impl Into<Bytes>,
    ) -> Result<()> {
        self.api_client.put_blob(url, mime_type, blob).await
    }

    /// Create a file, optionally overwriting an existing file.
    ///
    /// The result will contain an upload URL that can be used to upload a file.
    ///
    /// # Arguments
    ///
    /// * `overwrite` - Set this to `true` to overwrite existing files with the same `external_id`.
    ///   If this is `false`, and a file with the given `external_id` already exists, the request will fail.
    /// * `item` - The file to upload.
    pub async fn upload(
        &self,
        overwrite: bool,
        item: &AddFile,
    ) -> Result<FileUploadResult<UploadUrl>> {
        self.api_client
            .post_with_query("files", item, Some(FileUploadQuery::new(overwrite)))
            .await
    }

    /// Get an upload link for a file with given identity.
    ///
    /// # Arguments
    ///
    /// `id` - Identity of file metadata or data models file.
    pub async fn get_upload_link(
        &self,
        id: &IdentityOrInstance,
    ) -> Result<FileUploadResult<UploadUrl>> {
        let mut res = self
            .api_client
            .post::<Items<Vec<FileUploadResult<UploadUrl>>>, _>(
                "files/uploadlink",
                &Items::new([id]),
            )
            .await?;
        if res.items.is_empty() {
            Err(Error::Other(
                "File with given identity not found.".to_string(),
            ))
        } else {
            Ok(res.items.remove(0))
        }
    }

    /// Get multipart upload link for an existing file metadata or data models file.
    ///
    /// # Arguments
    ///
    /// * `id` - Identity of file metadata or data models file.
    /// * `parts` - Number of parts to be uploaded.
    pub async fn get_multipart_upload_link(
        &self,
        id: &IdentityOrInstance,
        parts: u32,
    ) -> Result<FileUploadResult<MultiUploadUrls>> {
        let mut res = self
            .api_client
            .post_with_query::<Items<Vec<FileUploadResult<MultiUploadUrls>>>, _, _>(
                "files/multiuploadlink",
                &Items::new([id]),
                Some(MultipartGetUploadLinkQuery::new(parts)),
            )
            .await?;
        if res.items.is_empty() {
            Err(Error::Other(
                "File with given identity not found.".to_string(),
            ))
        } else {
            Ok(res.items.remove(0))
        }
    }

    /// Create a file, specifying that it should be uploaded in multiple parts.
    ///
    /// This returns a `MultipartUploader`, which wraps the upload process.
    ///
    /// # Arguments
    ///
    /// * `overwrite` - Set this to `true` to overwrite existing files with the same `external_id`.
    ///   If this is `false`, and a file with the given `external_id` already exists, the request will fail.
    /// * `parts` - The number of parts to upload, should be a number between 1 and 250.
    /// * `item` - The file to upload.
    pub async fn multipart_upload<'a>(
        &'a self,
        overwrite: bool,
        parts: u32,
        item: &AddFile,
    ) -> Result<(MultipartUploader<'a>, FileMetadata)> {
        let res = self.init_multipart_upload(overwrite, parts, item).await?;
        self.create_multipart_upload(res)
    }

    /// Upload files for an existing file metadata or data models file.
    ///
    /// This returns a `MultipartUploader`, which wraps the upload process.
    ///
    /// # Arguments
    ///
    /// * `parts` - The number of parts to upload, should be a number between 1 and 250.
    /// * `id` - Identity of file metadata or data models file.
    pub async fn multipart_upload_existing<'a>(
        &'a self,
        id: &IdentityOrInstance,
        parts: u32,
    ) -> Result<(MultipartUploader<'a>, FileMetadata)> {
        let res = self.get_multipart_upload_link(id, parts).await?;
        self.create_multipart_upload(res)
    }

    fn create_multipart_upload(
        &self,
        res: FileUploadResult<MultiUploadUrls>,
    ) -> Result<(MultipartUploader, FileMetadata)> {
        Ok((
            MultipartUploader::new(
                self,
                IdentityOrInstance::Identity(Identity::Id {
                    id: res.metadata.id,
                }),
                res.extra,
            ),
            res.metadata,
        ))
    }

    /// Create a file, specifying that it should be uploaded in multiple parts.
    ///
    /// # Arguments
    ///
    /// * `overwrite` - Set this to `true` to overwrite existing files with the same `external_id`.
    ///   If this is `false`, and a file with the given `external_id` already exists, the request will fail.
    /// * `parts` - The number of parts to upload, should be a number between 1 and 250.
    /// * `item` - The file to upload.
    pub async fn init_multipart_upload(
        &self,
        overwrite: bool,
        parts: u32,
        item: &AddFile,
    ) -> Result<FileUploadResult<MultiUploadUrls>> {
        self.api_client
            .post_with_query(
                "files/initmultipartupload",
                item,
                Some(MultipartFileUploadQuery::new(overwrite, parts)),
            )
            .await
    }

    /// Complete a multipart upload. This endpoint must be called after all parts of a multipart file
    /// upload have been uploaded.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the file that was uploaded.
    /// * `upload_id` - `upload_id` returned by `init_multipart_upload`.
    pub async fn complete_multipart_upload(
        &self,
        id: IdentityOrInstance,
        upload_id: String,
    ) -> Result<()> {
        self.api_client
            .post::<serde_json::Value, _>(
                "files/completemultipartupload",
                &CompleteMultipartUpload { id, upload_id },
            )
            .await?;
        Ok(())
    }

    /// Get download links for a list of files.
    ///
    /// # Arguments
    ///
    /// * `ids` - List of file IDs or external IDs.
    pub async fn download_link(&self, ids: &[IdentityOrInstance]) -> Result<Vec<FileDownloadUrl>> {
        let items = Items::new(ids);
        let file_links_response: ItemsVec<FileDownloadUrl> =
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
        id: IdentityOrInstance,
    ) -> Result<impl TryStream<Ok = bytes::Bytes, Error = reqwest::Error>> {
        let items = vec![id];
        let links = self.download_link(&items).await?;
        let link = links.first().unwrap();
        self.download(&link.download_url).await
    }
}
