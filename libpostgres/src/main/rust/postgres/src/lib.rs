use std::ffi::{CStr, CString};

use anyhow::Result;
use log::{LevelFilter, info};
use tokio_postgres::NoTls;
mod pq_utils;

/// cargo build --lib --release --target x86_64-linux-android --target armv7-linux-androideabi --target aarch64-linux-android
///
/// cargo build --lib --release --target armv7-linux-androideabi

pub type QueryCallback = extern fn(*const u8, bool);

#[no_mangle]
pub extern fn init_log(){
    android_logger::init_once(android_logger::Config::default().with_max_level(LevelFilter::Info));
}

#[no_mangle]
pub extern fn query(host:*const u8, port:u16, user:*const u8, password:*const u8, dbname:*const u8, sql: *const u8, callback: QueryCallback){
    
    match query_sync(host, port, user, password, dbname, sql){
        Ok(json) => {
            let str = CString::new(format!("{json}")).unwrap();
            callback(str.as_ptr(), true);
        }
        Err(err) => {
            let err_str = CString::new(format!("{:?}", err)).unwrap();
            callback(err_str.as_ptr(), false);
        }
    }
}

fn query_sync(host:*const u8, port:u16, user:*const u8, password:*const u8, dbname:*const u8, sql: *const u8) -> Result<String>{
    let sql = unsafe{ CStr::from_ptr(sql) }.to_str()?;
    let host = unsafe{ CStr::from_ptr(host) }.to_str()?;
    let user = unsafe{ CStr::from_ptr(user) }.to_str()?;
    let password = unsafe{ CStr::from_ptr(password) }.to_str()?;
    let dbname = unsafe{ CStr::from_ptr(dbname) }.to_str()?;

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async{
        query_async(host, port, user, password, dbname, sql).await
    })
}

async fn query_async(host: &str, port:u16, user: &str, password: &str, dbname: &str, sql: &str) -> Result<String>{

    let config = format!("host={host} port={port} user={user} password={password} dbname={dbname}");

    info!("开始链接:{config}...");

    let (client, connection) =
        tokio_postgres::connect(&config, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    info!("执行sql:{sql}");

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client
        .query(sql, &[])
        .await?;

    info!("查询条数:{}", rows.len());

    // And then check that we got back the same string we sent over.
    let mut arr = vec![];

    for row in rows{
        arr.push(pq_utils::postgres_row_to_json_value(row)?);
    }

    let json = serde_json::to_string(&arr)?;

    Ok(json)
}