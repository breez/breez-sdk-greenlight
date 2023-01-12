use crate::models::NodeState;

use super::db::SqliteStorage;
use anyhow::Result;

impl SqliteStorage {
    pub fn get_cached_item(&self, key: String) -> Result<Option<String>> {
        let res = self.get_connection()?.query_row(
            "SELECT value FROM cached_items WHERE key = ?1",
            [key],
            |row| row.get(0),
        );
        Ok(res.ok())
    }

    pub fn update_cached_item(&self, key: String, value: String) -> Result<()> {
        self.get_connection()?.execute(
            "INSERT OR REPLACE INTO cached_items (key, value) VALUES (?1,?2)",
            (key, value),
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn delete_cached_item(&self, key: String) -> Result<()> {
        self.get_connection()?
            .execute("DELETE FROM cached_items WHERE key = ?1", [key])?;
        Ok(())
    }

    pub fn set_node_state(&self, state: &NodeState) -> Result<()> {
        let serialized_state = serde_json::to_string(state)?;
        self.update_cached_item("node_state".to_string(), serialized_state)?;
        Ok(())
    }

    pub fn get_node_state(&self) -> Result<Option<NodeState>> {
        let state_str = self.get_cached_item("node_state".to_string())?;
        Ok(match state_str {
            Some(str) => serde_json::from_str(str.as_str())?,
            None => None,
        })
    }
}

#[test]
fn test_cached_items() {
    use crate::persist::test_utils;

    let storage = SqliteStorage::from_file(test_utils::create_test_sql_file("cache".to_string()));

    storage.init().unwrap();
    storage
        .update_cached_item("key1".to_string(), "val1".to_string())
        .unwrap();
    let item_value = storage.get_cached_item("key1".to_string()).unwrap();
    assert_eq!(item_value, Some("val1".to_string()));

    storage.delete_cached_item("key1".to_string()).unwrap();
    let item_value = storage.get_cached_item("key1".to_string()).unwrap();
    assert_eq!(item_value, None);
}
