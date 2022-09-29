use crate::api::resource::*;
use crate::dto::core::sequences::*;
use crate::error::Result;
use crate::{Identity, Items, LimitCursorPartitionQuery, PartitionedFilter, Patch};

pub type Sequences = Resource<Sequence>;

impl WithBasePath for Sequences {
    const BASE_PATH: &'static str = "sequences";
}

impl List<LimitCursorPartitionQuery, Sequence> for Sequences {}
impl Create<AddSequence, Sequence> for Sequences {}
impl<'a> SearchItems<'a, SequenceFilter, SequenceSearch, Sequence> for Sequences {}
impl Update<Patch<PatchSequence>, Sequence> for Sequences {}
impl DeleteWithIgnoreUnknownIds<Identity> for Sequences {}
impl RetrieveWithIgnoreUnknownIds<Identity, Sequence> for Sequences {}
impl FilterWithRequest<PartitionedFilter<SequenceFilter>, Sequence> for Sequences {}

impl Sequences {
    pub async fn insert_rows(&self, rows: &[InsertSequenceRows]) -> Result<()> {
        let items = Items::from(rows);
        self.api_client
            .post(&format!("{}/{}", Self::BASE_PATH, "data"), &items)
            .await?;
        Ok(())
    }

    pub async fn retrieve_rows(
        &self,
        query: RetrieveSequenceRows,
    ) -> Result<RetrieveSequenceRowsResponse> {
        Ok(self
            .api_client
            .post(&format!("{}/{}", Self::BASE_PATH, "data/list"), &query)
            .await?)
    }

    pub async fn retrieve_latest_row(
        &self,
        query: RetrieveLatestSequenceRow,
    ) -> Result<RetrieveSequenceRowsResponse> {
        Ok(self
            .api_client
            .post(&format!("{}/{}", Self::BASE_PATH, "data/latest"), &query)
            .await?)
    }

    pub async fn delete_rows(&self, query: &[DeleteSequenceRows]) -> Result<()> {
        let items = Items::from(query);
        Ok(self
            .api_client
            .post(&format!("{}/{}", Self::BASE_PATH, "data/delete"), &items)
            .await?)
    }
}
