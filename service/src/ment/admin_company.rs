use super::super::util::DigestUtil;
use super::super::dc::DataBase;
use super::super::dc::MyDbPool;
use super::super::cons::CONS;
use super::super::cons::ErrCode;

use std::collections::BTreeMap;
use std::io::Read;

extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;
use std::str::FromStr;

extern crate regex;
use self::regex::Regex;

extern crate time;

extern crate sev_helper;
use sev_helper::AccountService;

use super::super::inter::{DataApi};
use super::super::sv_util::{KeyHelper};

//admin add terminal
pub struct AC01;

impl DataApi for AC01 {

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
        let mut data = data.clone();
        json_set!(&mut data; "nickname"; "company");
        let now = time::get_time();
        json_set!(&mut data; "reg_time"; now.sec);
        json_set!(&mut data; "type"; CONS.code_to_id("user_type", "company").unwrap());
		
		let mut op = json!("{}");
		let mut ret_data = json!("{}");
		json_set!(&mut ret_data; "id"; 1);
		json_set!(&mut op; "ret"; ret_data);
		
		let table = db.get_table("customer").expect("customer table not exist.");
		let rst = table.save(&data, &op);
		
		//save the account info		
		let rst = rst.and_then(|back_json| {
			let id = json_i64!(&back_json; "data", "0", "id");
			let table = db.get_table("account").expect("account table not exist.");
			
			let mut data = json!("{}");
			json_set!(&mut data; "id"; id);
			
			let op = json!("{}");
			
			let rst = table.save(&data, &op);
			rst
		});
		
		rst
    }
    
}


//admin get company list
pub struct AC02;

impl DataApi for AC02 {

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
     	let customer_id = json_i64!(msg; "head", "userId");
        let table = db.get_table("customer").expect("customer table not exists.");

        let cond = json_str!(msg; "body", "cond");
        let mut cond_json = json!(cond);
        json_set!(&mut cond_json; "type"; CONS.code_to_id("user_type", "company").unwrap());

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
        		Result::Ok(back_json)
        });
        rst
    }
}


//admin get company detail info
pub struct AC03;

impl DataApi for AC03 {

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
     	let table = db.get_table("customer").expect("customer table not exists.");
		let com_id = json_i64!(msg; "body", "id");
		
        let mut cond = json!("{}");
        json_set!(&mut cond; "type"; CONS.code_to_id("user_type", "company").unwrap());
        json_set!(&mut cond; "id"; com_id);

        let mut op = json!("{}");
        
        let mut back_json = json!("{}");
        let rst = table.find_one(&cond, &json!("{}"), &op);
        let rst = rst.and_then(|cus_json|{
        		json_set!(&mut back_json; "customer"; cus_json);
        		
        		let table = db.get_table("account").expect("account table not exists.");
        		let mut cond = json!("{}");
        		json_set!(&mut cond; "id"; com_id);
        		
        		let mut op = json!("{}");
        		table.find_one(&cond, &json!("{}"), &op)
        });
        let rst = rst.and_then(|account_json|{
        		json_set!(&mut back_json; "account"; account_json);
        		Result::Ok(back_json)
        });
        rst
    }
}

//admin get account list
pub struct AC04;

impl DataApi for AC04 {

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
    	//info!("{}", msg);
     	let table = db.get_table("moneylog").expect("moneylog table not exists.");
		let com_id = json_i64!(msg; "body", "id");
		
		let cond = json_str!(msg; "body", "cond");
        let mut cond_json = json!(cond);
		json_set!(&mut cond_json; "customer_id"; com_id);

        let sort = json_str!(msg; "body", "sort");
        let limit = json_i64!(msg; "body", "limit");
        let offset = json_i64!(msg; "body", "offset");
        let mut op = json!("{}");
        json_set!(&mut op; "limit"; limit);
        json_set!(&mut op; "offset"; offset);
        json_set!(&mut op; "sort"; json!(sort));
        
        let mut back_json = json!("{}");
        let rst = table.count(&cond_json, &json!("{}"));
        let rst = rst.and_then(|c_json|{
        		let count = json_i64!(&c_json; "data", "0", "count");
        		json_set!(&mut back_json; "count"; count);
        		table.find(&cond_json, &json!("{}"), &op)
        });
        let rst = rst.and_then(|cus_json|{
        		json_set!(&mut back_json; "moneylog"; cus_json);
        		
        		json_set!(&mut back_json; "moneylog_type"; CONS.get_json_obj("moneylog_type"));
        		Result::Ok(back_json)
        });
        rst
    }
}