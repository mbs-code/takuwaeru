use std::error::Error;

use sql_builder::{bind::Bind, quote, SqlBuilder};

use crate::{
    db::{chrono_now, get_conn},
    model::Site,
};

#[tauri::command]
pub fn get(page_id: &i64) -> Result<Site, Box<dyn Error>> {
    let conn = get_conn()?.lock()?;

    let sql = SqlBuilder::select_from("sites")
        .field("*")
        .and_where("id = ?".bind(&page_id))
        .sql()?;

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let site = conn.query_row(&sql, [], |row| Site::by_row(row))?;
    Ok(site)
}

#[tauri::command]
pub fn create(url: &String, title: &Option<String>) -> Result<Site, Box<dyn Error>> {
    let conn = get_conn()?.lock()?;

    let now = &chrono_now();
    let sql = SqlBuilder::insert_into("sites")
        .field("url")
        .field("title")
        .field("created_at")
        .field("updated_at")
        .values(&["?", "?", &quote(now), &quote(now)])
        .sql()?
        .bind(url)
        .bind(title);

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let _ = conn.execute(&sql, [])?;
    let site_id = conn.last_insert_rowid();

    #[cfg(debug_assertions)]
    println!("site_id: {}", &site_id);

    // 再取得のため開放
    drop(conn);
    let new_site = get(&site_id)?;
    Ok(new_site)
}

#[tauri::command]
pub fn update(site_id: &i64, url: &String, title: &Option<String>) -> Result<Site, Box<dyn Error>> {
    let conn = get_conn()?.lock()?;

    let now = &chrono_now();
    let sql = SqlBuilder::update_table("sites")
        .set("url", "?")
        .set("title", "?")
        .set("updated_at", &quote(now))
        .and_where("id = ?".bind(site_id))
        .sql()?
        .bind(url)
        .bind(title);

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let _ = conn.execute(&sql, [])?;

    #[cfg(debug_assertions)]
    println!("site_id: {}", &site_id);

    // 再取得のため開放
    drop(conn);
    let new_site = get(&site_id)?;
    Ok(new_site)
}

#[tauri::command]
pub fn delete(site_id: &i64) -> Result<(), Box<dyn Error>> {
    let conn = get_conn()?.lock()?;

    let sql = SqlBuilder::delete_from("sites")
        .and_where("id = ?".bind(site_id))
        .sql()?;

    #[cfg(debug_assertions)]
    println!("{:?}", &sql);

    let _ = conn.execute(&sql, [])?;

    Ok(())
}
