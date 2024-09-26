use crate::models::{
    data_models::{CogniteExtractorFile, FileProperties},
    views::ViewReference,
};

use super::{DataModelsResource, RetrieveExtendedCollection, UpsertExtendedCollection, WithView};

pub type FilesResource = DataModelsResource;

impl WithView for FilesResource {
    const SPACE: &'static str = "cdf_extraction_extensions";
    const EXTERNAL_ID: &'static str = "CogniteExtractorFile";
    const VERSION: &'static str = "v1";

    fn with_view(&mut self, space: String, external_id: String, version: String) {
        self.view = Some(ViewReference {
            space,
            external_id,
            version,
        });
    }

    fn view(&self) -> ViewReference {
        self.view.to_owned().unwrap_or(ViewReference {
            space: FilesResource::SPACE.to_string(),
            version: FilesResource::VERSION.to_string(),
            external_id: FilesResource::EXTERNAL_ID.to_string(),
        })
    }
}

impl RetrieveExtendedCollection<FileProperties, CogniteExtractorFile> for FilesResource {}

impl UpsertExtendedCollection<CogniteExtractorFile, FileProperties> for FilesResource {}
