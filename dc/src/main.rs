extern crate dc;
use dc::{MyDbPool, DataBase, MyRedisPool};

use std::sync::Arc;

#[macro_use]
extern crate log;
extern crate elog;

#[macro_use]
extern crate easy_config;
use easy_config::CFG;

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;
use std::str::FromStr;

extern crate redis;
use redis::Commands;
use redis::RedisResult;

fn main() {
	let _ = elog::init();
    info!(target:"main", "{}", CFG.get_data());
    let dsn = cfg_str!("db", "dsn");
    let my_pool:MyDbPool = MyDbPool::new(dsn, cfg_i64!("db", "conn_limit") as u32);
    let my_db = DataBase::new("main", Arc::new(my_pool));
    let table = my_db.get_table("ticket").unwrap();

	let mut or_data = Vec::<Json>::new();
    let mut data = json!("{}");
    json_set!(&mut data; "id"; 1);
    or_data.push(data);

    let mut data = json!("{}");
    json_set!(&mut data; "out_id"; "123");
    or_data.push(data);
        
    let mut cond = json!("{}");
    json_set!(&mut cond; "id"; 2);
    json_set!(&mut cond; "$or"; or_data);
    info!("{}", cond);
		
	let doc = json!("{}");
	let op = json!("{}");
		
    let mut back_body = json!("{}");
	let rst = table.find_one(&cond, &doc, &op);

	/*
    let rst = my_db.execute("select * from ticket");
    let _ = rst.and_then(|json| {
        println!("{}", json);
        Result::Ok(())
    });
    
    let _ = my_db.stream("select * from ticket", |json| {
        println!("{}", json);
        true
    });
    */
	
    /*
    let conn = my_db.cache.get_conn().unwrap().lock().unwrap();
    let rst:RedisResult<()> = conn.set("test_001", "nothing");
    let rst = rst.and_then(|_|{
    		let rst:RedisResult<(String)> = conn.get("test_001");
    		rst
    });
    let rst = rst.and_then(|value|{
    		info!("{}", value);
    		Result::Ok(1)
    });
    
    let rst:RedisResult<()> = conn.lpush("test_list", "001");
    let rst = rst.and_then(|_|{
    		let rst:RedisResult<(String)> = conn.rpop("test_list");
    		rst
    });
    let rst = rst.and_then(|value|{
    		info!("{}", value);
    		Result::Ok(1)
    });
    */
}
