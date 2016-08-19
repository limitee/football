extern crate util;
use util::*;

use super::super::dc::DataBase;
use super::super::dc::MyDbPool;
use super::super::cons::CONS;
use super::super::cons::ErrCode;

use std::collections::BTreeMap;
use std::io::Read;

extern crate rustc_serialize;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::str::FromStr;

extern crate regex;
use self::regex::Regex;

extern crate time;
extern crate chrono;
use chrono::*;

use super::super::inter::{DataApi};
use super::super::sv_util::{KeyHelper};

extern crate game;
use self::game::base::GF;

extern crate sev_helper;
use sev_helper::GameService;
use sev_helper::PrintService;
use sev_helper::TicketService;
use sev_helper::CacheService;

struct Helper;

impl Helper {
	
	fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
		let rst = {
			let user_type = json_str!(head; "userType");
	    		if user_type == "admin" {
	    			Result::Ok(1)
	    		} else {
	    			Result::Err(ErrCode::NotAllowed as i32)	
	    		}
		};
		let rst = rst.and_then(|_|{
			KeyHelper::from_cache(db, head)
		});
		if rst.is_ok() {
			KeyHelper::active(db, head);
		}
		rst
	}
}

//获得票据列表
pub struct ATI01;

impl DataApi for ATI01 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let cur_time_string;
		let cur_time = json_i64!(msg; "body", "cur_time");
        if cur_time > 0 {
            cur_time_string = format!("_{}", cur_time);
        } else {
            cur_time_string = "".to_string();
        }

        let table = db.get_table("ticket").expect("ticket table not exists.");
        let body = msg.find("body").unwrap();
        let cond = body.find("cond").unwrap();
        let cond_string = table.get_cond(cond, "", None);
        let op = body.find("op").unwrap();
        let op_string = table.get_option(op, None);

        let columns = "id,game_id,term_code,play_type,bet_type,amount,number,status,create_time,end_time,bonus_try_count";
        let query_sql = format!("select {} from ticket{} where {} {}", columns, cur_time_string, cond_string, op_string);
    	let mut back_json = try!(db.execute(&query_sql));

        let count_sql = format!("select count(id) from ticket{} where {}", cur_time_string, cond_string);
        let rst = try!(db.execute(&count_sql));
        let count = get_count(rst);
   		json_set!(&mut back_json;"count";count);

    	let game_list = GameService.get_game_list();
    	json_set!(&mut back_json;"game_list";game_list);
    	json_set!(&mut back_json;"ticket_status";CONS.get_json_obj("ticket_status"));

        json_set!(&mut back_json; "cur_year"; 2016);
       	json_set!(&mut back_json; "cur_month"; 6);

   		Result::Ok(back_json)
    }
}

//获得票据详情
pub struct ATI02;

impl DataApi for ATI02 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let cur_time_string;
		let cur_time = json_i64!(msg; "body", "cur_time");
        if cur_time > 0 {
            cur_time_string = format!("_{}", cur_time);
        } else {
            cur_time_string = "".to_string();
        }
		let id = json_i64!(msg; "body", "id");

        let sql = format!("select * from ticket{} where id={}", cur_time_string, id);
        let rst = try!(db.execute(&sql));
        let ticket = get_first_data(rst);

        let mut back_json = json();
        let game_list = GameService.get_game_list();
   		json_set!(&mut back_json;"game_list";game_list);
   		json_set!(&mut back_json;"ticket_status";CONS.get_json_obj("ticket_status"));
    		
   		json_set!(&mut back_json;"ticket";ticket);
   		Result::Ok(back_json)
    }
}

///重新出票
pub struct ATI03;

impl DataApi for ATI03 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let id = json_i64!(msg; "body", "id");
        let terminal_id = json_i64!(msg; "body", "terminal_id");
        PrintService.reprint(id, -1, db)
    }
}

///退款
pub struct ATI04;

impl DataApi for ATI04 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let id = json_i64!(msg; "body", "id");
        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; id);
        let table = db.get_table("ticket").unwrap();
        let rst = table.find_one(&cond, &json!("{}"), &json!("{}"));
        let ticket = try!(rst);
        let amount = json_i64!(&ticket; "amount");
        let customer_id = json_i64!(&ticket; "customer_id");
        let rst = TicketService.refund(id, customer_id, amount, db);
        rst
    }
}

