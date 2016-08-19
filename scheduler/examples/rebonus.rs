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

fn get_ticket_string(ticket:&Json) -> String {
    let game_id = json_i64!(ticket; "game_id");
    let out_id = json_str!(ticket; "out_id");
    let create_time = json_i64!(ticket; "create_time");
    let date_time = Local.timestamp(create_time, 0);
    let date_time_string = date_time.format("%Y-%m-%d %H:%M:%S").to_string();
    let amount = json_i64!(ticket; "amount");
    let bonus_after_tax = json_i64!(ticket; "bonus_after_tax");
    let rst_string = format!("{}\t{}\t{}\t{}\t{}\r\n", game_id, out_id, date_time_string,
        amount, bonus_after_tax); 
    rst_string
}

fn main() {
    let _ = elog::init();
    
    info!(target:"main", "{}", CFG.get_data());
    let dsn = cfg_str!("db", "dsn");
    let my_pool:MyDbPool = MyDbPool::new(dsn, 1);
    let my_db_arc = Arc::new(DataBase::new("main", Arc::new(my_pool)));

    let my_db = my_db_arc.clone();
    let conn_rst = my_db.get_connection();
    let conn = conn_rst.unwrap();
    let start = Local.ymd(2016, 05, 01).and_hms_milli(0, 0, 0, 0);
    let end = Local.ymd(2016, 07, 01).and_hms_milli(0, 0, 0, 0);
    let start_stamp = start.timestamp();
    let end_stamp = end.timestamp();

    let sql = format!("select id, seq, bonus, bonus_after_tax, terminal_id from ticket where status=65 and terminal_id in (select id from customer where type=500 and province=3)");
    stream(conn, &sql, move |ticket| {
        info!("{}", ticket);
        true
    }); 
}
