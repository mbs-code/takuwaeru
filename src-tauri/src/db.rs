use std::{error::Error, sync::Mutex};

use chrono::Utc;
use once_cell::sync::OnceCell;
use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};

static DB_INSTANCE: OnceCell<Mutex<Connection>> = OnceCell::new();

/** DBコネクションの取得 */
pub fn get_conn() -> Result<&'static Mutex<Connection>, Box<dyn Error>> {
    if DB_INSTANCE.get().is_none() {
        let new_conn = establish_connection()?;
        let _ = DB_INSTANCE.set(Mutex::new(new_conn));
    }

    let res = DB_INSTANCE.get().unwrap();
    Ok(res)
}

/** UTC 現在時刻 */
pub fn chrono_now() -> String {
    return Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
}

///

/** ビルドインマイグレ */
fn generate_migrations() -> Migrations<'static> {
    return Migrations::new(vec![M::up(include_str!(
        "../migrations/20220522_150500_init.sql"
    ))]);
}

/** DB接続確立 */
fn establish_connection() -> Result<Connection, Box<dyn Error>> {
    // 接続確立
    let mut conn = Connection::open("strage.db")?;

    // マイグレーション
    let migrations = generate_migrations();
    migrations.to_latest(&mut conn)?;

    Ok(conn)
}
