use std::collections::VecDeque;

use futures::stream::{try_unfold, SelectAll};
use futures::{FutureExt, StreamExt, TryStream, TryStreamExt};

use crate::api::resource::Resource;
use crate::dto::items::Items;
use crate::error::Result;
use crate::{raw::*, CursorState, CursorStreamState};
use crate::{Cursor, ItemsVec, LimitCursorQuery};

/// Raw is a NoSQL JSON store. Each project can have a variable number of databases,
/// each of which will have a variable number of tables, each of which will have a variable
/// number of key-value objects. Only queries on key are supported through this API.
pub type RawResource = Resource<RawRow>;

impl RawResource {
    /// List Raw databases in the project.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of databases to retrieve.
    /// * `cursor` - Optional cursor for pagination.
    pub async fn list_databases(
        &self,
        limit: Option<i32>,
        cursor: Option<String>,
    ) -> Result<ItemsVec<Database, Cursor>> {
        let query = LimitCursorQuery { limit, cursor };
        self.api_client
            .get_with_params("raw/dbs", Some(query))
            .await
    }

    /// Create a list of Raw databases.
    ///
    /// # Arguments
    ///
    /// * `dbs` - Databases to create.
    pub async fn create_databases(&self, dbs: &[Database]) -> Result<Vec<Database>> {
        let items = Items::new(dbs);
        let result: ItemsVec<Database, Cursor> = self.api_client.post("raw/dbs", &items).await?;
        Ok(result.items)
    }

    /// Delete a list of raw databases.
    ///
    /// # Arguments
    ///
    /// * `to_delete` - Request describing which databases to delete and how.
    pub async fn delete_databases(&self, to_delete: &DeleteDatabasesRequest) -> Result<()> {
        self.api_client
            .post::<::serde_json::Value, DeleteDatabasesRequest>("raw/dbs/delete", to_delete)
            .await?;
        Ok(())
    }

    /// List tables in a a raw database.
    ///
    /// # Arguments
    ///
    /// * `db_name` - Database to list tables in.
    /// * `limit` - Maximum number of tables to retrieve.
    /// * `cursor` - Optional cursor for pagination.
    pub async fn list_tables(
        &self,
        db_name: &str,
        limit: Option<i32>,
        cursor: Option<String>,
    ) -> Result<ItemsVec<Table, Cursor>> {
        let query = LimitCursorQuery { limit, cursor };
        let path = format!("raw/dbs/{db_name}/tables");
        self.api_client.get_with_params(&path, Some(query)).await
    }

    /// Create tables in a raw database.
    ///
    /// # Arguments
    ///
    /// * `db_name` - Database to create tables in.
    /// * `ensure_parent` - If this is set to `true`, create database if it doesn't already exist.
    /// * `tables` - Tables to create.
    pub async fn create_tables(
        &self,
        db_name: &str,
        ensure_parent: bool,
        tables: &[Table],
    ) -> Result<Vec<Table>> {
        let query = EnsureParentQuery {
            ensure_parent: Some(ensure_parent),
        };
        let path = format!("raw/dbs/{db_name}/tables");
        let items = Items::new(tables);
        let result: ItemsVec<Table, Cursor> = self
            .api_client
            .post_with_query(&path, &items, Some(query))
            .await?;
        Ok(result.items)
    }

    /// Delete tables in a raw database.
    ///
    /// # Arguments
    ///
    /// * `db_name` - Database to delete tables from.
    /// * `to_delete` - Tables to delete.
    pub async fn delete_tables(&self, db_name: &str, to_delete: &[Table]) -> Result<()> {
        let path = format!("raw/dbs/{db_name}/tables/delete");
        let items = Items::new(to_delete);
        self.api_client
            .post::<::serde_json::Value, _>(&path, &items)
            .await?;
        Ok(())
    }

