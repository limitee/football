extern crate util;
use util::*;

use super::super::dc::DataBase;
use super::super::dc::MyDbPool;
use super::super::cons::CONS;
use super::super::cons::GLF;
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
use super::super::sv_util::{KeyHelper, get_dlt_draw_info};

extern crate game;
use self::game::base::GF;

extern crate sev_helper;
use sev_helper::GameService;
use sev_helper::TermService;


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

//获得所有游戏列表
pub struct AG01;

impl DataApi for AG01 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
		Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let game_list = GameService.get_game_list();
        
        let mut back_body = json!("{}");
        json_set!(&mut back_body; "data"; game_list);
		let rst = Result::Ok(back_body);
		rst
    }
}

//添加期次
pub struct AGT01;

impl DataApi for AGT01 {

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

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let data = json_path!(msg; "body", "data");
        let (sale_time, end_time) = {
        		(json_str!(data; "sale_time"), json_str!(data; "end_time"))
        };
        let mut term = data.clone();
        
        let sale_time_local = Local.datetime_from_str(sale_time, "%Y-%m-%d %H:%M:%S").unwrap();
        json_set!(&mut term; "sale_time"; sale_time_local.timestamp());
        
        let end_time_local = Local.datetime_from_str(end_time, "%Y-%m-%d %H:%M:%S").unwrap();
        json_set!(&mut term; "end_time"; end_time_local.timestamp());
        
        json_set!(&mut term; "status"; CONS.code_to_id("term_status", "init").unwrap());
        
        let table = db.get_table("term").expect("term table not exist.");
        let rst = table.save(&term, &json!("{}"));
        rst
    }
    
}

//获得期次列表
pub struct AGT02;

impl DataApi for AGT02 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let table = db.get_table("term").expect("term table not exists.");
		
		let cond = json_str!(msg; "body", "cond");
        let mut cond_json = json!(cond);

        let sort = json_str!(msg; "body", "sort");
        let limit = json_i64!(msg; "body", "limit");
        let offset = json_i64!(msg; "body", "offset");
        let mut op = json!("{}");
        json_set!(&mut op; "limit"; limit);
        json_set!(&mut op; "offset"; offset);
        json_set!(&mut op; "sort"; json!(sort));
        
        let mut count = 0;
        let rst = table.count(&cond_json, &json!("{}"));
        let rst = rst.and_then(|back_json|{
        	count = json_i64!(&back_json; "data", "0", "count");
        	table.find(&cond_json, &json!("{}"), &op)
        });
        let rst = rst.and_then(|mut back_json|{
        	json_set!(&mut back_json;"count";count);
        		
        	let game_list = GameService.get_game_list();
        	json_set!(&mut back_json;"game_list";game_list);
        		
        	json_set!(&mut back_json;"term_status";CONS.get_json_obj("term_status"));
        	Result::Ok(back_json)
        });
        rst
    }
}

//更新期次信息
pub struct AGT03;

impl DataApi for AGT03 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
		Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let table = db.get_table("term").expect("term table not exists.");
		
		let cond = json_path!(msg; "body", "cond");

		let doc = json_path!(msg; "body", "doc");
        let mut doc = doc.clone();
        let mut inc_data = json();
        json_set!(&mut inc_data; "version"; 1);
        json_set!(&mut doc; "$inc"; inc_data);

        let rst = table.update(cond, &doc, &json!("{}"));
        rst
    }
}

//根据id获得游戏期次信息
pub struct AGT04;

impl DataApi for AGT04 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
		Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let table = db.get_table("term").expect("term table not exists.");
		
		let id = json_i64!(msg; "body", "id");
		let mut cond = json!("{}");
		json_set!(&mut cond; "id"; id);
		
        let mut back_json = json!("{}");
        let rst = table.find_one(&cond, &json!("{}"), &json!("{}"));
        let rst = rst.and_then(|mut json| {
    		json_set!(&mut back_json; "term"; json);
            Result::Ok(1)
        });
    	json_set!(&mut back_json; "term_status";CONS.get_json_obj("term_status"));
        Result::Ok(back_json)
    }
}

//获得游戏，期次的奖级信息
pub struct AGT05;

impl DataApi for AGT05 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
		Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let table = db.get_table("game_level").expect("term table not exists.");
		let game_id = json_i64!(msg; "body", "game_id");
		let term_code = json_i64!(msg; "body", "term_code");
		
		let rst = GLF.get_by_game_id(game_id as i32).ok_or(ErrCode::ValueNotExist as i32);
		let list = try!(rst);
		
		let mut gl_list = Vec::<Json>::new(); 
		for gl in list {
			gl_list.push(gl.clone());
		}
		
		let mut cond = json!("{}");
		json_set!(&mut cond; "game_id"; game_id);
		json_set!(&mut cond; "term_code"; term_code);
		
		let doc = json!("{}");
		let op = json!("{}");
		
		let rst = table.find(&cond, &doc, &op);
		let mut db_data = try!(rst);
		let mut db_list = db_data.as_object_mut().unwrap().remove("data").unwrap();
		
		let mut back_json = json!("{}");
		json_set!(&mut back_json; "gl_list"; gl_list);
		json_set!(&mut back_json; "db_list"; db_list);
		
        Result::Ok(back_json)
    }
}

//保存游戏，期次的奖级信息
pub struct AGT06;

impl DataApi for AGT06 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
		Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        //let table = db.get_table("game_level").expect("game_level table not exists.");
		let list_json = json_path!(msg; "body", "gl_list");
		let list = list_json.as_array().unwrap();
		for gl in list {
			let _ = try!(TermService.save_gl(gl, db));
		}
        Result::Ok(json!("{}"))
    }
}

//期次开奖
pub struct AGT07;

impl DataApi for AGT07 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
		Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let body = msg.find("body").unwrap();
        let term_id = {
            let node = body.find("term_id").unwrap();
            node.as_i64().unwrap()
        };
        let version = {
            let node = body.find("version").unwrap();
            node.as_i64().unwrap()
        };
		let rst = TermService.draw(term_id, version, db);
        rst
    }
}

//dlt抓取开奖号码
pub struct AGT08;

impl DataApi for AGT08 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
		Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
		let term_id = json_i64!(msg; "body", "term_id");
		let url = json_str!(msg; "body", "url");
        let term_rst = TermService.find_by_id(term_id, db);
        let term = try!(term_rst);
        let game_id = json_i64!(&term; "game_id");
        if game_id != 200 {
            return Result::Err(-1);
        }
        let code = json_i64!(&term; "code");
        let (term_code, draw_number, gl_list) = get_dlt_draw_info(url);
        if code != term_code {
            error!("期次信息不一致:client:{}, server:{}. ", term_code, code);
            return Result::Err(-1);
        }
        for ref gl in gl_list {
            let _ = TermService.save_gl(gl, db);
        }
        
		let table = db.get_table("term").unwrap();
        let mut cond = json!("{}");
		json_set!(&mut cond; "id"; term_id);

        //check the term draw number 
        let mut doc = json!("{}");
        let mut set_data = json!("{}");
		json_set!(&mut set_data; "draw_number"; draw_number);
		json_set!(&mut doc; "$set"; set_data);

        let op = json!("{}");

        table.update(&cond, &doc, &op)
    }
}
