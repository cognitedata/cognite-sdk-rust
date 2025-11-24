use crate::{
    models::records::{ListStreamParams, RetrieveStreamParams, Stream, StreamWrite},
    CogniteExternalId, Create, Items, List, Resource, WithBasePath,
};

pub type StreamsResource = Resource<Stream>;

impl WithBasePath for StreamsResource {
    const BASE_PATH: &'static str = "streams";
}

impl Create<StreamWrite, Stream> for StreamsResource {}
impl List<ListStreamParams, Stream> for StreamsResource {}

impl StreamsResource {
    /// Retrieve a stream by its ID.
    ///
    /// # Arguments
    ///
    /// * `stream_id` - ID of the stream to retrieve.
    pub async fn retrieve(
        &self,
        stream_id: &str,
        include_statistics: bool,
    ) -> crate::error::Result<Stream> {
        self.api_client
            .get_with_params(
                &format!("{}/{}", Self::BASE_PATH, stream_id),
                Some(RetrieveStreamParams {
                    include_statistics: Some(include_statistics),
                }),
            )
            .await
    }

    /// Delete a stream by its ID.
    ///
    /// # Arguments
    ///
    /// * `stream_ids` - IDs of the streams to delete.
    pub async fn delete(&self, stream_ids: &[&str]) -> crate::error::Result<()> {
        self.api_client
            .post::<serde_json::Value, _>(
                &format!("{}/delete", Self::BASE_PATH),
                &Items::new(
                    stream_ids
                        .iter()
                        .map(|id| CogniteExternalId {
                            external_id: id.to_string(),
                        })
                        .collect::<Vec<_>>(),
                ),
            )
            .await?;
        Ok(())
    }
}
