extern crate scheduler;
use scheduler::*;

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
use service::get_jcc_draw_info;

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
    
    let my_db = my_db_arc.clone();
    thread::spawn(move || {
		let cj = CronJob::new(String::from("0,10,20,30,40,50 * *"));
	    let mut jt = JobTracker::new(cj);
	    jt.start(move |_, time| {
	        info!("{}", time);
	        //开售
	        let op = TermService.find_to_sale(&my_db);
	        if let Some(term) = op {
		        	info!("{}", term);
		        	let id = json_i64!(&term; "id");
		        	
		        	let mut cond = json!("{}");
		        	json_set!(&mut cond; "id"; id);
		        	
		        	let mut doc = json!("{}");
		        	let mut set_data = json!("{}");
		        	json_set!(&mut set_data; "status"; CONS.code_to_id("term_status", "sale").unwrap());
		        	json_set!(&mut doc; "$set"; set_data);
		        	
		        	let table = my_db.get_table("term").unwrap();
		        	let _ = table.update(&cond, &doc, &json!("{}"));
	        }
	        
	        let _ = EndJob.run(&my_db);
	        let _ = BonusJob.run(&my_db);
	        true
	    });
	});

    //算奖定时任务
    let my_db = my_db_arc.clone();
    thread::spawn(move || {
		let cj = CronJob::new(String::from("0,10,20,30,40,50 * *"));
	    let mut jt = JobTracker::new(cj);
	    jt.start(move |_, time| {
	        let _ = DrawJob.run(&my_db);
	        true
	    });
	});
    
    //定时触发奖期查询请求，请求存在redis中，并不真在这儿发送
    let my_db = my_db_arc.clone();
    thread::spawn(move || {
		let cj = CronJob::new(String::from("0 * *"));
		//let cj = CronJob::new(String::from("0,30 * *"));
	    let mut jt = JobTracker::new(cj);
	    jt.start(move |_, time| {
            info!("the cmd request.....");
            let mut msg = json!("{}");
            json_set!(&mut msg; "cmd"; "T04");
            let mut body = json!("{}");
            json_set!(&mut body; "game_id"; 201);
            json_set!(&mut body; "type"; 0);
            json_set!(&mut msg; "body"; body);
            let _ = CacheService.send_query_msg(&msg, &my_db);

            let mut msg = json!("{}");
            json_set!(&mut msg; "cmd"; "T04");
            let mut body = json!("{}");
            json_set!(&mut body; "game_id"; 301);
            json_set!(&mut body; "type"; 0);
            json_set!(&mut msg; "body"; body);
            let _ = CacheService.send_query_msg(&msg, &my_db);

	        true
	    });
	});

    //抓取竞彩开奖号码
    let my_db = my_db_arc.clone();
    thread::spawn(move || {
		let cj = CronJob::new(String::from("0 0,20,40 *"));
		//let cj = CronJob::new(String::from("0 * *"));
	    let mut jt = JobTracker::new(cj);
	    jt.start(move |_, time| {
            get_jcc_draw_info_from_sporttery(&my_db);
            get_jcl_draw_info_from_sporttery(&my_db);
   	        true
	    });
	});
    
    loop {
		thread::sleep(std::time::Duration::new(60, 0));
		info!("main thread beat..");
    }
}

fn get_jcc_draw_info_from_sporttery(db:&DataBase<MyDbPool>) {
    let now = Local::now();
    handle_jcc_by_date(&now, &db);
    let last_day = now - Duration::days(1);
    handle_jcc_by_date(&last_day, &db);
}

fn get_jcl_draw_info_from_sporttery(db:&DataBase<MyDbPool>) {
    let now = Local::now();
    handle_jcl_by_date(&now, &db);
    let last_day = now - Duration::days(1);
    handle_jcl_by_date(&last_day, &db);
}

fn update_jcc_draw_info_by_date(date:&DateTime<Local>, db:&DataBase<MyDbPool>) {
    let rst = get_jcc_draw_info(&date);
    if let Err(msg) = rst {
        println!("Err:{}.", msg); 
        return;
    }
    let map = rst.unwrap();
    for (key, value) in map {
        update_jcc_draw_info(key, &value, db);
    }
}

fn update_jcc_draw_info(code:i64, draw_number:&str, db:&DataBase<MyDbPool>) {
    let table = db.get_table("term").unwrap();

    let mut cond = json!("{}");
    json_set!(&mut cond; "game_id"; 201);
    json_set!(&mut cond; "code"; code);
    let src_status = CONS.code_to_id("term_status", "end").unwrap();
    json_set!(&mut cond; "status"; src_status);
    json_set!(&mut cond; "draw_number"; "");

    let mut doc = json!("{}");
    let mut set_data = json!("{}");
    json_set!(&mut set_data; "draw_number"; draw_number);
    json_set!(&mut doc; "$set"; set_data);

    let mut inc_data = json();
    json_set!(&mut inc_data; "version"; 1);
    json_set!(&mut doc; "$inc"; inc_data);

    let op = json!("{}");
    let _ = table.update(&cond, &doc, &op);
}





