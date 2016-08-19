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

use super::super::inter::{DataApi};
use super::super::sv_util::{KeyHelper};

extern crate sev_helper;
use sev_helper::GameService;
use sev_helper::CacheService;

//admin add center 
pub struct ACT01;

impl DataApi for ACT01 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = KeyHelper::from_cache(db, head);
        KeyHelper::active(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        //println!("{}", msg);
        let data = json_path!(msg; "body", "data");
        let mut data = data.clone();
        let now = time::get_time();
        json_set!(&mut data; "reg_time"; now.sec);
        json_set!(&mut data; "type"; CONS.code_to_id("user_type", "center").unwrap());
		
		let mut op = json!("{}");
		let mut ret_data = json!("{}");
		json_set!(&mut ret_data; "id"; 1);
		json_set!(&mut op; "ret"; ret_data);
		
		let table = db.get_table("customer").expect("customer table not exist.");
		let rst = table.save(&data, &op);
		
		let back_json = try!(rst);
		let id = json_i64!(&back_json; "data", "0", "id");

        //新增终端机账户
		let table = db.get_table("account").expect("account table not exist.");
		let mut data = json!("{}");
		json_set!(&mut data; "id"; id);
		let op = json!("{}");
		let rst = table.save(&data, &op);

		rst
    }
}

//admin get center list
pub struct ACT02;

impl DataApi for ACT02 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = KeyHelper::from_cache(db, head);
        KeyHelper::active(db, head);
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
        json_set!(&mut cond_json; "type"; CONS.code_to_id("user_type", "center").unwrap());

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
        let mut json = try!(rst);
        {
            let mut json_obj = json.as_object_mut().unwrap();
            let mut data = json_obj.get_mut("data").unwrap();
            let mut list = data.as_array_mut().unwrap();
            for mut group in list {
                let id = {
                    let id_node = group.find("id").unwrap();
                    id_node.as_i64().unwrap()
                };
                let online_sql = format!("select count(*) from customer as c, terminal as t where c.group_id={} and c.id = t.id and t.status=1", id);
                let rst = db.execute(&online_sql);
                let online_data = rst.unwrap();
                let online_count = json_i64!(&online_data; "data", "0", "count");
                let offline_sql = format!("select count(*) from customer as c, terminal as t where c.group_id={} and c.id = t.id and t.status=0", id);
                let rst = db.execute(&offline_sql);
                let offline_data = rst.unwrap();
                let offline_count = json_i64!(&offline_data; "data", "0", "count");

                let mut group_obj = group.as_object_mut().unwrap();
                group_obj.insert("online_count".to_string(), online_count.to_json());
                group_obj.insert("offline_count".to_string(), offline_count.to_json());
            }
        }
        //省份信息
    	json_set!(&mut json; "province_type"; CONS.get_json_obj("province_type"));

        Result::Ok(json)
    }
}

//admin get center report 
pub struct ACT03;

impl DataApi for ACT03 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = KeyHelper::from_cache(db, head);
        KeyHelper::active(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
     	let customer_id = json_i64!(msg; "head", "userId");
        //let table = db.get_table("customer").expect("customer table not exists.");

        let id = json_i64!(msg; "body", "id");
        let sql = format!("select game_id,play_type,bet_type,count(*)::bigint,sum(amount)::bigint as amount,sum(bonus)::bigint as bonus from ticket where customer_id={} and (status=20 or status=30 or status=40 or status=60 or status=65 or status=70) group by game_id, play_type, bet_type order by game_id,play_type,bet_type", id);
        let rst = db.execute(sql.as_str());
        let data = try!(rst);
        info!("{}", data);

        let mut back = json!("{}");
        let game_list = GameService.get_game_list();
   		json_set!(&mut back;"game_list";game_list);
   		json_set!(&mut back;"data";data);

        Result::Ok(back)
    }
}

//admin get center detail info 
pub struct ACT04;

impl DataApi for ACT04 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = KeyHelper::from_cache(db, head);
        KeyHelper::active(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let table = db.get_table("customer").unwrap();
		let center_id = json_i64!(msg; "body", "center_id");

        let mut cond = json!("{}");
        json_set!(&mut cond; "type"; CONS.code_to_id("user_type", "center").unwrap());
        json_set!(&mut cond; "id"; center_id);

        let mut op = json!("{}");
        let rst = table.find_one(&cond, &json!("{}"), &op);
        let customer = try!(rst);

        let mut back_body = json!("{}");
        json_set!(&mut back_body; "center"; customer);

        //省份信息
    	json_set!(&mut back_body; "province_type"; CONS.get_json_obj("province_type"));

        Result::Ok(back_body)
    }
}

//admin update center detail info 
pub struct ACT05;

impl DataApi for ACT05 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = KeyHelper::from_cache(db, head);
        KeyHelper::active(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let body = json_path!(msg; "body");
        let id = json_i64!(body; "center_id");
        let province = json_i64!(body; "province");
        let nickname = json_str!(body; "nickname");

        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; id);

        let mut doc = json!("{}");
        let mut set_data = json!("{}");
        json_set!(&mut set_data; "province"; province);
        json_set!(&mut set_data; "nickname"; nickname);
        json_set!(&mut doc; "$set"; set_data);

        let table = db.get_table("customer").unwrap();
        let rst = table.update(&cond, &doc, &json!("{}"));
        let _ = try!(rst);

        //更新中心下所有终端的省份
        let mut cond = json!("{}");
        json_set!(&mut cond; "group_id"; id);

        let mut doc = json!("{}");
        let mut set_data = json!("{}");
        json_set!(&mut set_data; "province"; province);
        json_set!(&mut doc; "$set"; set_data);

        let table = db.get_table("customer").unwrap();
        let rst = table.update(&cond, &doc, &json!("{}"));
        rst
    }
}







