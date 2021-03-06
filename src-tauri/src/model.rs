use rusqlite::{Error, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Site {
    pub id: i64,
    pub key: String,
    pub url: String,
    pub title: Option<String>,
    pub analysis_count: i64,
    pub download_count: i64,
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
            analysis_count: row.get(4)?,
            download_count: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
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
    pub dom_selector: Option<String>,
    pub url_filter: String,
    pub filename: Option<String>,
    pub is_persist: bool,
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
            dom_selector: row.get(5)?,
            url_filter: row.get(6)?,
            filename: row.get(7)?,
            is_persist: row.get(8)?,
            priority: row.get(9)?,
            created_at: row.get(10)?,
            updated_at: row.get(11)?,
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
    pub analysis_count: i64,
    pub download_count: i64,
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
            analysis_count: site.analysis_count,
            download_count: site.download_count,
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
    pub analysis_count: i64,
    pub download_count: i64,
    pub site_queries: Vec<SiteQueryParam>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteQueryParam {
    pub id: Option<i64>, // id ????????????????????????????????????????????????
    pub key: String,
    pub url_pattern: String,
    pub processor: String,
    pub dom_selector: Option<String>,
    pub url_filter: String,
    pub filename: Option<String>,
    pub is_persist: bool,
    pub priority: i64,
}
/// ////////////////////////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub id: i64,
    pub site_id: i64,
    pub parent_page_id: Option<i64>,
    pub url: String,
    pub title: Option<String>,
    pub is_persist: bool,
    pub created_at: String,
    pub updated_at: String,
}
impl Page {
    pub fn by_row(row: &Row) -> Result<Page, Error> {
        Ok(Page {
            id: row.get(0)?,
            site_id: row.get(1)?,
            parent_page_id: row.get(2)?,
            url: row.get(3)?,
            title: row.get(4)?,
            is_persist: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PageOrBool {
    Value(Page),
    Bool(bool),
}

///

#[derive(Debug, Serialize, Deserialize)]
pub struct PageParam {
    pub site_id: i64,
    pub parent_page_id: Option<i64>,
    pub url: String,
    pub title: Option<String>,
    pub is_persist: bool,
}

/// ////////////////////////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize)]
pub struct Queue {
    pub id: i64,
    pub site_id: i64,
    pub page_id: i64,
    pub priority: i64,
    pub created_at: String,
    pub updated_at: String,
}
impl Queue {
    pub fn by_row(row: &Row) -> Result<Queue, Error> {
        Ok(Queue {
            id: row.get(0)?,
            site_id: row.get(1)?,
            page_id: row.get(2)?,
            priority: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    }
}

///

#[derive(Debug, Serialize, Deserialize)]
pub struct QueueWithPage {
    pub id: i64,
    pub site_id: i64,
    pub page_id: i64,
    pub priority: i64,
    pub created_at: String,
    pub updated_at: String,
    pub page: Page,
}
impl QueueWithPage {
    pub fn new(queue: Queue, page: Page) -> QueueWithPage {
        QueueWithPage {
            id: queue.id,
            site_id: queue.site_id,
            page_id: queue.page_id,
            priority: queue.priority,
            created_at: queue.created_at,
            updated_at: queue.updated_at,
            page,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QueueWithPageOrBool {
    Value(QueueWithPage),
    Bool(bool),
}

///

#[derive(Debug, Serialize, Deserialize)]
pub struct QueueParam {
    pub url: String,
    pub priority: i64,
    pub parent_page_id: Option<i64>,
    pub is_persist: bool,
}
