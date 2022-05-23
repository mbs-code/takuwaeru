use std::error::Error;

use crate::{api, db::get_conn, model::Site};

#[tauri::command]
pub fn list(
    page: i64,
    per_page: Option<i64>,
    order: Option<String>,
    desc: Option<bool>,
) -> Result<Vec<Site>, String> {
    let do_steps = || -> Result<Vec<Site>, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let v_limit = per_page.unwrap_or(10);
        let v_offset = (page - 1) * v_limit;
        let sites = api::site::list(&conn, Some(v_limit), Some(v_offset), order, desc)?;
        Ok(sites)
    }();

    return do_steps.map_err(|s| s.to_string());
}

#[tauri::command]
pub fn create(key: String, url: String, title: Option<String>) -> Result<Site, String> {
    let do_steps = || -> Result<Site, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let site_id = api::site::create(&conn, key, url, title)?;
        let new_site = api::site::get(&conn, site_id)?;
        Ok(new_site)
    }();

    return do_steps.map_err(|s| s.to_string());
}

#[tauri::command]
pub fn update(
    site_id: i64,
    key: String,
    url: String,
    title: Option<String>,
) -> Result<Site, String> {
    let do_steps = || -> Result<Site, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let _ = api::site::update(&conn, site_id, key, url, title)?;
        let new_site = api::site::get(&conn, site_id)?;
        Ok(new_site)
    }();

    return do_steps.map_err(|s| s.to_string());
}

#[tauri::command]
pub fn delete(site_id: i64) -> Result<i64, String> {
    let do_steps = || -> Result<i64, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let _ = api::site::delete(&conn, site_id)?;
        Ok(site_id)
    }();

    return do_steps.map_err(|s| s.to_string());
}
