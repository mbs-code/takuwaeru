use std::error::Error;

use rusqlite::Connection;
use sql_builder::{bind::Bind, quote, SqlBuilder};

use crate::{
    api,
    db::chrono_now,
    model::{Site, SiteQueryParam},
};

pub fn list_count(conn: &Connection) -> Result<i64, Box<dyn Error>> {
    let mut builder = SqlBuilder::select_from("sites");
    let sql = builder.count("*").sql()?;

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let count = conn.query_row::<i64, _, _>(&sql, [], |row| row.get(0))?;

    Ok(count)
}

pub fn list(
    conn: &Connection,
    limit: &Option<i64>,
    offset: &Option<i64>,
    order: &Option<String>,
    desc: &Option<bool>,
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

pub fn get(conn: &Connection, site_id: &i64) -> Result<Site, Box<dyn Error>> {
    let sql = SqlBuilder::select_from("sites")
        .field("*")
        .and_where("id = ?".bind(site_id))
        .sql()?;

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let site = conn.query_row(&sql, [], |row| Site::by_row(row))?;
    Ok(site)
}

pub fn create(
    conn: &Connection,
    key: &String,
    url: &String,
    title: &Option<String>,
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
        .bind(key)
        .bind(url)
        .bind(title);

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let _ = conn.execute_batch(&sql)?;
    let site_id = conn.last_insert_rowid();

    #[cfg(debug_assertions)]
    println!("site_id: {}", site_id);

    Ok(site_id)
}

pub fn update(
    conn: &Connection,
    site_id: &i64,
    key: &String,
    url: &String,
    title: &Option<String>,
) -> Result<i64, Box<dyn Error>> {
    let now = chrono_now();
    let sql = SqlBuilder::update_table("sites")
        .set("key", "?")
        .set("url", "?")
        .set("title", "?")
        .set("updated_at", &quote(&now))
        .and_where("id = ?".bind(site_id))
        .sql()?
        .bind(key)
        .bind(url)
        .bind(title);

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let _ = conn.execute_batch(&sql)?;

    #[cfg(debug_assertions)]
    println!("site_id: {}", &site_id);

    Ok(site_id.clone())
}

pub fn delete(conn: &Connection, site_id: &i64) -> Result<(), Box<dyn Error>> {
    let sql = SqlBuilder::delete_from("sites")
        .and_where("id = ?".bind(site_id))
        .sql()?;

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let _ = conn.execute_batch(&sql)?;

    Ok(())
}

/// ////////////////////////////////////////////////////////////

pub fn sync_site_queries(
    conn: &Connection,
    site_id: &i64,
    new_site_queries: &Vec<SiteQueryParam>,
) -> Result<(), Box<dyn Error>> {
    // DBの値を拾ってくる（先に拾わないと余計なものを削除してしまう）
    let db_site_queries = api::site_query::list(
        &conn,
        &Some(site_id.clone()),
        &None,
        &None,
        &None,
        &None,
        &None,
    )?;

    // ID 基準で、クエリをDBに突っ込む
    for site_query in new_site_queries {
        // ID があったら create, 別なら update
        let site_query_id = site_query.id.unwrap_or(0);
        println!("qid {:?}", site_query_id);
        if site_query_id > 0 {
            api::site_query::update(
                &conn,
                &site_query_id,
                &site_id,
                &site_query.key,
                &site_query.url_pattern,
                &site_query.processor,
                &site_query.url_filter,
                &site_query.priority,
            )?;
        } else {
            api::site_query::create(
                &conn,
                &site_id,
                &site_query.key,
                &site_query.url_pattern,
                &site_query.processor,
                &site_query.url_filter,
                &site_query.priority,
            )?;
        }
    }

    // DBレコードを回して、クエリに無いものを削除
    for db_site_query in db_site_queries {
        // クエリの中に ID が存在するか確認
        let has = new_site_queries
            .iter()
            .filter(|&site_query| db_site_query.id.eq(&site_query.id.unwrap_or(0)))
            .count();

        // 一つも無いなら削除
        println!("has {:?}", has);
        if has == 0 {
            api::site_query::delete(&conn, &Some(db_site_query.id), &None)?;
        }
    }

    Ok(())
}
