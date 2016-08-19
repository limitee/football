extern crate picker;

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
    
    let db = my_db_arc.clone();

    loop {
        let print_list_len_rst = CacheService.get_print_list_len(&db);         
        if print_list_len_rst.is_err() {
        	thread::sleep(std::time::Duration::from_millis(500));
            continue;
        }
        let len = print_list_len_rst.unwrap();
        info!("the print_list len is {}.", len);
        if len > 20 {
        	thread::sleep(std::time::Duration::from_millis(500));
            continue;
        }
        let now = Local::now();
        let timestamp = now.timestamp();
        let sql = "select id,game_id,play_type,bet_type,icount,multiple,number,amount,end_time,version from print_pool where status=0 order by end_time, amount desc limit 20";
        let rst = db.execute(sql);
        if rst.is_err() {
            thread::sleep(std::time::Duration::from_millis(500));
            continue;
        }
        let mut rst_data = rst.unwrap();
        let mut rst_obj = rst_data.as_object_mut().unwrap();
        let mut data = rst_obj.remove("data").unwrap();
        let mut list = data.as_array_mut().unwrap(); 
        loop {
            let ticket_op = list.pop();
            if ticket_op.is_none() {
                break;
            }
            let ticket = ticket_op.unwrap();
            let id = {
                let node = ticket.find("id").unwrap(); 
                node.as_i64().unwrap()
            };
            let version = {
                let node = ticket.find("version").unwrap(); 
                node.as_i64().unwrap()
            };
            let del_sql = format!("update print_pool set status=1,version=version+1,send_time={} where id={} and status=0 and version={} returning id", timestamp, id, version);
            let mut up_rst = db.execute(&del_sql);
            let mut up_data = up_rst.unwrap();
            let mut up_obj = up_data.as_object_mut().unwrap();
            let rows = {
                let node = up_obj.remove("rows").unwrap(); 
                node.as_i64().unwrap()
            };
            if rows > 0 {
                
                print(ticket, &db);
            }
        }
       	thread::sleep(std::time::Duration::from_millis(200));
    }
}


fn print(mut ticket:Json, db:&DataBase<MyDbPool>) {
    {
        let mut obj = ticket.as_object_mut().unwrap(); 

        //转换play_type的类型
        let node = obj.remove("play_type").unwrap();
        let play_type = node.as_i64().unwrap();
        let play_type_str;
		if play_type < 10 {
			play_type_str = format!("0{}", play_type);
		} else {
			play_type_str = format!("{}", play_type);
		}
        obj.insert("play_type".to_string(), play_type_str.to_json());

        //转换bet_type的类型
        let node = obj.remove("bet_type").unwrap();
        let bet_type = node.as_i64().unwrap();
        let bet_type_str;
		if bet_type < 10 {
			bet_type_str = format!("0{}", bet_type);
		} else {
			bet_type_str = format!("{}", bet_type);
		}
        obj.insert("bet_type".to_string(), bet_type_str.to_json());

        //设置try_count
        obj.insert("try_count".to_string(), 1.to_json());
        //删除version
        let _ = obj.remove("version");
    }
	let _ = CacheService.print(&ticket, -1, db);
}































