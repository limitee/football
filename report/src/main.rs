extern crate report;
use report::*;

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
use util::*;

extern crate chrono;
use chrono::*;

#[macro_use]
extern crate easy_config;
use easy_config::CFG;

extern crate dc;
use dc::MyDbPool;
use dc::DataBase;

extern crate sev_helper;
use sev_helper::TermService;
use sev_helper::CacheService;

extern crate service;

extern crate cronjob;
use cronjob::*;

extern crate cons;
use cons::CONS;

use std::thread;

fn main() {
	let _ = elog::init();
    
    info!(target:"main", "{}", CFG.get_data());
    let dsn = cfg_str!("db", "dsn");
    let my_pool:MyDbPool = MyDbPool::new(dsn, cfg_i64!("db", "conn_limit") as u32);
    let my_db_arc = Arc::new(DataBase::new("main", Arc::new(my_pool)));
    
    //每个小时的40分，更新当天兑奖的数据
    let db = my_db_arc.clone();
    thread::spawn(move || {
		let cj = CronJob::new(String::from("0 42 *"));
	    let mut jt = JobTracker::new(cj);
	    jt.start(move |_, time| {
            let date_string = time.format("%Y%m%d").to_string();
            let date:i64 = date_string.parse().unwrap();
            let _ = run_bonus(date, &db);
            let _ = run_bonus_err(date, &db);
   	        true
	    });
	});
    
    loop {
		thread::sleep(std::time::Duration::new(60, 0));
		info!("main thread beat..");
    }
}

































