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

extern crate cons;
use cons::CONS;

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

fn run() -> Result<i32, i32> {
    info!(target:"main", "{}", CFG.get_data());
    let dsn = cfg_str!("db", "dsn");
    let my_pool:MyDbPool = MyDbPool::new(dsn, 1);
    let my_db_arc = Arc::new(DataBase::new("main", Arc::new(my_pool)));

    let my_db = my_db_arc.clone();
    let table = my_db.get_table("term").unwrap();

    let game_id = 203;
    let start_term = 2016202;
    let start = Local.ymd(2016, 07, 26).and_hms_milli(22, 0, 0, 0);
    let end = Local.ymd(2016, 07, 27).and_hms_milli(20, 0, 0, 0);
    let gap = Duration::days(1);

    let count = 10;

    let mut cur_start_term = 2016202;
    let term_gap = (cur_start_term - start_term) as i32;
    let mut cur_start = start + gap*term_gap;
    let mut cur_end = end + gap*term_gap;
    let status = CONS.code_to_id("term_status", "init").unwrap();

    let op = json!("{}");

    for i in 0..count {
        let mut term = json!("{}");
        json_set!(&mut term; "game_id"; 203);
        json_set!(&mut term; "code"; cur_start_term);
        json_set!(&mut term; "sale_time"; cur_start.timestamp());
        json_set!(&mut term; "end_time"; cur_end.timestamp());
        json_set!(&mut term; "status"; status);
        let _ = table.save(&term, &op);
        info!("{}", term);

        cur_start_term += 1;
        cur_start = cur_start + gap;
        cur_end = cur_end + gap;
    }

    Result::Ok(1)
}

fn main() {
    let _ = elog::init();
    let _ = run();    
}
