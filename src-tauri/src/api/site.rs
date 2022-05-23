use std::error::Error;

use rusqlite::Connection;
use sql_builder::{bind::Bind, quote, SqlBuilder};

use crate::{db::chrono_now, model::Site};

pub fn list(
    conn: &Connection,
    limit: Option<i64>,
    offset: Option<i64>,
    order: Option<String>,
    desc: Option<bool>,
) -> Result<Vec<Site>, Box<dyn Error>> {
    let mut builder = SqlBuilder::select_from("sites");

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
    let sites = stmt
        .query_map([], |row| Site::by_row(row))
        .unwrap()
        .map(|r| r.unwrap())
        .collect::<Vec<Site>>();

    Ok(sites)
}

pub fn get(conn: &Connection, site_id: i64) -> Result<Site, Box<dyn Error>> {
    let sql = SqlBuilder::select_from("sites")
        .field("*")
        .and_where("id = ?".bind(&site_id))
        .sql()?;

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let site = conn.query_row(&sql, [], |row| Site::by_row(row))?;
    Ok(site)
}

pub fn create(
    conn: &Connection,
    key: String,
    url: String,
    title: Option<String>,
) -> Result<i64, Box<dyn Error>> {
    let now = chrono_now();
    let sql = SqlBuilder::insert_into("sites")
        .field("key")
        .field("url")
        .field("title")
        .field("created_at")
        .field("updated_at")
        .values(&["?", "?", "?", &quote(&now), &quote(&now)])
        .sql()?
        .bind(&key)
        .bind(&url)
        .bind(&title);

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let _ = conn.execute(&sql, [])?;
    let site_id = conn.last_insert_rowid();

    #[cfg(debug_assertions)]
    println!("site_id: {}", &site_id);

    Ok(site_id)
}

pub fn update(
    conn: &Connection,
    site_id: i64,
    key: String,
    url: String,
    title: Option<String>,
) -> Result<i64, Box<dyn Error>> {
    let now = chrono_now();
    let sql = SqlBuilder::update_table("sites")
        .set("key", "?")
        .set("url", "?")
        .set("title", "?")
        .set("updated_at", &quote(&now))
        .and_where("id = ?".bind(&site_id))
        .sql()?
        .bind(&key)
        .bind(&url)
        .bind(&title);

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let _ = conn.execute(&sql, [])?;

    #[cfg(debug_assertions)]
    println!("site_id: {}", &site_id);

    Ok(site_id)
}

pub fn delete(conn: &Connection, site_id: i64) -> Result<i64, Box<dyn Error>> {
    let sql = SqlBuilder::delete_from("sites")
        .and_where("id = ?".bind(&site_id))
        .sql()?;

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let _ = conn.execute(&sql, [])?;

    Ok(site_id)
}
