use crate::{
    models::records::{ListStreamParams, Stream, StreamWrite},
    Create, List, Resource, WithBasePath,
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
    pub async fn retrieve(&self, stream_id: &str) -> crate::error::Result<Stream> {
        self.api_client
            .get(&format!("{}/{}", Self::BASE_PATH, stream_id))
            .await
    }

    /// Delete a stream by its ID.
    ///
    /// # Arguments
    ///
    /// * `stream_id` - ID of the stream to delete.
    pub async fn delete(&self, stream_id: &str) -> crate::error::Result<()> {
        self.api_client
            .delete::<serde_json::Value>(&format!("{}/{}", Self::BASE_PATH, stream_id))
            .await?;
        Ok(())
    }
}
