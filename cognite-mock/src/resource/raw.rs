use std::sync::Arc;

use cognite::raw::{RawRow, RawRowCreate};
use cognite::{Cursor, ItemsVec};

use crate::client::MockClient;
use crate::error::{MockError, Result};

const DEFAULT_LIMIT: usize = 1000;

pub struct MockRawResource {
    pub(crate) client: Arc<MockClient>,
}

impl MockRawResource {
    pub fn new(client: Arc<MockClient>) -> Self {
        Self { client }
    }

    pub async fn create_db(&self, name: &str) -> Result<()> {
        self.client.raw.write().await.create_db(name);
        Ok(())
    }

    pub async fn delete_db(&self, name: &str, ignore_unknown: bool) -> Result<()> {
        let mut store = self.client.raw.write().await;
        if !store.db_exists(name) {
            if ignore_unknown {
                return Ok(());
            }
            return Err(MockError::NotFound(format!(
                "Database '{}' not found",
                name
            )));
        }
        store.delete_db(name);
        Ok(())
    }

    pub async fn list_dbs(&self) -> Result<Vec<String>> {
        Ok(self.client.raw.read().await.list_dbs())
    }

    pub async fn create_table(&self, db: &str, table: &str) -> Result<()> {
        let mut store = self.client.raw.write().await;
        if !store.db_exists(db) {
            return Err(MockError::NotFound(format!("Database '{}' not found", db)));
        }
        store.create_table(db, table);
        Ok(())
    }

    pub async fn delete_table(&self, db: &str, table: &str, ignore_unknown: bool) -> Result<()> {
        let mut store = self.client.raw.write().await;
        if !store.table_exists(db, table) {
            if ignore_unknown {
                return Ok(());
            }
            return Err(MockError::NotFound(format!(
                "Table '{}/{}' not found",
                db, table
            )));
        }
        store.delete_table(db, table);
        Ok(())
    }

    pub async fn list_tables(&self, db: &str) -> Result<Vec<String>> {
        let store = self.client.raw.read().await;
        if !store.db_exists(db) {
            return Err(MockError::NotFound(format!("Database '{}' not found", db)));
        }
        Ok(store.list_tables(db))
    }

    pub async fn insert_rows(&self, db: &str, table: &str, rows: &[RawRowCreate]) -> Result<()> {
        let mut store = self.client.raw.write().await;
        if !store.table_exists(db, table) {
            return Err(MockError::NotFound(format!(
                "Table '{}/{}' not found",
                db, table
            )));
        }
        let owned: Vec<RawRow> = rows
            .iter()
            .map(|r| RawRow {
                key: r.key.clone(),
                columns: r.columns.clone(),
                last_updated_time: 0,
            })
            .collect();
        store.insert_rows(db, table, owned);
        Ok(())
    }

    pub async fn retrieve_rows(
        &self,
        db: &str,
        table: &str,
        keys: &[String],
    ) -> Result<Vec<RawRow>> {
        let store = self.client.raw.read().await;
        if !store.table_exists(db, table) {
            return Err(MockError::NotFound(format!(
                "Table '{}/{}' not found",
                db, table
            )));
        }
        Ok(store.retrieve_rows(db, table, keys))
    }

    pub async fn list_rows(
        &self,
        db: &str,
        table: &str,
        cursor: Option<String>,
        limit: Option<u32>,
    ) -> Result<ItemsVec<RawRow, Cursor>> {
        let store = self.client.raw.read().await;
        if !store.table_exists(db, table) {
            return Err(MockError::NotFound(format!(
                "Table '{}/{}' not found",
                db, table
            )));
        }
        let limit = limit.map(|l| l as usize).unwrap_or(DEFAULT_LIMIT);
        let (items, next_cursor) = store.list_rows(db, table, cursor.as_deref(), limit);
        Ok(ItemsVec {
            items,
            extra_fields: Cursor { next_cursor },
        })
    }

    pub async fn delete_rows(&self, db: &str, table: &str, keys: &[String]) -> Result<()> {
        let mut store = self.client.raw.write().await;
        if !store.table_exists(db, table) {
            return Err(MockError::NotFound(format!(
                "Table '{}/{}' not found",
                db, table
            )));
        }
        store.delete_rows(db, table, keys);
        Ok(())
    }
}
