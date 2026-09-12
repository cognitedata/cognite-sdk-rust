use cognite::raw::RawRow;
use std::collections::HashMap;

#[derive(Default)]
pub struct RawStore {
    /// database → table → key → row
    pub data: HashMap<String, HashMap<String, HashMap<String, RawRow>>>,
}

impl RawStore {
    pub fn create_db(&mut self, name: &str) {
        self.data.entry(name.to_string()).or_default();
    }

    pub fn delete_db(&mut self, name: &str) {
        self.data.remove(name);
    }

    pub fn list_dbs(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    pub fn create_table(&mut self, db: &str, table: &str) {
        self.data
            .entry(db.to_string())
            .or_default()
            .entry(table.to_string())
            .or_default();
    }

    pub fn delete_table(&mut self, db: &str, table: &str) {
        if let Some(db_map) = self.data.get_mut(db) {
            db_map.remove(table);
        }
    }

    pub fn list_tables(&self, db: &str) -> Vec<String> {
        self.data
            .get(db)
            .map(|t| t.keys().cloned().collect())
            .unwrap_or_default()
    }

    pub fn insert_rows(&mut self, db: &str, table: &str, rows: Vec<RawRow>) {
        let table_map = self
            .data
            .entry(db.to_string())
            .or_default()
            .entry(table.to_string())
            .or_default();
        for row in rows {
            table_map.insert(row.key.clone(), row);
        }
    }

    pub fn retrieve_rows(&self, db: &str, table: &str, keys: &[String]) -> Vec<RawRow> {
        let Some(table_map) = self.data.get(db).and_then(|d| d.get(table)) else {
            return vec![];
        };
        keys.iter()
            .filter_map(|k| table_map.get(k))
            .cloned()
            .collect()
    }

    pub fn list_rows(
        &self,
        db: &str,
        table: &str,
        cursor: Option<&str>,
        limit: usize,
    ) -> (Vec<RawRow>, Option<String>) {
        let Some(table_map) = self.data.get(db).and_then(|d| d.get(table)) else {
            return (vec![], None);
        };
        let mut rows: Vec<RawRow> = table_map.values().cloned().collect();
        rows.sort_by(|a, b| a.key.cmp(&b.key));

        let offset = cursor.and_then(|c| c.parse::<usize>().ok()).unwrap_or(0);
        let end = (offset + limit).min(rows.len());
        let next_cursor = if end < rows.len() {
            Some(end.to_string())
        } else {
            None
        };
        (rows[offset..end].to_vec(), next_cursor)
    }

    pub fn delete_rows(&mut self, db: &str, table: &str, keys: &[String]) {
        if let Some(table_map) = self.data.get_mut(db).and_then(|d| d.get_mut(table)) {
            for key in keys {
                table_map.remove(key);
            }
        }
    }

    pub fn db_exists(&self, name: &str) -> bool {
        self.data.contains_key(name)
    }

    pub fn table_exists(&self, db: &str, table: &str) -> bool {
        self.data
            .get(db)
            .map(|d| d.contains_key(table))
            .unwrap_or(false)
    }
}
