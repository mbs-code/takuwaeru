use std::error::Error;

use crate::{
    api::{self, site::sync_site_queries},
    db::get_conn,
    model::{SiteParam, SiteWithQuery},
};

#[tauri::command]
pub fn list(
    page: i64,
    per_page: Option<i64>,
    order: Option<String>,
    desc: Option<bool>,
) -> Result<Vec<SiteWithQuery>, String> {
    let do_steps = || -> Result<Vec<SiteWithQuery>, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let v_limit = per_page.unwrap_or(10);
        let v_offset = (page - 1) * v_limit;
        let sites = api::site::list(&conn, &Some(v_limit), &Some(v_offset), &order, &desc)?;

        let site_with_queries = sites
            .into_iter()
            .map(|site| {
                let queries =
                    api::site_query::list(&conn, &Some(site.id), &None, &None, &None, &None, &None)
                        .unwrap();
                SiteWithQuery::new(site, queries)
            })
            .collect::<Vec<SiteWithQuery>>();
        Ok(site_with_queries)
    }();

    return do_steps.map_err(|s| s.to_string());
}

#[tauri::command]
pub fn create(param: SiteParam) -> Result<SiteWithQuery, String> {
    let do_steps = || -> Result<SiteWithQuery, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let site_id = api::site::create(&conn, &param.key, &param.url, &param.title)?;
        sync_site_queries(&conn, &site_id, &param.site_queries)?;

        let new_site = api::site::get(&conn, &site_id)?;
        let queries =
            api::site_query::list(&conn, &Some(site_id), &None, &None, &None, &None, &None)?;
        Ok(SiteWithQuery::new(new_site, queries))
    }();

    return do_steps.map_err(|s| s.to_string());
}

#[tauri::command]
pub fn update(site_id: i64, param: SiteParam) -> Result<SiteWithQuery, String> {
    let do_steps = || -> Result<SiteWithQuery, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let _ = api::site::update(&conn, &site_id, &param.key, &param.url, &param.title)?;
        sync_site_queries(&conn, &site_id, &param.site_queries)?;

        let new_site = api::site::get(&conn, &site_id)?;
        let queries =
            api::site_query::list(&conn, &Some(site_id), &None, &None, &None, &None, &None)?;
        Ok(SiteWithQuery::new(new_site, queries))
    }();

    return do_steps.map_err(|s| s.to_string());
}

#[tauri::command]
pub fn delete(site_id: i64) -> Result<i64, String> {
    let do_steps = || -> Result<i64, Box<dyn Error>> {
        let conn = get_conn()?.lock()?;

        let _ = api::site_query::delete(&conn, &None, &Some(site_id));
        let _ = api::site::delete(&conn, &site_id)?;

        Ok(site_id)
    }();

    return do_steps.map_err(|s| s.to_string());
}
