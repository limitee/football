extern crate util;
use util::*;

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

//admin get customer detail info
pub struct AA03;

impl DataApi for AA03 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = {
   			let user_type = json_str!(head; "userType");
    		if user_type == "admin" || user_type == "center" {
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
        json_set!(&mut cond; "id"; com_id);

        let mut op = json!("{}");
        
        let mut back_json = json!("{}");

        //先获得用户信息
        let rst = table.find_one(&cond, &json!("{}"), &op);
        let cus_json = try!(rst);
       	json_set!(&mut back_json; "customer"; cus_json);
        		
        //获得帐户信息
        let table = db.get_table("account").expect("account table not exists.");
       	let mut cond = json!("{}");
       	json_set!(&mut cond; "id"; com_id);
       	let mut op = json!("{}");
       	let rst = table.find_one(&cond, &json!("{}"), &op);
        let account_json = try!(rst);

    	json_set!(&mut back_json; "account"; account_json);
       	Result::Ok(back_json)
    }
}

//admin get account list
pub struct AA04;

impl DataApi for AA04 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = {
			let user_type = json_str!(head; "userType");
    		if user_type == "admin" || user_type == "center" {
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
        let cur_time_string;
		let cur_time = json_i64!(msg; "body", "cur_time");
        if cur_time > 0 {
            cur_time_string = format!("_{}", cur_time);
        } else {
            cur_time_string = "".to_string();
        }
        
     	let table = db.get_table("moneylog").expect("moneylog table not exists.");
		let com_id = json_i64!(msg; "body", "id");
		
		let cond = json_path!(msg; "body", "cond");
        let mut cond = cond.clone();
		json_set!(&mut cond; "customer_id"; com_id);
        let cond_string = table.get_cond(&cond, "", None);

		let op = json_path!(msg; "body", "op");
        let op_string = table.get_option(op, None);
        
        let sql = format!("select count(id) from moneylog{} where {}", cur_time_string, cond_string);
        let mut rst_data = try!(db.execute(&sql));
        let count = get_count(rst_data);
        let mut back_json = json!("{}");
       	json_set!(&mut back_json; "count"; count);

        let sql = format!("select * from moneylog{} where {} {}", cur_time_string, cond_string, op_string);
        let mut rst_data = try!(db.execute(&sql));
       	json_set!(&mut back_json; "moneylog"; rst_data);

       	json_set!(&mut back_json; "moneylog_type"; CONS.get_json_obj("moneylog_type"));
       	json_set!(&mut back_json; "cur_year"; 2016);
       	json_set!(&mut back_json; "cur_month"; 7);

    	Result::Ok(back_json)
    }
}

//给销售渠道充值
pub struct AA05;

impl DataApi for AA05 {

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
		let table = db.get_table("account").expect("account table not exists.");
		let com_id = json_i64!(msg; "body", "id");
		let amount = json_i64!(msg; "body", "amount");
		let order_id = json_str!(msg; "body", "order_id");
		
		let rst = AccountService.handle(com_id, order_id, 
			CONS.code_to_id("moneylog_type", "charge").unwrap(), 
			amount, db
		);
		rst
	}
    
}

//admin get charge_report list
pub struct AA06;

impl DataApi for AA06 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = {
			let user_type = json_str!(head; "userType");
    		if user_type == "admin" || user_type == "center" {
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
     	let table = db.get_table("charge_report").unwrap();
		let com_id = json_i64!(msg; "body", "id");
		
		let cond = json_str!(msg; "body", "cond");
        let mut cond_json = json!(cond);
		json_set!(&mut cond_json; "terminal_id"; com_id);

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
       		json_set!(&mut back_json; "list"; cus_json);
       		json_set!(&mut back_json; "charge_report_status"; CONS.get_json_obj("charge_report_status"));
       		Result::Ok(back_json)
        });
        rst
    }
}

//admin charge from the charge_report
pub struct AA07;

impl DataApi for AA07 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = {
			let user_type = json_str!(head; "userType");
    		if user_type == "admin" || user_type == "center" {
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
        let body = json_path!(msg; "body");
        let report_id = json_i64!(body; "report_id");
        let terminal_id = json_i64!(body; "terminal_id");
        let flag = json_i64!(body; "flag");
			
		let mut cond = json!("{}");
		json_set!(&mut cond; "terminal_id"; terminal_id);
		json_set!(&mut cond; "id"; report_id);
		json_set!(&mut cond; "status"; CONS.code_to_id("charge_report_status", "unhandle").unwrap());
			
		let mut doc = json!("{}");
		let mut set_data = json!("{}");
		json_set!(&mut set_data; "status"; CONS.code_to_id("charge_report_status", "handled").unwrap());
		json_set!(&mut doc; "$set"; set_data);
			
		let mut op = json!("{}");
		let mut ret_data = json!("{}");
		json_set!(&mut ret_data; "amount"; 1);
		json_set!(&mut ret_data; "terminal_id"; 1);
		json_set!(&mut op; "ret"; ret_data);
			
        let table = db.get_table("charge_report").unwrap();
		let rst = table.update(&cond, &doc, &op);
        let data = try!(rst);
        let rows = json_i64!(&data; "rows");
        let rst;
        if rows > 0 {
            let amount = json_i64!(&data; "data", "0", "amount");
            rst = Result::Ok(amount);
        } else {
            rst = Result::Err(ErrCode::DataExpired as i32);
        }
        let amount = try!(rst);
        if flag == 0 {  //不需要充值
            return Result::Ok(json!("{}"));
        }
        let order_id = format!("charge_report_{}", report_id);
        let rst = AccountService.handle(terminal_id, order_id.as_str(), 
		    CONS.code_to_id("moneylog_type", "charge").unwrap(), 
		    amount, db
		);
        rst
    }
}

//admin 矫正终端机余额 
pub struct AA08;

impl DataApi for AA08 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = {
			let user_type = json_str!(head; "userType");
    		if user_type == "admin" || user_type == "center" {
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
		let table = db.get_table("account").expect("account table not exists.");
		let com_id = json_i64!(msg; "body", "id");
		let amount = json_i64!(msg; "body", "amount");
		let order_id = json_str!(msg; "body", "order_id");
		
		let rst = AccountService.handle(com_id, order_id, 
			CONS.code_to_id("moneylog_type", "reset").unwrap(), 
			amount, db
		);
		rst
    }
}


