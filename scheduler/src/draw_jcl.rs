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
use dc::stream;

extern crate easydb;
use easydb::DbPool;

extern crate easy_config;
use easy_config::CFG;

fn check(term_code:i64, draw_number_str:&str, draw_number:&Vec<i32>, mut ticket:Json, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
    let id = {
        let node = ticket.find("id").unwrap();
        node.as_i64().unwrap()
    };
    let multiple = {
        let node = ticket.find("multiple").unwrap(); 
        node.as_i64().unwrap()
    };
    let rst = TicketService.jc_check(term_code, draw_number_str, &ticket, db);
    if let Ok(new_list) = rst {
        let mut ticket_obj = ticket.as_object_mut().unwrap();
        ticket_obj.insert("draw_code_list".to_string(), new_list.to_json()); 
    }
    //从票据中拿取draw_code_list
    let draw_code_list = {
        let node = ticket.find("draw_code_list").unwrap();
        let node_str = node.as_string().unwrap();
        node_str.to_string()
    };
    //单场校验draw_code_list的长度
    if draw_code_list.len() == 0 {
        return Result::Ok(1);
    }
    let term_code_list = {
        let node = ticket.find("term_code_list").unwrap();
        let node_str = node.as_string().unwrap();
        node_str.to_string()
    };
    let term_code_array:Vec<&str> = term_code_list.split(";").collect();
    let draw_code_array:Vec<&str> = draw_code_list.split(";").collect();
    //当所有场次已经开奖时，进行算奖
    if term_code_array.len() != draw_code_array.len() {
        return Result::Ok(1);
    }
	let gl_map = BTreeMap::<i64, Json>::new();
    let draw_rst = DF.draw(&ticket, &draw_number, &gl_map);
    if draw_rst.is_err() {
        return Result::Ok(1);
    }
    let bonus_info = draw_rst.unwrap();
    let _ = TicketService.draw(id, multiple, &bonus_info, 301, "",  db);
    return Result::Ok(1);
}

///竞彩篮球算奖
pub fn draw_jcl(term:&Json) -> Result<i32, i32> {
	let game_id = json_i64!(term; "game_id");
	let term_code = json_i64!(term; "code");
	let draw_number_str = json_str!(term; "draw_number");
    let draw_number_string = draw_number_str.to_string();
    let draw_number = Vec::<i32>::new();
	
	let ticket_status = CONS.code_to_id("ticket_status", "print_success").unwrap();
	let sql = format!("select id, game_id, term_code_list, draw_code_list, play_type, bet_type, multiple, number, print_number from ticket where game_id={} and status={}", game_id, ticket_status);
	info!("{}", sql);

    let dsn = cfg_str!("db", "dsn");
    let my_pool:MyDbPool = MyDbPool::new(dsn, 2);
    let my_db_arc = Arc::new(DataBase::new("main", Arc::new(my_pool)));

    let my_db = my_db_arc.clone();
    let conn_rst = my_db.get_connection();
    let conn = conn_rst.unwrap();

    stream(conn, &sql, move |ticket| {
        info!("{}", ticket);
        let _ = check(term_code, &draw_number_string, &draw_number, ticket, &my_db);
        true
    }); 
	
	Result::Ok(1)
}


