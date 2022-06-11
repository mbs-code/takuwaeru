use std::error::Error;

use rusqlite::Connection;
use sql_builder::{bind::Bind, quote, SqlBuilder};

use crate::{
    db::chrono_now,
    model::{SiteQuery, SiteQueryParam},
};

pub fn list(
    conn: &Connection,
    site_id: &Option<i64>,
    site_ids: &Option<Vec<i64>>,
    limit: &Option<i64>,
    offset: &Option<i64>,
    order: &Option<String>,
    desc: &Option<bool>,
) -> Result<Vec<SiteQuery>, Box<dyn Error>> {
    let mut builder = SqlBuilder::select_from("site_queries");

    if let Some(v_site_id) = site_id {
        builder.and_where("site_id = ?".bind(&v_site_id));
    }
    if let Some(v_site_ids) = site_ids {
        builder.and_where_in("site_id", &v_site_ids);
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

    builder.order_by("id", false);
    let sql = builder.field("*").sql()?;

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let mut stmt = conn.prepare(&sql)?;
    let site_queries = stmt
        .query_map([], |row| SiteQuery::by_row(row))
        .unwrap()
        .map(|r| r.unwrap())
        .collect::<Vec<SiteQuery>>();

    Ok(site_queries)
}

pub fn get(conn: &Connection, site_query_id: &i64) -> Result<SiteQuery, Box<dyn Error>> {
    let sql = SqlBuilder::select_from("site_queries")
        .field("*")
        .and_where("id = ?".bind(&site_query_id))
        .sql()?;

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let site_query = conn.query_row(&sql, [], |row| SiteQuery::by_row(row))?;
    Ok(site_query)
}

pub fn create(
    conn: &Connection,
    site_id: &i64,
    params: &SiteQueryParam,
) -> Result<i64, Box<dyn Error>> {
    let now = chrono_now();
    let sql = SqlBuilder::insert_into("site_queries")
        .field("site_id")
        .field("key")
        .field("url_pattern")
        .field("processor")
        .field("dom_selector")
        .field("url_filter")
        .field("filename")
        .field("is_persist")
        .field("priority")
        .field("created_at")
        .field("updated_at")
        .values(&[
            ":site_id:",
            ":key:",
            ":url_pattern:",
            ":processor:",
            ":dom_selector:",
            ":url_filter:",
            ":filename:",
            ":is_persist:",
            ":priority:",
            &quote(&now),
            &quote(&now),
        ])
        .sql()?
        .bind_name(&"site_id", &site_id)
        .bind_name(&"key", &params.key)
        .bind_name(&"url_pattern", &params.url_pattern)
        .bind_name(&"processor", &params.processor)
        .bind_name(&"dom_selector", &params.dom_selector)
        .bind_name(&"url_filter", &params.url_filter)
        .bind_name(&"filename", &params.filename)
        .bind_name(&"is_persist", &params.is_persist)
        .bind_name(&"priority", &params.priority);

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let _ = conn.execute_batch(&sql)?;
    let site_query_id = conn.last_insert_rowid();

    #[cfg(debug_assertions)]
    println!("site_query_id: {}", &site_query_id);

    Ok(site_query_id)
}

pub fn update(
    conn: &Connection,
    site_query_id: &i64,
    site_id: &i64,
    params: &SiteQueryParam,
) -> Result<i64, Box<dyn Error>> {
    let now = chrono_now();
    let sql = SqlBuilder::update_table("site_queries")
        .set("site_id", ":site_id:")
        .set("key", ":key:")
        .set("url_pattern", ":url_pattern:")
        .set("processor", ":processor:")
        .set("dom_selector", ":dom_selector:")
        .set("url_filter", ":url_filter:")
        .set("filename", ":filename:")
        .set("is_persist", ":is_persist:")
        .set("priority", ":priority:")
        .set("updated_at", &quote(&now))
        .and_where("id = ?".bind(site_query_id))
        .sql()?
        .bind_name(&"site_id", &site_id)
        .bind_name(&"key", &params.key)
        .bind_name(&"url_pattern", &params.url_pattern)
        .bind_name(&"processor", &params.processor)
        .bind_name(&"dom_selector", &params.dom_selector)
        .bind_name(&"url_filter", &params.url_filter)
        .bind_name(&"filename", &params.filename)
        .bind_name(&"is_persist", &params.is_persist)
        .bind_name(&"priority", &params.priority);

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let _ = conn.execute_batch(&sql)?;

    #[cfg(debug_assertions)]
    println!("site_query_id: {}", &site_query_id);

    Ok(site_query_id.clone())
}

pub fn delete(
    conn: &Connection,
    site_query_id: &Option<i64>,
    site_id: &Option<i64>,
) -> Result<(), Box<dyn Error>> {
    let mut builder = SqlBuilder::delete_from("site_queries");

    if let Some(v_site_query_id) = site_query_id {
        builder.and_where("id = ?".bind(v_site_query_id));
    }
    if let Some(v_site_id) = site_id {
        builder.and_where("site_id = ?".bind(v_site_id));
    }

    let sql = builder.sql()?;

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let _ = conn.execute_batch(&sql)?;

    Ok(())
}
