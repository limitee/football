use std::collections::BTreeMap;
use std::sync::{Arc};

extern crate easy_util;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::str::FromStr;

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

pub struct BonusJob;

impl BonusJob {
	
	pub fn run(&self, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
        let op = TermService.find_to_fund(db);
        if let Some(term) = op {
            info!("{}", term);
            let game_id = json_i64!(&term; "game_id");
            bonus(&term, db);
            if game_id == 201_i64 {
                let mut term = term.clone();
                json_set!(&mut term; "game_id"; 202);
                bonus(&term, db);
            }
            let id = json_i64!(&term; "id");
	        	
	       	let mut cond = json!("{}");
	       	json_set!(&mut cond; "id"; id);
	        	
	       	let mut doc = json!("{}");
	       	let mut set_data = json!("{}");
	       	json_set!(&mut set_data; "status"; CONS.code_to_id("term_status", "funded").unwrap());
	       	json_set!(&mut doc; "$set"; set_data);
	        	
	       	let table = db.get_table("term").unwrap();
	       	let _ = table.update(&cond, &doc, &json!("{}"));
        }
		//查看是否有需要停售的期次
        Result::Ok(1)
	}
}

///返奖
fn bonus(term:&Json, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
	let game_id = json_i64!(term; "game_id");
	let term_code = json_i64!(term; "code");
	let ticket_status = CONS.code_to_id("ticket_status", "hit").unwrap();
    let sql;
    match game_id {
        201_i64 | 301_i64 => {
	        sql = format!("select id, customer_id, terminal_id, game_id, play_type, bet_type, multiple, bonus, bonus_after_tax, seq from ticket where game_id={} and status={}", game_id, ticket_status);
        },
        _ => {
	        sql = format!("select id, customer_id, terminal_id, game_id, play_type, bet_type, multiple, bonus, bonus_after_tax, seq from ticket where game_id={} and term_code={} and status={}", game_id, term_code, ticket_status);
        },
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
                    //
                    let ticket = json_path!(&json; "data", "0");
                    let _ = TicketService.bonus(ticket, db);

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