    /// Retrieve cursors for parallel reads. This can be used to efficiently download
    /// large volumes of data from a raw table in parallel.
    ///
    /// # Arguments
    ///
    /// * `db_name` - Database to retrieve from.
    /// * `table_name` - Table to retrieve from.
    /// * `params` - Optional filter parameters.
    pub async fn retrieve_cursors_for_parallel_reads(
        &self,
        db_name: &str,
        table_name: &str,
        params: Option<RetrieveCursorsQuery>,
    ) -> Result<Vec<String>> {
        let path = format!("raw/dbs/{db_name}/tables/{table_name}/cursors");
        let result: ItemsVec<String, Cursor> =
            self.api_client.get_with_params(&path, params).await?;
        Ok(result.items)
    }

    /// Retrieve rows from a table, with some basic filtering options.
    ///
    /// # Arguments
    ///
    /// * `db_name` - Database to retrieve rows from.
    /// * `table_name` - Table to retrieve rows from.
    /// * `params` - Optional filter parameters.
    pub async fn retrieve_rows(
        &self,
        db_name: &str,
        table_name: &str,
        params: Option<RetrieveRowsQuery>,
    ) -> Result<ItemsVec<RawRow, Cursor>> {
        let path = format!("raw/dbs/{db_name}/tables/{table_name}/rows");
        self.api_client.get_with_params(&path, params).await
    }

