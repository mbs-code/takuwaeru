use std::error::Error;

use rusqlite::Connection;
use sql_builder::{bind::Bind, quote, SqlBuilder};

use crate::{db::chrono_now, model::Page};

pub fn list_count(
    conn: &Connection,
    site_id: &Option<i64>,
    url: &Option<String>,
) -> Result<i64, Box<dyn Error>> {
    let mut builder = SqlBuilder::select_from("pages");

    if let Some(v_site_id) = site_id {
        builder.and_where("site_id = ?".bind(&v_site_id));
    }

    if let Some(v_url) = url {
        builder.and_where("url = ?".bind(v_url));
    }

    let sql = builder.count("*").sql()?;

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let count = conn.query_row::<i64, _, _>(&sql, [], |row| row.get(0))?;

    Ok(count)
}

pub fn list(
    conn: &Connection,
    site_id: &Option<i64>,
    parent_id: &Option<i64>,
    limit: &Option<i64>,
    offset: &Option<i64>,
    order: &Option<String>,
    desc: &Option<bool>,
) -> Result<Vec<Page>, Box<dyn Error>> {
    let mut builder = SqlBuilder::select_from("pages");

    if let Some(v_site_id) = site_id {
        builder.and_where("site_id = ?".bind(&v_site_id));
    }

    if let Some(v_parent_id) = parent_id {
        builder.and_where("parent_id = ?".bind(&v_parent_id));
    }

    if let Some(v_order) = order {
        let v_desc = desc.unwrap_or(false);
        builder.order_by(quote(v_order), v_desc);
    }
    if let Some(v_limit) = limit {
        builder.limit(v_limit);
    }
    if let Some(v_offset) = offset {
        builder.offset(v_offset);
    }
    let sql = builder.field("*").sql()?;

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let mut stmt = conn.prepare(&sql)?;
    let pages = stmt
        .query_map([], |row| Page::by_row(row))
        .unwrap()
        .map(|r| r.unwrap())
        .collect::<Vec<Page>>();

    Ok(pages)
}

pub fn get(conn: &Connection, page_id: &i64) -> Result<Page, Box<dyn Error>> {
    let sql = SqlBuilder::select_from("pages")
        .field("*")
        .and_where("id = ?".bind(&page_id))
        .sql()?;

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let page = conn.query_row(&sql, [], |row| Page::by_row(row))?;
    Ok(page)
}

pub fn create(
    conn: &Connection,
    site_id: &i64,
    parent_page_id: &Option<i64>,
    url: &String,
    title: &Option<String>,
    is_persist: &bool,
) -> Result<i64, Box<dyn Error>> {
    let now = chrono_now();
    let sql = SqlBuilder::insert_into("pages")
        .field("site_id")
        .field("parent_page_id")
        .field("url")
        .field("title")
        .field("is_persist")
        .field("created_at")
        .field("updated_at")
        .values(&[
            ":site_id:",
            ":parent_page_id:",
            ":url:",
            ":title:",
            ":is_persist:",
            &quote(&now),
            &quote(&now),
        ])
        .sql()?
        .bind_name(&"site_id", site_id)
        .bind_name(&"parent_page_id", parent_page_id)
        .bind_name(&"url", url)
        .bind_name(&"title", title)
        .bind_name(&"is_persist", is_persist);

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let _ = conn.execute_batch(&sql)?;
    let page_id = conn.last_insert_rowid();

    #[cfg(debug_assertions)]
    println!("page_id: {}", &page_id);

    Ok(page_id)
}

pub fn update(
    conn: &Connection,
    page_id: &i64,
    site_id: &i64,
    parent_page_id: &Option<i64>,
    url: &String,
    title: &Option<String>,
    is_persist: &bool,
) -> Result<i64, Box<dyn Error>> {
    let now = chrono_now();
    let sql = SqlBuilder::update_table("pages")
        .set("site_id", ":site_id:")
        .set("parent_page_id", ":parent_page_id:")
        .set("url", ":url:")
        .set("title", ":title:")
        .set("is_persist", ":is_persist:")
        .set("updated_at", &quote(&now))
        .and_where("id = ?".bind(page_id))
        .sql()?
        .bind_name(&"site_id", site_id)
        .bind_name(&"parent_page_id", parent_page_id)
        .bind_name(&"url", url)
        .bind_name(&"title", title)
        .bind_name(&"is_persist", is_persist);

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let _ = conn.execute_batch(&sql)?;

    #[cfg(debug_assertions)]
    println!("page_id: {}", &page_id);

    Ok(page_id.clone())
}

pub fn delete(
    conn: &Connection,
    page_id: &Option<i64>,
    site_id: &Option<i64>,
    keep_persist: &bool,
) -> Result<(), Box<dyn Error>> {
    let mut builder = SqlBuilder::delete_from("pages");

    if let Some(v_page_id) = page_id {
        builder.and_where("id = ?".bind(v_page_id));
    }
    if let Some(v_site_id) = site_id {
        builder.and_where("site_id = ?".bind(v_site_id));
    }

    if *keep_persist {
        builder.and_where("is_persist <> ?".bind(&true));
    }

    let sql = builder.sql()?;

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let _ = conn.execute_batch(&sql)?;

    Ok(())
}
