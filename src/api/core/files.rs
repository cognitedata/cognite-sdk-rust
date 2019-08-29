use crate::api::ApiClient;
use crate::dto::core::files::*;
use crate::dto::items::Items;
use crate::error::Result;

pub struct Files {
    api_client: ApiClient,
}

impl Files {
    pub fn new(api_client: ApiClient) -> Files {
        Files {
            api_client: api_client,
        }
    }

    pub fn filter_all(&self, file_filter: FileFilter) -> Result<Vec<FileMetadata>> {
        let filter: Filter = Filter::new(file_filter, None, None);
        let files_response: FileListResponse = self.api_client.post("files/list", &filter)?;
        Ok(files_response.items)
    }

    pub fn upload(&self, _file_stream: Vec<u8>) -> Result<FileMetadata> {
        unimplemented!();
    }

    pub fn retrieve_metadata(&self, file_ids: &[u64]) -> Result<Vec<FileMetadata>> {
        let id_list: Vec<FileId> = file_ids.iter().map(|f_id| FileId::from(*f_id)).collect();
        let id_items = Items::from(&id_list);
        let files_response: FileListResponse = self.api_client.post("files/byids", &id_items)?;
        Ok(files_response.items)
    }

    pub fn search(
        &self,
        file_filter: FileFilter,
        file_search: FileSearch,
    ) -> Result<Vec<FileMetadata>> {
        let filter: Search = Search::new(file_filter, file_search, None);
        let files_response: FileListResponse = self.api_client.post("files/search", &filter)?;
        Ok(files_response.items)
    }

    pub fn delete(&self, file_ids: Vec<u64>) -> Result<()> {
        let id_list: Vec<FileId> = file_ids.iter().map(|a_id| FileId::from(*a_id)).collect();
        let id_items = Items::from(&id_list);
        self.api_client
            .post::<::serde_json::Value, Items>("files/delete", &id_items)?;
        Ok(())
    }

    pub fn download_link(&self, file_ids: Vec<u64>) -> Result<Vec<FileLink>> {
        let id_list: Vec<FileId> = file_ids.iter().map(|a_id| FileId::from(*a_id)).collect();
        let id_items = Items::from(&id_list);
        let file_links_response: FileLinkListResponse =
            self.api_client.post("files/download", &id_items)?;
        Ok(file_links_response.items)
    }

    pub fn update(&self, _file_ids: Vec<u64>) -> Result<()> {
        unimplemented!();
    }
}
