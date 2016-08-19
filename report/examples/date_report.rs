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
    
    let date = cfg_i64!("report_date");
    let db = my_db_arc.clone();
    let _ = run_sale(date, &db);
}

























