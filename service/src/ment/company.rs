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
use self::game::base::GF;

extern crate sev_helper;
use sev_helper::GameService;

//销售方登录
pub struct C01;

impl DataApi for C01 {

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
			KeyHelper::from_db(db, head)
		});
		rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let key = json_str!(msg; "key");
        let username = json_str!(msg; "head", "userId");
        
        let table = db.get_table("customer").expect("table not exists.");
        let cond = format!(r#"
            {{
                "username":"{}"
            }}
        "#, username);
        let fd_back = try!(table.find_by_str(&cond, "{}", "{}"));
        let user_id = json_i64!(&fd_back; "data", "0", "id");
        
        let mut back_body = json!("{}");
        json_set!(&mut back_body; "userId"; user_id);
        json_set!(&mut back_body; "st"; key);
        
        let rst = Result::Ok(back_body);
		rst
    }
}