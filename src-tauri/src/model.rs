use rusqlite::{Error, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Site {
    pub id: i64,
    pub url: String,
    pub title: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
impl Site {
    pub fn by_row(row: &Row) -> Result<Site, Error> {
        Ok(Site {
            id: row.get(0)?,
            url: row.get(1)?,
            title: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    }
}
