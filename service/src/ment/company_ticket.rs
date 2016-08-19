use super::super::util::DigestUtil;
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
use game::base::GF;
use game::VF;

extern crate sev_helper;
use sev_helper::GameService;
use sev_helper::BetService;
use sev_helper::CacheService;

use std::collections::HashSet;

struct Helper;

impl Helper {
	
	fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
		let rst = {
			let user_type = json_str!(head; "userType");
	    		if user_type == "company" {
	    			Result::Ok(1)
	    		} else {
	    			Result::Err(ErrCode::NotAllowed as i32)	
	    		}
		};
		let rst = rst.and_then(|_|{
			KeyHelper::from_db_by_id(db, head)
		});
		rst
	}
	
}

///获得竞彩的期次列表
fn get_jc_termcode_list(number:&str) -> Result<String, i32> {
	let mut term_set =  HashSet::<String>::new();
    let number_array:Vec<&str> = number.split('|').collect();
    let number_cc_array:Vec<&str> = number_array[0].split(';').collect();
    info!("{:?}", number_cc_array);
    let mut list = String::new();
    let mut count = 0;
    for cc in number_cc_array {
        if count > 0 {
            list.push_str(";");
        }
        let (term_code, _)  = cc.split_at(11);
        term_set.insert(term_code.to_string());
        list.push_str(term_code);
        count += 1; 
    }
    if term_set.len() != count {
        return  Result::Err(ErrCode::TermStatusNotAllowed as i32);
    }
    Result::Ok(list)
}

//投注
pub struct CT01;

impl DataApi for CT01 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
		Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
   		let body = json_path!(msg; "body");
        let tickets_rst = body.find("tickets").ok_or(ErrCode::KeyIsMissing as i32);
        let rst = tickets_rst.and_then(|tickets| {
       		let mut rst = Result::Ok(1);
       		rst
        });
        rst
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let table = db.get_table("ticket").expect("table not exists.");
        let customer_id = json_i64!(msg; "head", "userId");
        let body = json_path!(msg; "body");
        let tickets = json_path!(body; "tickets").as_array().unwrap();
        let now = time::get_time();
        
        let mut back_tickets:Vec<Json> = Vec::new();
        for ticket in tickets {
       		let out_id = json_str!(ticket; "out_id");
       		let game_id = json_i64!(ticket; "game_id");
       		let play_type = json_i64!(ticket; "play_type");
       		let bet_type = json_i64!(ticket; "bet_type");
       		let number = json_str!(ticket; "number");
       		let mut tk = ticket.clone();
       		json_set!(&mut tk; "customer_id"; customer_id);
       		json_set!(&mut tk; "game_id"; game_id);
       		json_set!(&mut tk; "play_type"; play_type);
       		json_set!(&mut tk; "bet_type"; bet_type);
       		json_set!(&mut tk; "create_time"; now.sec);
       		json_set!(&mut tk; "status"; CONS.code_to_id("ticket_status", "printing").unwrap());
        		
       		info!("{}", tk);
       		let mut back_ticket = json!("{}");
       		
       		let rst = VF.validate(ticket);
            //校验out_id是否重复
            let rst = rst.and_then(|_|{
                let rst = CacheService.get_com_out_id(customer_id, out_id, db);   
                rst.and_then(|flag|{
                    if flag {
                        Result::Err(ErrCode::OutIdRepeat as i32)
                    } else {
                        Result::Ok(1)
                    }
                })
            });
       		let rst = rst.and_then(|_|{
                match game_id {
          		    //竞彩单场，设置票据的期次编码
                    202_i64 => {
                        let (term_code, _)  = number.split_at(11);
	        		    json_set!(&mut tk; "term_code_list"; term_code);
	        		    json_set!(&mut tk; "term_code"; i64::from_str(term_code).unwrap());
	        	        Result::Ok(1)
                    },
                    201_i64 | 301_i64 => {
                        let term_list_rst = get_jc_termcode_list(number);
                        match term_list_rst {
                            Ok(term_list) => {
                                info!("{}", term_list);
       		                    json_set!(&mut tk; "term_code"; 0);
	        		            json_set!(&mut tk; "term_code_list"; term_list);
	        	                Result::Ok(1)
                            },
                            Err(code) => {
	        	                Result::Err(code)
                            },
                        }
                    },
                    _ => {
       		            let term_code = json_i64!(ticket; "term_code");
       		            json_set!(&mut tk; "term_code"; term_code);
	        	        Result::Ok(1)
                    },
                }
        	});
       		let rst = rst.and_then(|_| {
	       		BetService.bet(customer_id, &mut tk, db)	
       		});
       		match rst {
       			Ok(id) => {
                    CacheService.set_com_out_id(customer_id, out_id, db);
       				json_set!(&mut back_ticket; "id"; id);
       				json_set!(&mut back_ticket; "out_id"; out_id);
                    let err = CONS.get_err(ErrCode::Success as i32);
       				json_set!(&mut back_ticket; "err"; err);
       			},
       			Err(code) => {
       				json_set!(&mut back_ticket; "out_id"; out_id);
                    let err = CONS.get_err(code);
       				json_set!(&mut back_ticket; "err"; err);	
       			}
       		}
       		back_tickets.push(back_ticket);
        }
        
        let mut back_body = json!("{}");
        json_set!(&mut back_body; "tickets"; back_tickets);
        let rst = Result::Ok(back_body);
		rst
    }
}

