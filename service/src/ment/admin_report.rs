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
use sev_helper::GameService;

use super::super::inter::{DataApi};
use super::super::sv_util::{KeyHelper};

extern crate chrono;
use chrono::*;

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

fn get_in_string(col_name:&str, mut data:Json) -> String {
    let mut objs = data.as_object_mut().unwrap();
    let list_json = objs.remove("data").unwrap();
    let list = list_json.as_array().unwrap();
    let mut in_cond = format!("{} in (", col_name);
    let mut count = 0;
    for company in list {
        let id_node = company.find("id").unwrap();
        let id = id_node.as_i64().unwrap();
        if count > 0 {
            in_cond.push_str(",");
        }
        in_cond.push_str(id.to_string().as_str());
        count += 1;
    }
    in_cond.push_str(")");
    in_cond
}

//admin get system report 
pub struct AR01;

impl DataApi for AR01 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
		Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);

        let mut back_json = BTreeMap::new();
        let sale_channel_type = CONS.code_to_id("user_type", "company").unwrap();
        let sql = format!("select id from customer where type={}", sale_channel_type);
        let rst = db.execute(&sql);
        let mut company_objs = try!(rst);
        let in_cond = get_in_string("id", company_objs);
        let sql = format!("select sum(balance)::bigint as balance from account where {}", in_cond);
        let rst = db.execute(&sql);
        let obj = try!(rst);
        let balance = json_i64!(&obj; "data", "0", "balance");
        back_json.insert("company_balance".to_string(), balance.to_json());

        let user_type = CONS.code_to_id("user_type", "terminal").unwrap();
        let sql = format!("select id from customer where type={}", user_type);
        let rst = db.execute(&sql);
        let mut data = try!(rst);
        let in_cond = get_in_string("id", data);
        let sql = format!("select sum(balance)::bigint as balance from account where {}", in_cond);
        let rst = db.execute(&sql);
        let obj = try!(rst);
        let balance = json_i64!(&obj; "data", "0", "balance");
        back_json.insert("terminal_balance".to_string(), balance.to_json());

        let sql = "select sum(bonus_after_tax)::bigint as balance from ticket where status=55 or status=60";
        let rst = db.execute(sql);
        let obj = try!(rst);
        let balance = json_i64!(&obj; "data", "0", "balance");
        back_json.insert("ticket_balance".to_string(), balance.to_json());

        let rst = Result::Ok(Json::Object(back_json));
        rst
    }
}

fn get_date(date_str:&str, is_start:bool, time_type:i64) -> DateTime<Local> {
    let date_str_array:Vec<&str> = date_str.split("-").collect();
    let year = i32::from_str(date_str_array[0]).unwrap();
    let month = u32::from_str(date_str_array[1]).unwrap();
    let day = u32::from_str(date_str_array[2]).unwrap();
    if is_start {
        let start;
        if time_type == 0{
            start = Local.ymd(year, month, day).and_hms_milli(0, 0, 0, 0);
        } else {
            start = Local.ymd(year, month, day).and_hms_milli(6, 0, 0, 0);
        }
        return start;
    } else {
        let start;
        if time_type == 0{
            start = Local.ymd(year, month, day).and_hms_milli(0, 0, 0, 0);
        } else {
            start = Local.ymd(year, month, day).and_hms_milli(6, 0, 0, 0);
        }
        let end = start + Duration::days(1);
        return end;
    }
}

//admin get daiyly report 
pub struct AR02;

