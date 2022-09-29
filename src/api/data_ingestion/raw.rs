use crate::api::resource::Resource;
use crate::dto::items::Items;
use crate::error::Result;
use crate::raw::*;
use crate::{ItemsWithCursor, LimitCursorQuery};

pub type Raw = Resource<RawRow>;

impl Raw {
    pub async fn list_databases(
        &self,
        limit: Option<i32>,
        cursor: Option<String>,
    ) -> Result<ItemsWithCursor<Database>> {
        let query = LimitCursorQuery { limit, cursor };
        self.api_client
            .get_with_params("raw/dbs", Some(query))
            .await
    }

    pub async fn create_databases(&self, dbs: &[Database]) -> Result<Vec<Database>> {
        let items = Items::from(dbs);
        let result: ItemsWithCursor<Database> = self.api_client.post("raw/dbs", &items).await?;
        Ok(result.items)
    }

    pub async fn delete_databases(&self, to_delete: &DeleteDatabasesRequest) -> Result<()> {
        self.api_client
            .post::<::serde_json::Value, DeleteDatabasesRequest>("raw/dbs/delete", to_delete)
            .await?;
        Ok(())
    }

    pub async fn list_tables(
        &self,
        db_name: &str,
        limit: Option<i32>,
        cursor: Option<String>,
    ) -> Result<ItemsWithCursor<Table>> {
        let query = LimitCursorQuery { limit, cursor };
        let path = format!("raw/dbs/{}/tables", db_name);
        self.api_client.get_with_params(&path, Some(query)).await
    }

    pub async fn create_tables(
        &self,
        db_name: &str,
        ensure_parent: Option<bool>,
        tables: &[Table],
    ) -> Result<Vec<Table>> {
        let query = EnsureParentQuery { ensure_parent };
        let path = format!("raw/dbs/{}/tables", db_name);
        let items = Items::from(tables);
        let result: ItemsWithCursor<Table> = self
            .api_client
            .post_with_query(&path, &items, Some(query))
            .await?;
        Ok(result.items)
    }

    pub async fn delete_tables(&self, db_name: &str, to_delete: &[Table]) -> Result<()> {
        let path = format!("raw/dbs/{}/tables/delete", db_name);
        let items = Items::from(to_delete);
        self.api_client
            .post::<::serde_json::Value, Items>(&path, &items)
            .await?;
        Ok(())
    }

    pub async fn retrieve_cursors_for_parallel_reads(
        &self,
        db_name: &str,
        table_name: &str,
        params: Option<RetrieveCursorsQuery>,
    ) -> Result<Vec<String>> {
        let path = format!("raw/dbs/{}/tables/{}/cursors", db_name, table_name);
        let result: ItemsWithCursor<String> =
            self.api_client.get_with_params(&path, params).await?;
        Ok(result.items)
    }

    pub async fn retrieve_rows(
        &self,
        db_name: &str,
        table_name: &str,
        params: Option<RetrieveRowsQuery>,
    ) -> Result<ItemsWithCursor<RawRow>> {
        let path = format!("raw/dbs/{}/tables/{}/rows", db_name, table_name);
        self.api_client.get_with_params(&path, params).await
    }

    pub async fn insert_rows(
        &self,
        db_name: &str,
        table_name: &str,
        ensure_parent: Option<bool>,
        rows: &[RawRowCreate],
    ) -> Result<()> {
        let path = format!("raw/dbs/{}/tables/{}/rows", db_name, table_name);
        let query = EnsureParentQuery { ensure_parent };
        let items = Items::from(rows);
        self.api_client
            .post_with_query::<::serde_json::Value, Items, EnsureParentQuery>(
                &path,
                &items,
                Some(query),
            )
            .await?;
        Ok(())
    }

    pub async fn retrieve_row(&self, db_name: &str, table_name: &str, key: &str) -> Result<RawRow> {
        let path = format!("raw/dbs/{}/tables/{}/rows/{}", db_name, table_name, key);
        self.api_client.get(&path).await
    }

    pub async fn delete_rows(
        &self,
        db_name: &str,
        table_name: &str,
        to_delete: &[DeleteRow],
    ) -> Result<()> {
        let path = format!("raw/dbs/{}/tables/{}/rows/delete", db_name, table_name);
        let items = Items::from(to_delete);
        self.api_client
            .post::<::serde_json::Value, Items>(&path, &items)
            .await?;
        Ok(())
    }
}
