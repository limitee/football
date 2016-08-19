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
    //let game_id = json_i64!(ticket; "game_id");
    let out_id = json_str!(ticket; "out_id");
    let create_time = json_i64!(ticket; "create_time");
    let date_time = Local.timestamp(create_time, 0);
    let date_time_string = date_time.format("%Y-%m-%d %H:%M:%S").to_string();
    let amount = json_i64!(ticket; "amount");
    let bonus_after_tax = json_i64!(ticket; "bonus_after_tax");
    let rst_string = format!("{}\t{}\t{}\t{}\r\n", out_id, amount, bonus_after_tax, date_time_string);
    rst_string
}

fn main() {
    let _ = elog::init();
    
    info!(target:"main", "{}", CFG.get_data());
    let dsn = cfg_str!("db", "dsn");
    let my_pool:MyDbPool = MyDbPool::new(dsn, 1);
    let my_db_arc = Arc::new(DataBase::new("main", Arc::new(my_pool)));

    let mut f = File::create("/data/workspace/export/ticket.txt").unwrap();
    //let head = "彩种编号\t外部id\t创建时间\t金额(分)\t税后奖金(分)\r\n".to_string();
    let head = "外部id\t金额(分)\t税后奖金(分)\t创建时间\r\n".to_string();
    f.write_all(&head.into_bytes());

    let my_db = my_db_arc.clone();
    let conn_rst = my_db.get_connection();
    let conn = conn_rst.unwrap();
    let start = Local.ymd(2016, 07, 01).and_hms_milli(0, 0, 0, 0);
    let end = Local.ymd(2016, 08, 01).and_hms_milli(0, 0, 0, 0);
    let start_stamp = start.timestamp();
    let end_stamp = end.timestamp();

    let sql = format!("select out_id, create_time, amount, bonus_after_tax from ticket where (game_id=201 or game_id=202) and create_time >= {} and create_time < {} and status!=50 and status!=10 order by create_time", start_stamp, end_stamp);
    stream(conn, &sql, move |ticket| {
        let ticket_string = get_ticket_string(&ticket);
        let byte_array = ticket_string.into_bytes();
        f.write_all(&byte_array);
        true
    }); 
}