///兑奖
pub struct ATI05;

impl DataApi for ATI05 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let body = msg.find("body").unwrap();
        let id = json_i64!(body; "id");
        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; id);
        let table = db.get_table("ticket").unwrap();
        let rst = table.find_one(&cond, &json!("{}"), &json!("{}"));
        let ticket = try!(rst);

        let terminal_id_node = body.find("terminal_id").unwrap();
        let terminal_id = terminal_id_node.as_i64().unwrap();

        let bonus = json_i64!(&ticket; "bonus");
        let amount = json_i64!(&ticket; "bonus_after_tax");
        let seq = json_str!(&ticket; "seq");

        //加入兑奖队列
        let mut bonus_ticket = json!("{}");  
		json_set!(&mut bonus_ticket; "id"; id);
		json_set!(&mut bonus_ticket; "bonus"; bonus);
		json_set!(&mut bonus_ticket; "bonus_after_tax"; amount);
		json_set!(&mut bonus_ticket; "seq"; seq);
        let _ = CacheService.bonus(terminal_id, &bonus_ticket, db);

        Result::Ok(json!("{}"))
    }
}

//出票错误
pub struct ATI06;

impl DataApi for ATI06 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let id = json_i64!(msg; "body", "id");
        let rst = TicketService.get_by_id(id, db);
        let ticket = try!(rst);
        PrintService.print_err(-1, &ticket, db)
    }
}

//兑奖成功
pub struct ATI07;

impl DataApi for ATI07 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let id = json_i64!(msg; "body", "id");

        let mut cond = json!("{}");
		json_set!(&mut cond; "id"; id);
		
		let mut doc = json!("{}");
		json_set!(&mut doc; "id"; 1);
		json_set!(&mut doc; "terminal_id"; 1);
		json_set!(&mut doc; "bonus"; 1);
		
		let op = json!("{}");
		let table = db.get_table("ticket").unwrap();
        let ticket_rst = table.find_one(&cond, &doc, &op);
        let ticket = try!(ticket_rst);

        let terminal_id = json_i64!(&ticket; "terminal_id");
        let bonus = json_i64!(&ticket; "bonus");
        let rst = TicketService.bonus_success(id, terminal_id, bonus, "fix", db);
        let _ = try!(rst);

        Result::Ok(json!("{}"))
    }
}

//设置成大奖票
pub struct ATI08;

impl DataApi for ATI08 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let id = json_i64!(msg; "body", "id");

        let mut cond = json!("{}");
		json_set!(&mut cond; "id"; id);
        let status = CONS.code_to_id("ticket_status", "bonus_err").unwrap();
		json_set!(&mut cond; "status"; status);
		
		let mut doc = json!("{}");
        let mut set_data = json!("{}");
        let target_status = CONS.code_to_id("ticket_status", "bonus_big").unwrap();
		json_set!(&mut set_data; "status"; target_status);
		json_set!(&mut doc; "$set"; set_data);
		
		let mut op = json!("{}");
        let mut ret_data = json!("{}");
        json_set!(&mut ret_data; "id"; 1); 
        json_set!(&mut op; "ret"; ret_data); 

        let table = db.get_table("ticket").unwrap();
        let rst = table.update(&cond, &doc, &op);
        let back_json = try!(rst);
        info!("{}", back_json);

        let rows = json_i64!(&back_json; "rows");
        if rows <= 0 {
            return Result::Err(ErrCode::DataExpired as i32);
        }
        return Result::Ok(json!("{}"));
    }
}

///重出所有的异常票
pub struct ATI09;

impl DataApi for ATI09 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let mut cond = json!("{}");
        let status = CONS.code_to_id("ticket_status", "print_err").unwrap();
		json_set!(&mut cond; "status"; status);
		
		let mut doc = json!("{}");
        json_set!(&mut doc; "id"; 1); 
		
		let op = json!("{}");

        let table = db.get_table("ticket").unwrap();
        let rst = table.find(&cond, &doc, &op);
        let rst_data = try!(rst);
        let data = rst_data.find("data").unwrap();
        let array = data.as_array().unwrap();
        for ticket in array {
            let id_node = ticket.find("id").unwrap();
            let id = id_node.as_i64().unwrap();
            PrintService.reprint(id, -1, db);
        }
        Result::Ok(json!("{}"))
    }
}

