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

extern crate sev_helper;
use sev_helper::AccountService;

use std::sync::{Arc};
use std::fs::File;
use std::io::Write;

fn main() {
    let _ = elog::init();
    
    info!(target:"main", "{}", CFG.get_data());
    let dsn = cfg_str!("db", "dsn");
    let my_pool:MyDbPool = MyDbPool::new(dsn, 1);
    let my_db_arc = Arc::new(DataBase::new("main", Arc::new(my_pool)));

    let my_db = my_db_arc.clone();
    let conn_rst = my_db.get_connection();
    let conn = conn_rst.unwrap();

    let sql = format!("select id, balance, client_balance from account where id in (select id from terminal where id in (144, 142, 135, 136, 137, 138))");
    stream(conn, &sql, move |data| {
        let client_balance = json_i64!(&data; "client_balance");
        let balance = json_i64!(&data; "balance");
        let amount = client_balance - balance;
        let id = json_i64!(&data; "id");
        let order_id = "2016_08_01_reset";
        let _ = AccountService.handle(id, order_id, 
			CONS.code_to_id("moneylog_type", "reset").unwrap(), 
			amount, &my_db
		);
        true
    }); 
}
