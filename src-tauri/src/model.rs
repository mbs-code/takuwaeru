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
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteQuery {
    pub id: i64,
    pub site_id: i64,
    pub key: String,
    pub url_pattern: String,
    pub processor: String,
    pub url_filter: String,
    pub priority: i64,
    pub created_at: String,
    pub updated_at: String,
}
impl SiteQuery {
    pub fn by_row(row: &Row) -> Result<SiteQuery, Error> {
        Ok(SiteQuery {
            id: row.get(0)?,
            site_id: row.get(1)?,
            key: row.get(2)?,
            url_pattern: row.get(3)?,
            processor: row.get(4)?,
            url_filter: row.get(5)?,
            priority: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        })
    }
}

///

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteWithQuery {
    pub id: i64,
    pub key: String,
    pub url: String,
    pub title: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub site_queries: Vec<SiteQuery>,
}
impl SiteWithQuery {
    pub fn new(site: Site, queries: Vec<SiteQuery>) -> SiteWithQuery {
        SiteWithQuery {
            id: site.id,
            key: site.key,
            url: site.url,
            title: site.title,
            created_at: site.created_at,
            updated_at: site.updated_at,
            site_queries: queries,
        }
    }
}

///

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteParam {
    pub key: String,
    pub url: String,
    pub title: Option<String>,
    pub site_queries: Vec<SiteQueryParam>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteQueryParam {
    pub key: String,
    pub url_pattern: String,
    pub processor: String,
    pub url_filter: String,
    pub priority: i64,
}
