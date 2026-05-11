use std::sync::Arc;
use redb::{Database, TableDefinition, ReadableTable};
use std::path::Path;

const HISTORY_TABLE: TableDefinition<&str, &str> = TableDefinition::new("history");

pub struct AppState {
    pub db: Arc<Database>,
}

impl AppState {
    pub fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let db = Database::create(path)?;
        
        // Initialize table
        let write_txn = db.begin_write()?;
        {
            let _ = write_txn.open_table(HISTORY_TABLE)?;
        }
        write_txn.commit()?;

        Ok(Self {
            db: Arc::new(db),
        })
    }

    pub async fn save_history(&self, id: String, json_data: String) -> anyhow::Result<()> {
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            let write_txn = db.begin_write()?;
            {
                let mut table = write_txn.open_table(HISTORY_TABLE)?;
                table.insert(id.as_str(), json_data.as_str())?;
            }
            write_txn.commit()?;
            Ok::<(), anyhow::Error>(())
        }).await?
    }

    pub async fn get_all_history(&self) -> anyhow::Result<Vec<serde_json::Value>> {
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            let read_txn = db.begin_read()?;
            let table = read_txn.open_table(HISTORY_TABLE)?;
            let mut history = Vec::new();
            for item in table.iter()? {
                let (_, value) = item?;
                let val: serde_json::Value = serde_json::from_str(value.value())?;
                history.push(val);
            }
            // Sort or limit could be added here
            Ok::<Vec<serde_json::Value>, anyhow::Error>(history)
        }).await?
    }
}
