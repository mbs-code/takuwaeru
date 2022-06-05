use std::error::Error;

use crate::{
    api,
    db::get_conn,
    model::{QueueParam, QueueWithPage, QueueWithPageOrBool},
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
) -> Result<Vec<QueueWithPage>, String> {
    let do_steps = || -> Result<Vec<QueueWithPage>, Box<dyn Error>> {
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

        let queue_with_page = queues
            .into_iter()
            .map(|queue| {
                let page = api::page::get(&conn, &queue.page_id).unwrap();
                QueueWithPage::new(queue, page)
            })
            .collect::<Vec<QueueWithPage>>();
        Ok(queue_with_page)
    }();

    return do_steps.map_err(|s| s.to_string());
}

#[tauri::command]
pub fn queue_get(queue_id: i64) -> Result<QueueWithPage, String> {
    let do_steps = || -> Result<QueueWithPage, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let queue = api::queue::get(&conn, &queue_id)?;
        let page = api::page::get(&conn, &queue.page_id)?;
        Ok(QueueWithPage::new(queue, page))
    }();

    return do_steps.map_err(|s| s.to_string());
}

#[tauri::command]
pub fn queue_push(site_id: i64, param: QueueParam) -> Result<QueueWithPageOrBool, String> {
    let do_steps = || -> Result<QueueWithPageOrBool, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        // 存在チェックを行う
        let page_count = api::page::list_count(&conn, &Some(site_id), &Some(param.url.clone()))?;
        if page_count > 0 {
            return Ok(QueueWithPageOrBool::Bool(false));
            // Err(new_err("This URL already exists"))?
        }

        // 親IDがあるなら取得する
        let parent_page_id = match param.parent_page_id {
            Some(v) => Some(v),
            None => None,
        };

        // ページを作成する
        let new_page_id = api::page::create(&conn, &site_id, &parent_page_id, &param.url, &None)?;

        // キューに挿入する
        let new_queue_id = api::queue::create(&conn, &site_id, &new_page_id, &param.priority)?;

        // 返却値生成
        let new_queue = api::queue::get(&conn, &new_queue_id)?;
        let new_page = api::page::get(&conn, &new_page_id)?;
        Ok(QueueWithPageOrBool::Value(QueueWithPage::new(
            new_queue, new_page,
        )))
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
