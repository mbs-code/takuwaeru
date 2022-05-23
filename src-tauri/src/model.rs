use rusqlite::{Error, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Site {
    pub id: i64,
    pub key: String,
    pub url: String,
    pub title: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
impl Site {
    pub fn by_row(row: &Row) -> Result<Site, Error> {
        Ok(Site {
            id: row.get(0)?,
            key: row.get(1)?,
            url: row.get(2)?,
            title: row.get(3)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    }
}
