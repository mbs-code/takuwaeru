use std::error::Error;

use crate::{
    api::{self},
    db::get_conn,
    model::{Page, PageParam},
};

#[tauri::command]
pub fn page_count(site_id: Option<i64>, url: Option<String>) -> Result<i64, String> {
    let do_steps = || -> Result<i64, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let count = api::page::list_count(&conn, &site_id, &url)?;
        Ok(count)
    }();

    return do_steps.map_err(|s| s.to_string());
}

#[tauri::command]
pub fn page_list(
    site_id: Option<i64>,
    page: i64,
    per_page: Option<i64>,
    order: Option<String>,
    desc: Option<bool>,
) -> Result<Vec<Page>, String> {
    let do_steps = || -> Result<Vec<Page>, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let v_limit = per_page.unwrap_or(10);
        let v_offset = (page - 1) * v_limit;
        let pages = api::page::list(
            &conn,
            &site_id,
            &None,
            &Some(v_limit),
            &Some(v_offset),
            &order,
            &desc,
        )?;

        Ok(pages)
    }();

    return do_steps.map_err(|s| s.to_string());
}

#[tauri::command]
pub fn page_get(page_id: i64) -> Result<Page, String> {
    let do_steps = || -> Result<Page, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let site = api::page::get(&conn, &page_id)?;
        Ok(site)
    }();

    return do_steps.map_err(|s| s.to_string());
}

// NOTE page_create は queue 経由で行う必要がある

#[tauri::command]
pub fn page_update(page_id: i64, param: PageParam) -> Result<Page, String> {
    let do_steps = || -> Result<Page, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let _ = api::page::update(
            &conn,
            &page_id,
            &param.site_id,
            &param.parent_page_id,
            &param.url,
            &param.title,
            &param.is_persist,
        )?;

        let new_page = api::page::get(&conn, &page_id)?;
        Ok(new_page)
    }();

    return do_steps.map_err(|s| s.to_string());
}

#[tauri::command]
pub fn page_delete(page_id: i64) -> Result<i64, String> {
    let do_steps = || -> Result<i64, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let _ = api::queue::delete(&conn, &None, &Some(page_id), &None)?;
        let _ = api::page::delete(&conn, &Some(page_id), &None, &false)?;

        Ok(page_id)
    }();

    return do_steps.map_err(|s| s.to_string());
}

///

#[tauri::command]
pub fn page_clear(site_id: i64, keep_persist: bool) -> Result<(), String> {
    let do_steps = || -> Result<(), Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let _ = api::queue::delete(&conn, &None, &None, &Some(site_id));
        let _ = api::page::delete(&conn, &None, &Some(site_id), &keep_persist)?;

        Ok(())
    }();

    return do_steps.map_err(|s| s.to_string());
}
