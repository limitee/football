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

fn get_rate_map(db:&DataBase<MyDbPool>) -> Result<BTreeMap<String, i64>, i32> {
    let cond = json!("{}");
    let doc = json!("{}");
    let op = json!("{}");
    let table = db.get_table("terminal_game").unwrap();
    let rst = table.find(&cond, &doc, &op);
    let mut rst_data = try!(rst);
    let mut rst_data_obj = rst_data.as_object_mut().unwrap();
    let data = rst_data_obj.remove("data").unwrap();
    let array = data.as_array().unwrap();
    
    let mut map = BTreeMap::<String, i64>::new();
    for set in array {
        info!("{}", set);
    }
    Result::Ok(map)
}

fn run() -> Result<i32, i32> {
    info!(target:"main", "{}", CFG.get_data());
    let dsn = cfg_str!("db", "dsn");
    let my_pool:MyDbPool = MyDbPool::new(dsn, 1);
    let my_db_arc = Arc::new(DataBase::new("main", Arc::new(my_pool)));

    let my_db = my_db_arc.clone();
    let rate_map = get_rate_map(&my_db);

    Result::Ok(1)
}

fn main() {
    let _ = elog::init();
    let _ = run();    
}