//获得一张票据的返回结果
fn get_one_back_ticket(mut ticket:Json) -> Json {
    let status = {
        let status_node = ticket.find("status").unwrap();
        status_node.as_i64().unwrap()
    };
    let back_status = {
         match status {
             20 | 30 | 40 | 55 | 60 | 65 | 70 => {
                20
             },
             50 => {
                 50
             },
             _  => {
                 10
             }
         }
    };
    {
        let mut obj = ticket.as_object_mut().unwrap();
        obj.insert("status".to_string(), back_status.to_json());
    }
    ticket
}

///返回一张不存在的票据信息
fn get_one_unexist_ticket(id_node:Option<&Json>, out_id_node:Option<&Json>) -> Json {
    let mut tree = BTreeMap::<String, Json>::new();
    if let Some(node) = id_node {
        tree.insert("id".to_string(), node.clone()); 
    }
    if let Some(node) = out_id_node {
        tree.insert("out_id".to_string(), node.clone()); 
    }
    tree.insert("status".to_string(), Json::I64(-1));
    Json::Object(tree)
}

///获得返回的票据
fn get_back_ticket(cond:&Json, db:&DataBase<MyDbPool>) -> Json {
    let mut doc = json!("{}");
	json_set!(&mut doc; "id"; 1);
	json_set!(&mut doc; "out_id"; 1);
	json_set!(&mut doc; "game_id"; 1);
	json_set!(&mut doc; "play_type"; 1);
	json_set!(&mut doc; "bet_type"; 1);
	json_set!(&mut doc; "status"; 1);
	json_set!(&mut doc; "bonus"; 1);
	json_set!(&mut doc; "bonus_after_tax"; 1);
	json_set!(&mut doc; "create_time"; 1);
	json_set!(&mut doc; "print_time"; 1);
	json_set!(&mut doc; "bonus_detail"; 1);
	json_set!(&mut doc; "number"; 1);
	json_set!(&mut doc; "print_number"; 1);
	let op = json!("{}");

    let table = db.get_table("ticket").expect("table not exists.");
    let rst = table.find_one(&cond, &doc, &op);
    let mut back_ticket;
    match rst {
        Ok(mut ticket) => {
			back_ticket = get_one_back_ticket(ticket);
		},
		Err(_) => {
            let id_node = cond.find("id");
            let out_id_node = cond.find("out_id");
			back_ticket = get_one_unexist_ticket(id_node, out_id_node);
		}
	}
	back_ticket
}

//查询票据状态
pub struct CT02;

impl DataApi for CT02 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
		Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
   		Result::Ok(1)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let table = db.get_table("ticket").expect("table not exists.");
        let customer_id = json_i64!(msg; "head", "userId");
        let body = json_path!(msg; "body");

        let ticket_id_op = body.find("id");
        let out_id_op = body.find("out_id");

        if ticket_id_op.is_none() && out_id_op.is_none() {
            return Result::Err(ErrCode::KeyIsMissing as i32);
        }
        let mut cond = json!("{}");
        json_set!(&mut cond; "customer_id"; customer_id);
        if let Some(ticket_id) = ticket_id_op {
            json_set!(&mut cond; "id"; ticket_id);
        }
        if let Some(out_id) = out_id_op {
            json_set!(&mut cond; "out_id"; out_id);
        }
        let back_ticket = get_back_ticket(&cond, db);
        let mut back_body = json!("{}");
        json_set!(&mut back_body; "ticket"; back_ticket);
	    Result::Ok(back_body)
    }
}

///
fn get_tickets_by_ids(customer_id:i64, ids_node:&Json, db:&DataBase<MyDbPool>) -> BTreeMap<i64, Json> {
    let mut cond_map = BTreeMap::new();
    cond_map.insert("customer_id".to_string(), customer_id.to_json());

    let mut in_data = BTreeMap::new();
    in_data.insert("$in".to_string(), ids_node.clone());

    cond_map.insert("id".to_string(), Json::Object(in_data));

    let table = db.get_table("ticket").expect("table not exists.");

    let cond = Json::Object(cond_map);
    
    let mut doc = json!("{}");
	json_set!(&mut doc; "id"; 1);
	json_set!(&mut doc; "out_id"; 1);
	json_set!(&mut doc; "game_id"; 1);
	json_set!(&mut doc; "play_type"; 1);
	json_set!(&mut doc; "bet_type"; 1);
	json_set!(&mut doc; "status"; 1);
	json_set!(&mut doc; "bonus"; 1);
	json_set!(&mut doc; "bonus_after_tax"; 1);
	json_set!(&mut doc; "create_time"; 1);
	json_set!(&mut doc; "print_time"; 1);
	json_set!(&mut doc; "bonus_detail"; 1);
	json_set!(&mut doc; "number"; 1);
	json_set!(&mut doc; "print_number"; 1);

    let op = json!("{}");
    let rst = table.find(&cond, &doc, &op);
    let mut rst_data = rst.unwrap();
    let mut data_obj = rst_data.as_object_mut().unwrap();
    let mut list = data_obj.remove("data").unwrap();
    let array = list.as_array_mut().unwrap();
    let mut map = BTreeMap::<i64, Json>::new();
    loop {
        let ticket_op = array.pop();
        if let Some(ticket) = ticket_op {
            let id = {
                let id_node = ticket.find("id").unwrap(); 
                id_node.as_i64().unwrap()
            };
            map.insert(id, get_one_back_ticket(ticket));
        } else {
            break;
        }
    }
    map
}

