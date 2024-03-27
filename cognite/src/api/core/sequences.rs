use crate::api::resource::*;
use crate::dto::core::sequences::*;
use crate::error::Result;
use crate::{Identity, Items, LimitCursorPartitionQuery, Patch};

/// A sequence stores a table with up to 400 columns indexed by row number. There can be at most
/// 400 numeric columns and 200 string columns. Each of the columns has a pre-defined type:
/// a string, integer, or floating point number.
pub type SequencesResource = Resource<Sequence>;

impl WithBasePath for SequencesResource {
    const BASE_PATH: &'static str = "sequences";
}

impl List<LimitCursorPartitionQuery, Sequence> for SequencesResource {}
impl Create<AddSequence, Sequence> for SequencesResource {}
impl<'a> SearchItems<'a, SequenceFilter, SequenceSearch, Sequence> for SequencesResource {}
impl Update<Patch<PatchSequence>, Sequence> for SequencesResource {}
impl DeleteWithIgnoreUnknownIds<Identity> for SequencesResource {}
impl RetrieveWithIgnoreUnknownIds<Identity, Sequence> for SequencesResource {}
impl FilterWithRequest<SequenceFilterRequest, Sequence> for SequencesResource {}

impl SequencesResource {
    /// Insert a list of rows into a set of sequences.
    ///
    /// # Arguments
    ///
    /// * `rows` - Sequence row batches to insert.
    pub async fn insert_rows(&self, rows: &[InsertSequenceRows]) -> Result<()> {
        let items = Items::new(rows);
        self.api_client
            .post(&format!("{}/{}", Self::BASE_PATH, "data"), &items)
            .await?;
        Ok(())
    }

    /// Retrieve a rows from a set of sequences.
    ///
    /// # Arguments
    ///
    /// * `query` - Sequence rows retrieval query.
    pub async fn retrieve_rows(
        &self,
        query: RetrieveSequenceRows,
    ) -> Result<RetrieveSequenceRowsResponse> {
        self.api_client
            .post(&format!("{}/{}", Self::BASE_PATH, "data/list"), &query)
            .await
    }

    /// Retrieve the last row from a sequence. The last row is the one with the highest row number,
    /// not necessarily the one that was ingested the most recently.
    ///
    /// # Arguments
    ///
    /// * `query` - Sequence row retrieval query.
    pub async fn retrieve_last_row(
        &self,
        query: RetrieveLastSequenceRow,
    ) -> Result<RetrieveSequenceRowsResponse> {
        self.api_client
            .post(&format!("{}/{}", Self::BASE_PATH, "data/latest"), &query)
            .await
    }

    /// Delete rows from a set of sequences.
    ///
    /// # Arguments
    ///
    /// * `query` - Row ranges to delete.
    pub async fn delete_rows(&self, query: &[DeleteSequenceRows]) -> Result<()> {
        let items = Items::new(query);
        self.api_client
            .post(&format!("{}/{}", Self::BASE_PATH, "data/delete"), &items)
            .await
    }
}
