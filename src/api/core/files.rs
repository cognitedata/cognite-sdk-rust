use crate::api::resource::*;
use crate::dto::core::files::*;
use crate::dto::items::Items;
use crate::error::Result;
use crate::{CogniteId, Identity};

pub type Files = Resource<FileMetadata>;

impl WithBasePath for Files {
    const BASE_PATH: &'static str = "files";
}

impl FilterItems<FileFilter, FileMetadata> for Files {}
impl<'a> SearchItems<'a, FileFilter, FileSearch, FileMetadata> for Files {}
impl RetrieveWithIgnoreUnknownIds<Identity, FileMetadata> for Files {}
impl Delete<Identity> for Files {}

impl Files {
    pub async fn upload(&self, _file_stream: Vec<u8>) -> Result<FileMetadata> {
        unimplemented!();
    }

    pub async fn download_link(&self, file_ids: Vec<i64>) -> Result<Vec<FileLink>> {
        let id_list: Vec<CogniteId> = file_ids.iter().copied().map(CogniteId::from).collect();
        let id_items = Items::from(&id_list);
        let file_links_response: FileLinkListResponse =
            self.api_client.post("files/download", &id_items).await?;
        Ok(file_links_response.items)
    }

    pub async fn update(&self, _file_ids: Vec<u64>) -> Result<()> {
        unimplemented!();
    }
}
