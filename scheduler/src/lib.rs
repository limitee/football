use std::collections::BTreeMap;
use std::sync::{Arc};

#[macro_use]
extern crate easy_config;

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
use util::NumberUtil;

extern crate cons;
use cons::CONS;

extern crate game;
use game::DF;

extern crate sev_helper;
use sev_helper::TermService;
use sev_helper::TicketService;

extern crate dc;
use dc::MyDbPool;
use dc::DataBase;

extern crate easydb;
use easydb::DbPool;

extern crate hyper;
extern crate regex;
extern crate chrono;

extern crate encoding;
extern crate xml;

pub struct EndJob;

mod bonus;
pub use bonus::BonusJob;

mod draw_jcc;
use draw_jcc::draw_jcc;

mod draw_jcl;
use draw_jcl::draw_jcl;

mod get_jcl_draw_info;
pub use get_jcl_draw_info::handle_jcl_by_date;

mod jcc_draw_info;
pub use jcc_draw_info::handle_jcc_by_date;

impl EndJob {
	
	pub fn run(&self, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
		//查看是否有需要停售的期次
        let op = TermService.find_to_end(db);
        if let Some(term) = op {
        		info!("{}", term);
        		//出票失败的进行退款
        		end_refund(&term, db);
        		
	        	let id = json_i64!(&term; "id");
	        	
	        	let mut cond = json!("{}");
	        	json_set!(&mut cond; "id"; id);
	        	
	        	let mut doc = json!("{}");
	        	let mut set_data = json!("{}");
	        	json_set!(&mut set_data; "status"; CONS.code_to_id("term_status", "end").unwrap());
	        	json_set!(&mut doc; "$set"; set_data);
	        	
	        	let table = db.get_table("term").unwrap();
	        	let _ = table.update(&cond, &doc, &json!("{}"));
        }
        Result::Ok(1)
	}
	
}

pub struct DrawJob;

impl DrawJob {
	
	pub fn run(&self, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
		//查看是否有需要停售的期次
        let op = TermService.find_to_draw(db);
        if let Some(term) = op {
	   		info!("{}", term);
            let game_id = json_i64!(&term; "game_id");
            if game_id == 301_i64 {
                draw_jcl(&term);
            } else if game_id == 201_i64 {
                draw_jcc(&term, db);

                let mut dc_term = term.clone();
                json_set!(&mut dc_term; "game_id"; 202);
	   		    draw(&dc_term, db);
            } else {
	   		    draw(&term, db);
            }
	   		
	       	let id = json_i64!(&term; "id");
	       	let mut cond = json!("{}");
	       	json_set!(&mut cond; "id"; id);
	       	let mut doc = json!("{}");
	       	let mut set_data = json!("{}");
	       	json_set!(&mut set_data; "status"; CONS.code_to_id("term_status", "drawed").unwrap());
	       	json_set!(&mut doc; "$set"; set_data);
	        	
	       	let table = db.get_table("term").unwrap();
	       	let _ = table.update(&cond, &doc, &json!("{}"));
        }
        Result::Ok(1)
	}
}

///销售结束，未出票的票据进行退款
fn end_refund(term:&Json, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
	let game_id = json_i64!(term; "game_id");
	let term_code = json_i64!(term; "code");
	
	let ticket_status = CONS.code_to_id("ticket_status", "printing").unwrap();
    let sql;
    //竞彩串关，需要term_code_list
    if game_id == 201_i64 || game_id == 301_i64 {     
	    sql = format!("select id, customer_id, game_id, term_code, term_code_list, amount from ticket where game_id={} and status={}", game_id, ticket_status);
    } else {
	    sql = format!("select id, customer_id, game_id, term_code, amount from ticket where game_id={} and term_code={} and status={}", game_id, term_code, ticket_status);
    }
	info!("{}", sql);
	
	let conn = try!(db.dc.get_connection());
	let rst = conn.query("BEGIN", &[]);

    //begin
    let rst = rst.and_then(|rows| {
        let json = db.dc.get_back_json(rows);
        println!("{}", json);
        Result::Ok(1)
    }).or_else(|err|{
        println!("{}", err);
        Result::Err(-1)
    });

    //cursor
    let rst = rst.and_then(|_| {
   		let cursor_sql = format!("DECLARE myportal CURSOR FOR {}", sql);
   		println!("{}", cursor_sql);
   		let rst = conn.query(&cursor_sql, &[]);
   		rst.and_then(|rows|{
            let json = db.dc.get_back_json(rows);
            println!("{}", json);
            Result::Ok(1)
        }).or_else(|err|{
            println!("{}", err);
            Result::Err(-1)
        })
    });

    let rst = rst.and_then(|_| {
        let fetch_sql = "FETCH NEXT in myportal";
        println!("{}", fetch_sql);

        let mut flag = 0;
        loop {
            let rst = conn.query(&fetch_sql, &[]);
            let _ = rst.and_then(|rows|{
                let json = db.dc.get_back_json(rows);
                let rows = json_i64!(&json; "rows");
                if rows < 1 {
                    flag = -2;
                } else {
                    //let f_back = f(json);
                    let ticket = json_path!(&json; "data", "0");
                    let id = json_i64!(ticket; "id");
                    let customer_id = json_i64!(ticket; "customer_id");
                    let amount = json_i64!(ticket; "amount");
                    if game_id == 201_i64 || game_id == 301_i64 {
                        let term_code_list = json_str!(ticket; "term_code_list");
                        let term_code_string = term_code.to_string();
                        if term_code_list.contains(term_code_string.as_str()) {
					        let _ = TicketService.refund(id, customer_id, amount, db);
                        }
                    } else {
					    let _ = TicketService.refund(id, customer_id, amount, db);
                    }
					
                    let f_back = true;
                    if !f_back {
                        flag = -2;
                    }
                }
                Result::Ok(flag)
            }).or_else(|err|{
                println!("{}", err);
                flag = -1;
                Result::Err(flag)
            });
            if flag < 0 {
                break;
            }
        }
        match flag {
            -1 => {
                Result::Err(-1)
            },
            _ => {
                Result::Ok(1)
            },
        }
    });

    //close the portal
    let rst = rst.and_then(|_|{
    		let close_sql = "CLOSE myportal";
        println!("{}", close_sql);
        let rst = conn.query(&close_sql, &[]);
        rst.and_then(|rows|{
            let json = db.dc.get_back_json(rows);
            println!("{}", json);
            Result::Ok(1)
        }).or_else(|err|{
            println!("{}", err);
            Result::Err(-1)
        })
    });

    //end the cursor
    let rst = rst.and_then(|_|{
    		let end_sql = "END";
        println!("{}", end_sql);
        let rst = conn.query(&end_sql, &[]);
        rst.and_then(|rows|{
            let json = db.dc.get_back_json(rows);
            println!("{}", json);
            Result::Ok(1)
        }).or_else(|err|{
            println!("{}", err);
            Result::Err(-1)
        })		
    	});
    rst
}

