extern crate dc;
use dc::MyDbPool;
use dc::DataBase;
use dc::stream;

#[macro_use]
extern crate easy_config;
use easy_config::CFG;

#[macro_use]
extern crate log;
extern crate elog;

extern crate chrono;
use chrono::*;

extern crate scheduler;
use scheduler::*;

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::str::FromStr;

use std::sync::{Arc};
use std::fs::File;
use std::io::Write;

use std::collections::BTreeMap;

fn run() -> Result<i32, i32> {
    info!(target:"main", "{}", CFG.get_data());
    let dsn = cfg_str!("db", "dsn");
    let my_pool:MyDbPool = MyDbPool::new(dsn, 1);
    let my_db_arc = Arc::new(DataBase::new("main", Arc::new(my_pool)));
    let db = my_db_arc.clone();

    let now = Local::now();
    handle_jcc_by_date(&now, &db);
    Result::Ok(1)
}

fn main() {
    let _ = elog::init();
    let _ = run();    
}
