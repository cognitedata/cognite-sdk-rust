use crate::api::ApiClient;
use crate::dto::core::files::*;
use crate::dto::items::Items;
use crate::error::Result;

pub struct Files {
    api_client: ApiClient,
}

impl Files {
    pub fn new(api_client: ApiClient) -> Files {
        Files { api_client }
    }

    pub async fn filter_all(&self, file_filter: FileFilter) -> Result<Vec<FileMetadata>> {
        let filter: Filter = Filter::new(file_filter, None, None);
        let files_response: FileListResponse = self.api_client.post("files/list", &filter).await?;
        Ok(files_response.items)
    }

    pub async fn upload(&self, _file_stream: Vec<u8>) -> Result<FileMetadata> {
        unimplemented!();
    }

    pub async fn retrieve_metadata(&self, file_ids: &[u64]) -> Result<Vec<FileMetadata>> {
        let id_list: Vec<FileId> = file_ids.iter().copied().map(FileId::from).collect();
        let id_items = Items::from(&id_list);
        let files_response: FileListResponse =
            self.api_client.post("files/byids", &id_items).await?;
        Ok(files_response.items)
    }

    pub async fn search(
        &self,
        file_filter: FileFilter,
        file_search: FileSearch,
    ) -> Result<Vec<FileMetadata>> {
        let filter: Search = Search::new(file_filter, file_search, None);
        let files_response: FileListResponse =
            self.api_client.post("files/search", &filter).await?;
        Ok(files_response.items)
    }

    pub async fn delete(&self, file_ids: Vec<u64>) -> Result<()> {
        let id_list: Vec<FileId> = file_ids.iter().copied().map(FileId::from).collect();
        let id_items = Items::from(&id_list);
        self.api_client
            .post::<::serde_json::Value, Items>("files/delete", &id_items)
            .await?;
        Ok(())
    }

    pub async fn download_link(&self, file_ids: Vec<u64>) -> Result<Vec<FileLink>> {
        let id_list: Vec<FileId> = file_ids.iter().copied().map(FileId::from).collect();
        let id_items = Items::from(&id_list);
        let file_links_response: FileLinkListResponse =
            self.api_client.post("files/download", &id_items).await?;
        Ok(file_links_response.items)
    }

    pub async fn update(&self, _file_ids: Vec<u64>) -> Result<()> {
        unimplemented!();
    }
}
