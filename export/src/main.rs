extern crate export; 

use std::io::prelude::*;
use std::net::TcpStream;
use std::io::Error;
use std::io::Cursor;

use std::sync::{Arc};

extern crate byteorder;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::str::FromStr;

#[macro_use]
extern crate log;
extern crate elog;

extern crate util;
use self::util::DigestUtil;

extern crate chrono;
use chrono::*;

#[macro_use]
extern crate easy_config;
use easy_config::CFG;

extern crate dc;
use dc::MyDbPool;
use dc::DataBase;
use dc::stream;

extern crate sev_helper;
use sev_helper::TermService;
use sev_helper::CacheService;

extern crate service;

extern crate cronjob;
use cronjob::*;

extern crate cons;
use cons::CONS;

use std::thread;
use std::time::{SystemTime};

fn main() {
	let _ = elog::init();
    
    info!(target:"main", "{}", CFG.get_data());
    let dsn = cfg_str!("db", "dsn");
    let my_pool:MyDbPool = MyDbPool::new(dsn, cfg_i64!("db", "conn_limit") as u32);
    let my_db_arc = Arc::new(DataBase::new("main", Arc::new(my_pool)));
    
    let db = my_db_arc.clone();
    let rst = run(&db);
    if let Ok(flag) = rst {
        info!("处理成功:{}", flag); 
    }
    if let Err(flag) = rst {
        info!("处理失败:{}", flag); 
    }
}

fn run(db:&DataBase<MyDbPool>) -> Result<i32, i32> {
    let table_name = "ticket";
    let mut year = 2016;
    let month = 6;
    let next_month;
    if month == 12 {
        year += 1;
        next_month = 1;
    } else {
        next_month = month + 1;
    }
    let month_str;
    if month < 10 {
        month_str = format!("0{}", month);
    } else {
        month_str = month.to_string();
    }
    let target_table_name = format!("{}_{}{}", table_name, year, month_str);

    //删除已经存在月表
    /*
    let del_sql = format!("drop table if exists {}", target_table_name);
    info!("{}", del_sql);
    let del_rst_data = try!(db.execute(&del_sql));
    info!("del rst: {}", del_rst_data);
    */

    //导出数据
    let start_time = Local.ymd(year, month, 01).and_hms_milli(0, 0, 0, 0);
    info!("start_time: {}.", start_time);
    let end_time = Local.ymd(year, next_month, 01).and_hms_milli(0, 0, 0, 0);
    info!("end_time: {}.", end_time);
    let start_timestamp = start_time.timestamp();
    let end_timestamp = end_time.timestamp();
    let condition = format!("where create_time < {} and create_time >= {}", end_timestamp, start_timestamp);
    let sql = format!("select * into {} from {} {}", target_table_name, table_name, condition);
    info!("{}", sql);
    let start_time = SystemTime::now();
    let move_rst_data = try!(db.execute(&sql));
    info!("move rst: {}", move_rst_data);
    let used_time = start_time.elapsed().unwrap();
    info!("used {} seconds.", used_time.as_secs());

    //删除原始表数据
    let del_sql = format!("delete from {} {}", table_name, condition);
    info!("{}", del_sql);
    let del_rst_data = try!(db.execute(&del_sql));
    info!("del rst: {}", del_rst_data);

    //创建索引
    if table_name == "moneylog" {
        let sql = format!("CREATE INDEX {}_create_time ON {} (create_time)", target_table_name, target_table_name);
        let _ = try!(db.execute(&sql));
        let sql = format!("CREATE INDEX {}_customer_id ON {} (customer_id)", target_table_name, target_table_name);
        let _ = try!(db.execute(&sql));
        let sql = format!("CREATE UNIQUE INDEX {}_id ON {} (id)", target_table_name, target_table_name);
        let _ = try!(db.execute(&sql));
    }
    
    if table_name == "ticket" {
        let sql = format!("CREATE INDEX {}_create_time ON {} (create_time)", target_table_name, target_table_name);
        let _ = try!(db.execute(&sql));
        let sql = format!("CREATE INDEX {}_print_time ON {} (print_time)", target_table_name, target_table_name);
        let _ = try!(db.execute(&sql));
        let sql = format!("CREATE INDEX {}_status ON {} (status)", target_table_name, target_table_name);
        let _ = try!(db.execute(&sql));

        let sql = format!("CREATE INDEX {}_customer_id_out_id ON {} (customer_id, out_id)", target_table_name, target_table_name);
        let _ = try!(db.execute(&sql));

        let sql = format!("CREATE UNIQUE INDEX {}_id ON {} (id)", target_table_name, target_table_name);
        let _ = try!(db.execute(&sql));
    }
    Result::Ok(1)
}