impl DataApi for AR02 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
		Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let body = json_path!(msg; "body");
        let date = json_str!(msg; "body", "start_date");
        let end_date = json_str!(msg; "body", "end_date");
        let time_type = json_i64!(msg; "body", "time_type");
        let start = get_date(date, true, time_type);
        let end = get_date(end_date, false, time_type);
        let start_stamp = start.timestamp();
        let end_stamp = end.timestamp();

        let mut back_json = BTreeMap::new();

        let user_type = CONS.code_to_id("user_type", "terminal").unwrap();
        let sql = format!("select id from customer where type={}", user_type);
        let rst = db.execute(&sql);
        let mut data = try!(rst);

        let in_cond = get_in_string("terminal_id", data);
        let mut cond_string = "where ".to_string(); 
        cond_string.push_str(&in_cond);
        cond_string.push_str(" ");
        let time_cond = format!("and print_time >= {} and print_time < {}", 
                start_stamp, end_stamp);
        cond_string.push_str(&time_cond);
        cond_string.push_str(" ");

        let game_ids_node_op = body.find("game_ids");
        if let Some(game_ids_node) = game_ids_node_op {
            let mut in_string = "and game_id in (".to_string();
            let mut count = 0;
            let array = game_ids_node.as_array().unwrap();
            for game_id in array {
                if count > 0 {
                    in_string.push_str(",");
                }
                let game_id_string = game_id.as_string().unwrap();
                in_string.push_str(game_id_string); 
                count += 1;
            }
            in_string.push_str(")");
            if count > 0 {
                cond_string.push_str(&in_string);
                cond_string.push_str(" ");
            }
        }

        let sql = format!("select c.id, c.username, temp.terminal_id, temp.balance from customer as c, (select terminal_id,sum(amount)::bigint as balance from ticket {}and status!=50 and status!=10 group by terminal_id) as temp where c.id=temp.terminal_id", cond_string);
        let rst = db.execute(&sql);
        let mut obj = try!(rst);
        let mut obj = obj.as_object_mut().unwrap();
        let mut detail = obj.remove("data").unwrap();
        back_json.insert("tdetail".to_string(), detail);

        let game_list = GameService.get_game_list();
        back_json.insert("game_list".to_string(), game_list.to_json());

        let rst = Result::Ok(Json::Object(back_json));
        rst
    }
}

//admin get daiyly report from sales
pub struct AR03;

impl DataApi for AR03 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
		Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let date = json_str!(msg; "body", "start_date");
        let end_date = json_str!(msg; "body", "end_date");
        let start = get_date(date, true, 0);
        let end = get_date(end_date, false, 0);
        let start_stamp = start.timestamp();
        let end_stamp = end.timestamp();

        let mut back_json = BTreeMap::new();

        let user_type = CONS.code_to_id("user_type", "company").unwrap();
        let sql = format!("select id from customer where type={}", user_type);
        let rst = db.execute(&sql);
        let mut data = try!(rst);
        let in_cond = get_in_string("customer_id", data);
        let sql = format!("select c.id, c.username, temp.customer_id, temp.game_id, temp.balance, temp.bonus, temp.bat from customer as c, (select customer_id,game_id,sum(amount)::bigint as balance,sum(bonus)::bigint as bonus,sum(bonus_after_tax)::bigint as bat from ticket where {} and create_time >= {} and create_time < {} and status!=50 and status!=10 group by customer_id,game_id) as temp where c.id=temp.customer_id", in_cond, start_stamp, end_stamp);
        let rst = db.execute(&sql);
        let mut obj = try!(rst);
        let mut obj = obj.as_object_mut().unwrap();
        let mut detail = obj.remove("data").unwrap();
        back_json.insert("tdetail".to_string(), detail);

        let game_list = GameService.get_game_list();
        back_json.insert("game_list".to_string(), game_list.to_json());

        let rst = Result::Ok(Json::Object(back_json));
        rst
    }
}

//获得终端机返佣记录
pub struct AR04;

impl AR04 {

    fn get_date(date:&str) -> i64 {
        let date_str = date.replace("-", "");
        i64::from_str(&date_str).unwrap()
    }
}

impl DataApi for AR04 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) 
        -> Result<String, i32> {
		Helper.get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let body = msg.find("body").unwrap();
        let start_date_node = body.find_path(&["add", "start_date"]).unwrap();
        let start_date = start_date_node.as_string().unwrap();
        let start_date = AR04::get_date(start_date);

        let end_date_node = body.find_path(&["add", "end_date"]).unwrap();
        let end_date = end_date_node.as_string().unwrap();
        let end_date = AR04::get_date(end_date);

        let sql = format!("select ts.*,c.username from terminal_sale as ts,customer c where ts.terminal_id=c.id and ts.sale_date >= {} and ts.sale_date <= {}", start_date, end_date);
        info!("{}", sql);

        let mut back_json = json!("{}");
 
        let rst = db.execute(&sql);
        let mut obj = try!(rst);
        let mut obj = obj.as_object_mut().unwrap();
        let mut detail = obj.remove("data").unwrap();
        json_set!(&mut back_json; "sets"; detail);

        let rst = Result::Ok(back_json);
        rst
    }
}