///重兑所有的兑奖异常票
pub struct ATI10;

impl DataApi for ATI10 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let mut cond = json!("{}");
        let status = CONS.code_to_id("ticket_status", "bonus_err").unwrap();
		json_set!(&mut cond; "status"; status);
        let mut lt_data = json!("{}");
        json_set!(&mut lt_data; "$lt"; 3);
        json_set!(&mut cond; "bonus_try_count"; lt_data);
		
		let mut doc = json!("{}");
        json_set!(&mut doc; "id"; 1); 
		
		let op = json!("{}");

        let table = db.get_table("ticket").unwrap();
        let rst = table.find(&cond, &doc, &op);
        let rst_data = try!(rst);
        let data = rst_data.find("data").unwrap();
        let array = data.as_array().unwrap();
        for ticket in array {
            let id_node = ticket.find("id").unwrap();
            let id = id_node.as_i64().unwrap();
            TicketService.rebonus(id, db);
        }
        Result::Ok(json!("{}"))
    }
}

///一键退款，根据游戏id和期次
pub struct ATI11;

impl DataApi for ATI11 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let body = msg.find("body").unwrap();
        let game_id = {
            let node = body.find("game_id").unwrap();
            node.as_i64().unwrap()
        };
        let term_code = {
            let node = body.find("term_code").unwrap();
            node.as_i64().unwrap()
        };
        let sql = format!("select id,customer_id,amount from ticket where game_id={} and (term_code={} or term_code_list like '%{}%') and (status=10 or status=15)", game_id, term_code, term_code);
        let rst = db.execute(&sql);
        let mut data_rst = try!(rst);
        let mut obj_rst = data_rst.as_object_mut().unwrap();
        let data = obj_rst.remove("data").unwrap();
        let list = data.as_array().unwrap();
        for ticket in list {
            let id_node = ticket.find("id").unwrap();
            let id = id_node.as_i64().unwrap();

            let customer_id_node = ticket.find("customer_id").unwrap();
            let customer_id = customer_id_node.as_i64().unwrap();

            let amount_node = ticket.find("amount").unwrap();
            let amount = amount_node.as_i64().unwrap();

            let _ = TicketService.refund(id, customer_id, amount, db);            
        }
        Result::Ok(json!("{}"))
    }
}

//获得出票池票据列表
pub struct ATI12;

impl DataApi for ATI12 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let table = db.get_table("print_pool").unwrap();
		
		let cond = json_str!(msg; "body", "cond");
        let mut cond_json = json!(cond);

        let sort = json_str!(msg; "body", "sort");
        let limit = json_i64!(msg; "body", "limit");
        let offset = json_i64!(msg; "body", "offset");
        let mut op = json!("{}");
        json_set!(&mut op; "limit"; limit);
        json_set!(&mut op; "offset"; offset);
        json_set!(&mut op; "sort"; json!(sort));
        
        let rst = table.count(&cond_json, &json!("{}"));
        let back_json = try!(rst);
    	let count = json_i64!(&back_json; "data", "0", "count");

        let mut doc = json!("{}");
    	let rst = table.find(&cond_json, &doc, &op);
        let mut back_json = try!(rst);

   		json_set!(&mut back_json;"count";count);
   		let game_list = GameService.get_game_list();
   		json_set!(&mut back_json;"game_list";game_list);
   		json_set!(&mut back_json;"ticket_status";CONS.get_json_obj("ticket_status"));
    	Result::Ok(back_json)
    }
}

//重新把出票池票据发送给出票程序
pub struct ATI13;

impl DataApi for ATI13 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let body = msg.find("body").unwrap();
        let id = {
            let node = body.find("id").unwrap();
            node.as_i64().unwrap()
        };
        let version = {
            let node = body.find("version").unwrap();
            node.as_i64().unwrap()
        };
        TicketService.back_to_wait_list(id, version, db)
    }
}