///
fn get_tickets_by_out_ids(customer_id:i64, ids_node:&Json, db:&DataBase<MyDbPool>) -> BTreeMap<String, Json> {
    let mut cond_map = BTreeMap::new();
    cond_map.insert("customer_id".to_string(), customer_id.to_json());

    let mut in_data = BTreeMap::new();
    in_data.insert("$in".to_string(), ids_node.clone());

    cond_map.insert("out_id".to_string(), Json::Object(in_data));

    let table = db.get_table("ticket").expect("table not exists.");

    let cond = Json::Object(cond_map);
    
    let mut doc = json!("{}");
	json_set!(&mut doc; "id"; 1);
	json_set!(&mut doc; "out_id"; 1);
	json_set!(&mut doc; "game_id"; 1);
	json_set!(&mut doc; "play_type"; 1);
	json_set!(&mut doc; "bet_type"; 1);
	json_set!(&mut doc; "status"; 1);
	json_set!(&mut doc; "bonus"; 1);
	json_set!(&mut doc; "bonus_after_tax"; 1);
	json_set!(&mut doc; "create_time"; 1);
	json_set!(&mut doc; "print_time"; 1);
	json_set!(&mut doc; "bonus_detail"; 1);
	json_set!(&mut doc; "number"; 1);
	json_set!(&mut doc; "print_number"; 1);

    let op = json!("{}");
    let rst = table.find(&cond, &doc, &op);
    let mut rst_data = rst.unwrap();
    let mut data_obj = rst_data.as_object_mut().unwrap();
    let mut list = data_obj.remove("data").unwrap();
    let array = list.as_array_mut().unwrap();
    let mut map = BTreeMap::<String, Json>::new();
    loop {
        let ticket_op = array.pop();
        if let Some(ticket) = ticket_op {
            let id = {
                let id_node = ticket.find("out_id").unwrap(); 
                id_node.as_string().unwrap().to_string()
            };
            map.insert(id, get_one_back_ticket(ticket));
        } else {
            break;
        }
    }
    map
}

//批量查询票据状态
pub struct CT03;

impl DataApi for CT03 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
		Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
   		Result::Ok(1)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let table = db.get_table("ticket").expect("table not exists.");
        let customer_id = json_i64!(msg; "head", "userId");
        let body = json_path!(msg; "body");

        let ticket_id_op = body.find("id");
        let out_id_op = body.find("out_id");

        if ticket_id_op.is_none() && out_id_op.is_none() {
            return Result::Err(ErrCode::KeyIsMissing as i32);
        }
        let mut tickets = Vec::new();
        if let Some(ticket_ids) = ticket_id_op {
            let mut map = get_tickets_by_ids(customer_id, ticket_ids, db);
            let ids_array = ticket_ids.as_array().unwrap();
            for id_node in ids_array {
                let id = id_node.as_i64().unwrap(); 
                let ticket_op = map.remove(&id);
                if let Some(ticket) = ticket_op {
                    tickets.push(ticket); 
                } else {
                    tickets.push(get_one_unexist_ticket(Some(id_node), None)); 
                }
            }
        }
        if let Some(out_ids) = out_id_op {
            let mut map = get_tickets_by_out_ids(customer_id, out_ids, db);
            let ids_array = out_ids.as_array().unwrap();
            for id_node in ids_array {
                let id = id_node.as_string().unwrap(); 
                let ticket_op = map.remove(id);
                if let Some(ticket) = ticket_op {
                    tickets.push(ticket); 
                } else {
                    tickets.push(get_one_unexist_ticket(Some(id_node), None)); 
                }
            }
        }
        let mut back_body = json!("{}");
		json_set!(&mut back_body; "tickets"; tickets);
        Result::Ok(back_body)
    }
    
}

//余额查询接口
pub struct CT04;

impl DataApi for CT04 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
		Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
   		Result::Ok(1)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        //info!("{}", msg);
        let customer_id = json_i64!(msg; "head", "userId");
        //let body = json_path!(msg; "body");

        let table = db.get_table("account").unwrap();
        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; customer_id);

        let mut doc = json!("{}");
        json_set!(&mut doc; "balance"; 1);
        let op = json!("{}");

        let account_rst = table.find_one(&cond, &doc, &op);
        let account = try!(account_rst);

        let mut back_body = json!("{}");
        json_set!(&mut back_body; "account"; account);
        Result::Ok(back_body)
    }
}