///算奖
fn draw(term:&Json, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
	let game_id = json_i64!(term; "game_id");
	let term_code = json_i64!(term; "code");
	let draw_number_str = json_str!(term; "draw_number");
    let draw_number;
    if draw_number_str == "*" {
        draw_number = Vec::<i32>::new();
    } else {
	    draw_number = NumberUtil::to_int_array(draw_number_str);
    }
	let mut gl_map = BTreeMap::<i64, Json>::new();
	
	//获得游戏奖级详情
	let table = db.get_table("game_level").unwrap();
	let mut cond = json!("{}");
	json_set!(&mut cond; "game_id"; game_id);
	json_set!(&mut cond; "term_code"; term_code);
	let doc = json!("{}");
	let op = json!("{}");
	let rst = table.find(&cond, &doc, &op);
	let mut db_data = try!(rst);
	let db_list_json = db_data.as_object_mut().unwrap().remove("data").unwrap();
	let db_list_array = db_list_json.as_array().unwrap();
	for set in db_list_array {
		let lev = json_i64!(set; "lev");
		gl_map.insert(lev as i64, set.clone());
	}
	
	let ticket_status = CONS.code_to_id("ticket_status", "print_success").unwrap();
	let sql = format!("select id, game_id, play_type, bet_type, multiple, number, print_number from ticket where game_id={} and term_code={} and status={}", game_id, term_code, ticket_status);
	info!("{}", sql);
	
	let conn = try!(db.dc.get_connection());
	let rst = conn.query("BEGIN", &[]);

    //begin
    let rst = rst.and_then(|rows| {
        let json = db.dc.get_back_json(rows);
        println!("{}", json);
        Result::Ok(1)
    }).or_else(|err|{
        println!("{}", err);
        Result::Err(-1)
    });

    //cursor
    let rst = rst.and_then(|_| {
   		let cursor_sql = format!("DECLARE myportal CURSOR FOR {}", sql);
   		println!("{}", cursor_sql);
   		let rst = conn.query(&cursor_sql, &[]);
   		rst.and_then(|rows|{
           let json = db.dc.get_back_json(rows);
           println!("{}", json);
           Result::Ok(1)
        }).or_else(|err|{
            println!("{}", err);
            Result::Err(-1)
        })
    });

    let rst = rst.and_then(|_| {
        let fetch_sql = "FETCH NEXT in myportal";
        println!("{}", fetch_sql);

        let mut flag = 0;
        loop {
            let rst = conn.query(&fetch_sql, &[]);
            let _ = rst.and_then(|rows|{
                let json = db.dc.get_back_json(rows);
                let rows = json_i64!(&json; "rows");
                if rows < 1 {
                    flag = -2;
                } else {
                    //let f_back = f(json);
                    let ticket = json_path!(&json; "data", "0");
                    let id = json_i64!(ticket; "id");
                    let game_id = json_i64!(ticket; "game_id");
					let multiple = json_i64!(ticket; "multiple");
					let rst = DF.draw(ticket, &draw_number, &gl_map);
					let rst = rst.and_then(|json| {
						info!("{}", json);
						//let table = db.get_table("term").unwrap();
						let _ = TicketService.draw(id, multiple, &json, game_id, draw_number_str, db);
						Result::Ok(1)
					});
                    let f_back = true;
                    if !f_back {
                        flag = -2;
                    }
                }
                Result::Ok(flag)
            }).or_else(|err|{
                println!("{}", err);
                flag = -1;
                Result::Err(flag)
            });
            if flag < 0 {
                break;
            }
        }
        match flag {
            -1 => {
                Result::Err(-1)
            },
            _ => {
                Result::Ok(1)
            },
        }
    });

    //close the portal
    let rst = rst.and_then(|_|{
    		let close_sql = "CLOSE myportal";
        println!("{}", close_sql);
        let rst = conn.query(&close_sql, &[]);
        rst.and_then(|rows|{
            let json = db.dc.get_back_json(rows);
            println!("{}", json);
            Result::Ok(1)
        }).or_else(|err|{
            println!("{}", err);
            Result::Err(-1)
        })
    });

    //end the cursor
    let rst = rst.and_then(|_|{
    		let end_sql = "END";
        println!("{}", end_sql);
        let rst = conn.query(&end_sql, &[]);
        rst.and_then(|rows|{
            let json = db.dc.get_back_json(rows);
            println!("{}", json);
            Result::Ok(1)
        }).or_else(|err|{
            println!("{}", err);
            Result::Err(-1)
        })		
    	});
	Result::Ok(1)
}


