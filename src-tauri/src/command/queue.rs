use std::error::Error;

use crate::{
    api::{self},
    db::get_conn,
    model::{Page, PageParam, Queue, QueueParam},
};

#[tauri::command]
pub fn queue_count(site_id: Option<i64>) -> Result<i64, String> {
    let do_steps = || -> Result<i64, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let count = api::queue::list_count(&conn, &site_id)?;
        Ok(count)
    }();

    return do_steps.map_err(|s| s.to_string());
}

#[tauri::command]
pub fn queue_list(
    site_id: Option<i64>,
    page: i64,
    per_page: Option<i64>,
    order: Option<String>,
    desc: Option<bool>,
) -> Result<Vec<Queue>, String> {
    let do_steps = || -> Result<Vec<Queue>, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let v_limit = per_page.unwrap_or(10);
        let v_offset = (page - 1) * v_limit;
        let queues = api::queue::list(
            &conn,
            &site_id,
            &Some(v_limit),
            &Some(v_offset),
            &order,
            &desc,
        )?;

        Ok(queues)
    }();

    return do_steps.map_err(|s| s.to_string());
}

#[tauri::command]
pub fn queue_get(queue_id: i64) -> Result<Queue, String> {
    let do_steps = || -> Result<Queue, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let queue = api::queue::get(&conn, &queue_id)?;
        Ok(queue)
    }();

    return do_steps.map_err(|s| s.to_string());
}

#[tauri::command]
pub fn queue_create(param: QueueParam) -> Result<Queue, String> {
    let do_steps = || -> Result<Queue, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let queue_id = api::queue::create(&conn, &param.site_id, &param.page_id, &param.priority)?;

        let new_queue = api::queue::get(&conn, &queue_id)?;
        Ok(new_queue)
    }();

    return do_steps.map_err(|s| s.to_string());
}

#[tauri::command]
pub fn queue_update(queue_id: i64, param: QueueParam) -> Result<Queue, String> {
    let do_steps = || -> Result<Queue, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let _ = api::queue::update(
            &conn,
            &queue_id,
            &param.site_id,
            &param.page_id,
            &param.priority,
        )?;

        let new_queue = api::queue::get(&conn, &queue_id)?;
        Ok(new_queue)
    }();

    return do_steps.map_err(|s| s.to_string());
}

#[tauri::command]
pub fn queue_delete(queue_id: i64) -> Result<i64, String> {
    let do_steps = || -> Result<i64, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let _ = api::queue::delete(&conn, &Some(queue_id), &None, &None)?;

        Ok(queue_id)
    }();

    return do_steps.map_err(|s| s.to_string());
}

///

#[tauri::command]
pub fn queue_clear(site_id: i64) -> Result<(), String> {
    let do_steps = || -> Result<(), Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let _ = api::queue::delete(&conn, &None, &None, &Some(site_id))?;

        Ok(())
    }();

    return do_steps.map_err(|s| s.to_string());
}
