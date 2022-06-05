use std::error::Error;

use rusqlite::Connection;
use sql_builder::{bind::Bind, quote, SqlBuilder};

use crate::{
    db::chrono_now,
    model::{Page, Queue},
};

pub fn list_count(conn: &Connection, site_id: &Option<i64>) -> Result<i64, Box<dyn Error>> {
    let mut builder = SqlBuilder::select_from("queues");

    if let Some(v_site_id) = site_id {
        builder.and_where("site_id = ?".bind(&v_site_id));
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
    limit: &Option<i64>,
    offset: &Option<i64>,
    order: &Option<String>,
    desc: &Option<bool>,
) -> Result<Vec<Queue>, Box<dyn Error>> {
    let mut builder = SqlBuilder::select_from("queues");

    if let Some(v_site_id) = site_id {
        builder.and_where("site_id = ?".bind(&v_site_id));
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
        .query_map([], |row| Queue::by_row(row))
        .unwrap()
        .map(|r| r.unwrap())
        .collect::<Vec<Queue>>();

    Ok(pages)
}

pub fn get(conn: &Connection, queue_id: &i64) -> Result<Queue, Box<dyn Error>> {
    let sql = SqlBuilder::select_from("queues")
        .field("*")
        .and_where("id = ?".bind(&queue_id))
        .sql()?;

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let queue = conn.query_row(&sql, [], |row| Queue::by_row(row))?;
    Ok(queue)
}

pub fn create(
    conn: &Connection,
    site_id: &i64,
    page_id: &i64,
    priority: &i64,
) -> Result<i64, Box<dyn Error>> {
    let now = chrono_now();
    let sql = SqlBuilder::insert_into("queues")
        .field("site_id")
        .field("page_id")
        .field("priority")
        .values(&[
            ":site_id:",
            ":page_id:",
            ":priority:",
            &quote(&now),
            &quote(&now),
        ])
        .sql()?
        .bind_name(&"site_id", site_id)
        .bind_name(&"page_id", page_id)
        .bind_name(&"priority", priority);

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let _ = conn.execute_batch(&sql)?;
    let queue_id = conn.last_insert_rowid();

    #[cfg(debug_assertions)]
    println!("queue_id: {}", &queue_id);

    Ok(queue_id)
}

pub fn update(
    conn: &Connection,
    queue_id: &i64,
    site_id: &i64,
    page_id: &i64,
    priority: &i64,
) -> Result<i64, Box<dyn Error>> {
    let now = chrono_now();
    let sql = SqlBuilder::update_table("queues")
        .set("site_id", ":site_id:")
        .set("page_id", ":page_id:")
        .set("priority", ":priority:")
        .set("updated_at", &quote(&now))
        .and_where("id = ?".bind(queue_id))
        .sql()?
        .bind_name(&"site_id", site_id)
        .bind_name(&"page_id", page_id)
        .bind_name(&"priority", priority);

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let _ = conn.execute_batch(&sql)?;

    #[cfg(debug_assertions)]
    println!("queue_id: {}", &queue_id);

    Ok(queue_id.clone())
}

pub fn delete(
    conn: &Connection,
    queue_id: &Option<i64>,
    page_id: &Option<i64>,
    site_id: &Option<i64>,
) -> Result<(), Box<dyn Error>> {
    let mut builder = SqlBuilder::delete_from("queues");

    if let Some(v_queue_id) = queue_id {
        builder.and_where("id = ?".bind(v_queue_id));
    }
    if let Some(v_page_id) = page_id {
        builder.and_where("page_id = ?".bind(v_page_id));
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
