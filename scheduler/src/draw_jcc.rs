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

fn get_gl_value(draw_op:&str) -> Json {
    let mut rst = json!("{}");
    let array = NumberUtil::to_int_array(draw_op);
    //胜平负
    let all;
    if array[0] > array[1] {
        all = "3";
    } else if array[0] == array[1] {
        all = "1";
    } else {
        all = "0";
    }
    json_set!(&mut rst; "01"; all);
    //让球胜平负
    let value;
    if array[0] + array[2] > array[1] {
        value = "3";
    } else if array[0] + array[2] == array[1] {
        value = "1";
    } else {
        value = "0";
    }
    json_set!(&mut rst; "02"; value);
    //总进球数
    let mut value = (array[0] + array[1])/10;
    if value > 7 {
        value = 7;
    }
    json_set!(&mut rst; "03"; value.to_string());
    //比分
    let mut master;
    let mut guest;
    if array[0] > array[1] {
        master = array[0]/10;
        guest = array[1]/10;
        if master > 5 || guest > 2 {
            master = 9;
            guest = 0;
        }
    } else if array[0] == array[1] {
        master = array[0]/10;
        if master > 3 {
            master = 9;
        }
        guest = master;
    } else {
        master = array[0]/10;
        guest = array[1]/10;
        if master > 2 || guest > 5 {
            master = 0;
            guest = 9;
        }
    }
    let bf = format!("{}{}", master, guest);
    json_set!(&mut rst; "04"; bf);
    //半场
    let half;
    if array[3] > array[4] {
        half = "3";
    } else if array[3] == array[4] {
        half = "1";
    } else {
        half = "0";
    }
    let ha = format!("{}{}", half, all);
    json_set!(&mut rst; "05"; ha);
    rst
}

fn get_gl_cancel_value() -> Json {
    let mut rst = json!("{}");
    json_set!(&mut rst; "01"; "*");
    json_set!(&mut rst; "02"; "*");
    json_set!(&mut rst; "03"; "*");
    json_set!(&mut rst; "04"; "*");
    json_set!(&mut rst; "05"; "*");
    rst
}

fn get_gl_map(draw_array:&Vec<&str>) -> BTreeMap<i64, Json> {
    let mut gl_map = BTreeMap::<i64, Json>::new();
    for match_info in draw_array {
        let match_info_array:Vec<&str> = match_info.split(":").collect();
        let term_code = i64::from_str(match_info_array[0]).unwrap();
        let rst;
        if match_info_array[1] == "*" {
            rst = get_gl_cancel_value();
        } else {
            rst = get_gl_value(match_info_array[1]);
        }
        gl_map.insert(term_code, rst); 
    }
    gl_map
}

fn check(term_code:i64, draw_number_str:&str, draw_number:&Vec<i32>, ticket:&Json, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
    let id = json_i64!(ticket; "id");
    let multiple = json_i64!(ticket; "multiple");
    let rst = TicketService.jc_check(term_code, draw_number_str, ticket, db);
    //如果没有更新，则从票据中拿取
    let rst = rst.or_else(|_|{
        let draw_code_list = json_str!(ticket; "draw_code_list");
        Result::Ok(draw_code_list.to_string())
    });
    //判定是否需要算奖
    let rst = rst.and_then(|draw_code_list|{
        let term_code_list = json_str!(ticket; "term_code_list");
        let term_code_array:Vec<&str> = term_code_list.split(";").collect();
        let draw_code_array:Vec<&str> = draw_code_list.split(";").collect();
        //当所有场次已经开奖时，进行算奖
        if term_code_array.len() == draw_code_array.len() {
	        let mut gl_map = get_gl_map(&draw_code_array);
            DF.draw(ticket, &draw_number, &gl_map)
        } else {
            Result::Err(1)
        }
    });
    //更新票据状态
    let rst = rst.and_then(|json| {
        info!("{}", json);
        let _ = TicketService.draw(id, multiple, &json, 201, "",  db);
        Result::Ok(1)
    });
    rst
}

///竞彩算奖
pub fn draw_jcc(term:&Json, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
	let game_id = json_i64!(term; "game_id");
	let term_code = json_i64!(term; "code");
	let draw_number_str = json_str!(term; "draw_number");
    let draw_number = Vec::<i32>::new();
	
	let ticket_status = CONS.code_to_id("ticket_status", "print_success").unwrap();
	let sql = format!("select id, game_id, term_code_list, draw_code_list, play_type, bet_type, multiple, number, print_number from ticket where game_id={} and status={}", game_id, ticket_status);
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
                    info!("{}", ticket);
                    let _ = check(term_code, draw_number_str, &draw_number, ticket, db);
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