    /// Retrieve all rows from a table, following cursors. This returns a stream, you can abort the stream whenever you
    /// want and only resources retrieved up to that point will be returned.
    ///
    /// Each item in the stream will be a result, after the first error is returned the
    /// stream will end.
    ///
    /// `limit` in the filter only affects how many rows are returned _per request_.
    ///
    /// # Arguments
    ///
    /// * `db_name` - Database to retrieve rows from.
    /// * `table_name` - Table to retrieve rows from.
    /// * `params` - Optional filter parameters. This can set a cursor to start streaming from there.
    pub fn retrieve_all_rows_stream<'a>(
        &'a self,
        db_name: &'a str,
        table_name: &'a str,
        params: Option<RetrieveRowsQuery>,
    ) -> impl TryStream<Ok = RawRow, Error = crate::Error, Item = Result<RawRow>> + 'a {
        let req = params.unwrap_or_default();
        let initial_state = match &req.cursor {
            Some(p) => CursorState::Some(p.to_owned()),
            None => CursorState::Initial,
        };
        let state = CursorStreamState {
            req,
            responses: VecDeque::new(),
            next_cursor: initial_state,
        };

        try_unfold(state, move |mut state| async move {
            if let Some(next) = state.responses.pop_front() {
                Ok(Some((next, state)))
            } else {
                let cursor = match std::mem::take(&mut state.next_cursor) {
                    CursorState::Initial => None,
                    CursorState::Some(x) => Some(x),
                    CursorState::End => {
                        return Ok(None);
                    }
                };
                state.req.cursor = cursor;
                let response = self
                    .retrieve_rows(db_name, table_name, Some(state.req.clone()))
                    .await?;

                state.responses.extend(response.items);
                state.next_cursor = match response.extra_fields.next_cursor {
                    Some(x) => CursorState::Some(x),
                    None => CursorState::End,
                };
                if let Some(next) = state.responses.pop_front() {
                    Ok(Some((next, state)))
                } else {
                    Ok(None)
                }
            }
        })
    }

    /// Retrieve all rows from a table, following cursors.
    ///
    /// `limit` in the filter only affects how many rows are returned _per request_.
    ///
    /// # Arguments
    ///
    /// * `db_name` - Database to retrieve rows from.
    /// * `table_name` - Table to retrieve rows from.
    /// * `params` - Optional filter parameters. This can set a cursor to start reading from there.
    pub async fn retrieve_all_rows(
        &self,
        db_name: &str,
        table_name: &str,
        params: Option<RetrieveRowsQuery>,
    ) -> Result<Vec<RawRow>> {
        self.retrieve_all_rows_stream(db_name, table_name, params)
            .try_collect()
            .await
    }

    /// Retrieve all rows from a table, following cursors and reading from multiple streams in parallel.
    ///
    /// The order of the returned values is not guaranteed to be in any way consistent.
    ///
    /// * `db_name` - Database to retrieve rows from.
    /// * `table_name` - Table to retrieve rows from.
    /// * `params` - Optional filter parameters.
    pub async fn retrieve_all_rows_partitioned(
        &self,
        db_name: &str,
        table_name: &str,
        params: RetrieveAllPartitionedQuery,
    ) -> Result<Vec<RawRow>> {
        self.retrieve_all_rows_partitioned_stream(db_name, table_name, params)
            .try_collect()
            .await
    }

    /// Retrieve all rows from a table, following cursors and reading from multiple streams in parallel.
    ///
    /// The order of the returned values is not guaranteed to be in any way consistent.
    ///
    /// * `db_name` - Database to retrieve rows from.
    /// * `table_name` - Table to retrieve rows from.
    /// * `params` - Optional filter parameters.
    pub fn retrieve_all_rows_partitioned_stream<'a>(
        &'a self,
        db_name: &'a str,
        table_name: &'a str,
        params: RetrieveAllPartitionedQuery,
    ) -> impl TryStream<Ok = RawRow, Error = crate::Error, Item = Result<RawRow>> + 'a {
        self.retrieve_cursors_for_parallel_reads(
            db_name,
            table_name,
            Some(RetrieveCursorsQuery {
                min_last_updated_time: params.min_last_updated_time,
                max_last_updated_time: params.max_last_updated_time,
                number_of_cursors: params.number_of_cursors,
            }),
        )
        .into_stream()
        .map_ok(move |cursors| {
            let mut streams = SelectAll::new();
            for cursor in cursors {
                let query = RetrieveRowsQuery {
                    limit: params.limit,
                    columns: params.columns.clone(),
                    cursor: Some(cursor),
                    min_last_updated_time: params.min_last_updated_time,
                    max_last_updated_time: params.max_last_updated_time,
                };
                streams.push(
                    self.retrieve_all_rows_stream(db_name, table_name, Some(query))
                        .boxed_local(),
                );
            }
            streams
        })
        .try_flatten()
    }

    /// Insert rows into a table.
    ///
    /// If `ensure_parent` is true, create the database and/or table if they do not exist.
    ///
    /// # Arguments
    ///
    /// * `db_name` - Database to insert rows into.
    /// * `table_name` - Table to insert rows into.
    /// * `ensure_parent` - Create database and/or table if they do not exist.
    /// * `rows` - Raw rows to create.
    pub async fn insert_rows(
        &self,
        db_name: &str,
        table_name: &str,
        ensure_parent: bool,
        rows: &[RawRowCreate],
    ) -> Result<()> {
        let path = format!("raw/dbs/{db_name}/tables/{table_name}/rows");
        let query = EnsureParentQuery {
            ensure_parent: Some(ensure_parent),
        };
        let items = Items::new(rows);
        self.api_client
            .post_with_query::<::serde_json::Value, _, EnsureParentQuery>(
                &path,
                &items,
                Some(query),
            )
            .await?;
        Ok(())
    }

    /// Retrieve a single row from a raw table.
    ///
    /// # Arguments
    ///
    /// * `db_name` - Database to retrieve from.
    /// * `table_name` - Table to retrieve from.
    /// * `key` - Key of row to retrieve.
    pub async fn retrieve_row(&self, db_name: &str, table_name: &str, key: &str) -> Result<RawRow> {
        let path = format!("raw/dbs/{db_name}/tables/{table_name}/rows/{key}");
        self.api_client.get(&path).await
    }

    /// Delete rows from a raw table.
    ///
    /// # Arguments
    ///
    /// * `db_name` - Database to delete from.
    /// * `table_name` - Table to delete from.
    /// * `to_delete` - Rows to delete.
    pub async fn delete_rows(
        &self,
        db_name: &str,
        table_name: &str,
        to_delete: &[DeleteRow],
    ) -> Result<()> {
        let path = format!("raw/dbs/{db_name}/tables/{table_name}/rows/delete");
        let items = Items::new(to_delete);
        self.api_client
            .post::<::serde_json::Value, _>(&path, &items)
            .await?;
        Ok(())
    }
}
