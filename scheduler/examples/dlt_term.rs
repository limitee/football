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

    let mut gap_map = BTreeMap::<i32, i32>::new();
    gap_map.insert(0, 3);   //0-0
    gap_map.insert(1, 2);   //3-6
    gap_map.insert(2, 2);   //6-1

    let game_id = 200;
    let gap_key = 1;
    let start_term = 2016088;
    let start = Local.ymd(2016, 07, 27).and_hms_milli(20, 15, 0, 0);

    let count = 10;
    let mut cur_start_term = 2016098;
    let op = json!("{}");
    let status = CONS.code_to_id("term_status", "init").unwrap();
    for i in 0..count {
        let term_gap = cur_start_term - start_term;
        let week_count = term_gap/3 as i32;
        let remain_type = term_gap%3;
        let remain_days = {
            match remain_type {
                1 => {
                    3
                },
                2 => {
                    5
                },
                _ => {
                    0
                }
            }
        };
        let mut cur_start = start + Duration::days(1)*(week_count*7 + remain_days);
        let mut cur_end = cur_start + Duration::days(1)*(gap_map.get(&remain_type).unwrap() + 0);

        let mut term = json!("{}");
        json_set!(&mut term; "game_id"; 200);
        json_set!(&mut term; "code"; cur_start_term);
        json_set!(&mut term; "sale_time"; cur_start.timestamp());
        json_set!(&mut term; "end_time"; cur_end.timestamp() - 30*60);
        json_set!(&mut term; "status"; status);
        let _ = table.save(&term, &op);
        info!("{}", term);

        cur_start_term += 1;
    }
    Result::Ok(1)
}

fn main() {
    let _ = elog::init();
    let _ = run();    
}
