use std::sync::Arc;

#[macro_use]
extern crate log;
extern crate elog;

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;
use std::str::FromStr;

extern crate server;
use server::*;

extern crate dc;
use dc::MyDbPool;
use dc::DataBase;

#[macro_use]
extern crate easy_config;
use easy_config::CFG;

extern crate service;
use service::ApiFactory;

extern crate sev_helper;
use sev_helper::CacheService;
use sev_helper::TerminalService;

extern crate cons;
use cons::CONS;

fn main() {
	let _ = elog::init();
	info!(target:"main", "{}", CFG.get_data());
    let dsn = cfg_str!("db", "dsn");
    let my_pool:MyDbPool = MyDbPool::new(dsn, cfg_i64!("db", "conn_limit") as u32);
    let my_db = DataBase::new("main", Arc::new(my_pool));

    //检查出票间隔参数
    /*
    let gap_rst = CacheService.get_print_gap(&my_db);
    if gap_rst.is_err() {
        let _ = CacheService.set_print_gap(16, &my_db);
    }
    */

    //系统重置为出票模式
    let _ = CacheService.set_terminal_mode(CONS.code_to_id("sys_mode", "print").unwrap() as i64, &my_db);    

    //默认为出票错误，转发到其它的终端
    //let _ = CacheService.set_resend_ticket_when_err(true, &my_db);

    //所有终端机设置为离线状态
    let _ = TerminalService.all_offline(&my_db);

	let api = ApiFactory::new();
	let mut sev = Server::new(cfg_str!("server", "url"), api, my_db);
	sev.start();
}
