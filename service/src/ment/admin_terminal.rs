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

//admin add terminal
pub struct AT01;

impl DataApi for AT01 {

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
        json_set!(&mut data; "nickname"; "terminal");
        let now = time::get_time();
        json_set!(&mut data; "reg_time"; now.sec);
        json_set!(&mut data; "type"; CONS.code_to_id("user_type", "terminal").unwrap());
		
		let mut op = json!("{}");
		let mut ret_data = json!("{}");
		json_set!(&mut ret_data; "id"; 1);
		json_set!(&mut op; "ret"; ret_data);
		
		let table = db.get_table("customer").expect("customer table not exist.");
		let rst = table.save(&data, &op);
		
		let back_json = try!(rst);
		//save the st info
		let id = json_i64!(&back_json; "data", "0", "id");
		let table = db.get_table("st").expect("st table not exist.");
		let mut data = json!("{}");
		json_set!(&mut data; "id"; id);
		json_set!(&mut data; "last_active_time"; now.sec);
		let dyn_string = format!("{}{}", id, now.sec);
		json_set!(&mut data; "fix_st"; DigestUtil::md5(&dyn_string));
		json_set!(&mut data; "role"; CONS.code_to_id("user_type", "terminal").unwrap());
		let op = json!("{}");
		let rst = table.save(&data, &op);
		
        //新增终端机账户
		let rst = rst.and_then(|json| {
			let table = db.get_table("account").expect("account table not exist.");
			let mut data = json!("{}");
			json_set!(&mut data; "id"; id);
			let op = json!("{}");
			let rst = table.save(&data, &op);
			rst
		});
        //新增终端机详情表
        let rst = rst.and_then(|_| {
            let table = db.get_table("terminal").unwrap();
            let mut data = json!("{}");
			json_set!(&mut data; "id"; id);
			let op = json!("{}");
			let rst = table.save(&data, &op);
			rst
        });
		rst
    }
}

//admin get terminal list
pub struct AT02;

impl DataApi for AT02 {

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
        json_set!(&mut cond_json; "type"; CONS.code_to_id("user_type", "terminal").unwrap());

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
            let mut obj_mut = json.as_object_mut().unwrap();
            let mut data_json = obj_mut.get_mut("data").unwrap();
            let mut array = data_json.as_array_mut().unwrap();
            for mut terminal in array {
                let id = {
                    let id_node = terminal.find("id").unwrap();
                    id_node.as_i64().unwrap()
                };
                let terminal_obj = terminal.as_object_mut().unwrap();
                let bonus_len_rst = CacheService.get_bonus_length(id, db);
                if let Ok(len) = bonus_len_rst {
                    terminal_obj.insert("bonus_len".to_string(), len.to_json());
                } else {
                    terminal_obj.insert("bonus_len".to_string(), 0.to_json());
                }
            }
        }

        let mut in_len = 0;
        let mut in_data = json!("{}");
        {
            let mut id_array = Vec::new();
            let list = json_path!(&json; "data");
            let array = list.as_array().unwrap();
            in_len = array.len();
            for terminal in array {
                let id = json_i64!(terminal; "id");
                id_array.push(id.to_json());
            }
            json_set!(&mut in_data; "$in"; id_array);
        }

        if in_len > 0 {
            //加载额外信息
            let ext_rst = {
                let mut cond = json!("{}");
                json_set!(&mut cond; "id"; in_data);
            
                let table = db.get_table("terminal").unwrap();
                table.find(&cond, &json!("{}"), &json!("{}"))
            };
            let mut ext = try!(ext_rst);
            let mut ext_obj = ext.as_object_mut().unwrap();
            let ext_list = ext_obj.remove("data").unwrap();
            json_set!(&mut json; "ext"; ext_list);

            let mut cond = json!("{}");
            json_set!(&mut cond; "id"; in_data.clone());
            //加载额外信息
            let ext_rst = {
                let table = db.get_table("account").unwrap();
                table.find(&cond, &json!("{}"), &json!("{}"))
            };
            let mut ext = try!(ext_rst);
            let mut ext_obj = ext.as_object_mut().unwrap();
            let ext_list = ext_obj.remove("data").unwrap();
            json_set!(&mut json; "accounts"; ext_list);
        } else {
            json_set!(&mut json; "ext"; Vec::<Json>::new());
            json_set!(&mut json; "accounts"; Vec::<Json>::new());
        }

        //省份信息
    	json_set!(&mut json; "province_type"; CONS.get_json_obj("province_type"));
        //终端机的模式
    	json_set!(&mut json; "terminal_mode"; CONS.get_json_obj("terminal_mode"));

        Result::Ok(json)
    }
}

//admin get terminal detail info
pub struct AT03;

impl DataApi for AT03 {

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

		let terminal_id = json_i64!(msg; "body", "terminal_id");
        let mut cond = json!("{}");
        json_set!(&mut cond; "type"; CONS.code_to_id("user_type", "terminal").unwrap());
        json_set!(&mut cond; "id"; terminal_id);

        let mut op = json!("{}");
        
        let rst = table.find_one(&cond, &json!("{}"), &op);
        let rst = rst.and_then(|cus_back_json|{
			let mut cond = json!("{}");
			json_set!(&mut cond; "id"; terminal_id);
			
			let doc = json!("{}");
			let op = json!("{}");
			
			let table = db.get_table("st").expect("st table not exists.");
			let rst = table.find_one(&cond, &doc, &op);
			rst.and_then(|back_json|{
				let mut out_json = json!("{}");
				json_set!(&mut out_json; "terminal"; cus_back_json);
				json_set!(&mut out_json; "st"; back_json);
				Result::Ok(out_json)
			})
        });
        let mut back_body = try!(rst);

		let table = db.get_table("terminal").unwrap();
        let terminal_id = json_i64!(msg; "body", "terminal_id");
        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; terminal_id);

        let doc = json!("{}");
        let op = json!("{}");
            
        let ext = try!(table.find_one(&cond, &doc, &op));
        json_set!(&mut back_body; "ext"; ext);

        //终端机的模式
    	json_set!(&mut back_body; "terminal_mode"; CONS.get_json_obj("terminal_mode"));

        Result::Ok(back_body)
    }
}

//获得彩票机能出的游戏
pub struct AT04;

impl DataApi for AT04 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = KeyHelper::from_cache(db, head);
        KeyHelper::active(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let table = db.get_table("terminal_game").expect("terminal_game table not exists.");

		let terminal_id = json_i64!(msg; "body", "id");
        let mut cond = json!("{}");
        json_set!(&mut cond; "terminal_id"; terminal_id);
		
		let mut doc = json!("{}");
        let mut op = json!("{}");
        let sort = json!(r#"
            [
                {"id": 1}
            ] 
        "#);
        json_set!(&mut op; "sort"; sort); 
        
        let mut back_json = json!("{}");
        let rst = table.find(&cond, &doc, &op);
        let rst = rst.and_then(|json|{
			let game_list = GameService.get_game_list();
    		json_set!(&mut back_json;"game_list";game_list);
    		json_set!(&mut back_json;"terminal_game_list";json);
    		Result::Ok(back_json)
        });
        rst
    }
}

//添加彩票机能出的游戏
pub struct AT05;

impl DataApi for AT05 {

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
        let table = db.get_table("terminal_game").expect("terminal_game table not exists.");
		
		let terminal_id = json_i64!(msg; "body", "terminal_id");
		let game_id = json_i64!(msg; "body", "game_id");
		
		let mut conflict = json!("{}");
		json_set!(&mut conflict; "terminal_id"; 1);
		json_set!(&mut conflict; "game_id"; 1);
		
		let mut data = json!("{}");
		json_set!(&mut data; "game_id"; game_id);
		json_set!(&mut data; "terminal_id"; terminal_id);
		let now = time::get_time();
		json_set!(&mut data; "create_time"; now.sec);
		
		let mut up_data = json!("{}");
		let mut set_data = json!("{}");
		json_set!(&mut set_data; "create_time"; now.sec);
		json_set!(&mut up_data; "$set"; set_data);
		
		let op = json!("{}");
		
		let rst = table.upsert(&conflict, &data, &up_data, &op);
		
		rst
    }
}

//删除彩票机游戏
pub struct AT06;

impl DataApi for AT06 {

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
        let table = db.get_table("terminal_game").expect("terminal_game table not exists.");
		
		let terminal_game_id = json_i64!(msg; "body", "id");
		let mut cond = json!("{}");
		json_set!(&mut cond; "id"; terminal_game_id);
		
		let op = json!("{}");
		let rst = table.remove(&cond, &op);
		rst
    }
}

//获得系统模式
pub struct AT07;

impl DataApi for AT07 {

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
        let mut back_json = json!("{}");
    	json_set!(&mut back_json;"sys_mode";CONS.get_json_obj("sys_mode"));

        let cur_mode = CacheService.get_terminal_mode(db).unwrap();
    	json_set!(&mut back_json;"cur_mode";cur_mode);

        let resend = CacheService.resend_ticket_when_err(db);
        if resend {
    	    json_set!(&mut back_json; "resend"; 1);
        } else {
    	    json_set!(&mut back_json; "resend"; 0);
        }

        let cur_gap = CacheService.get_print_gap(db);
    	json_set!(&mut back_json;"cur_gap";cur_gap);

        Result::Ok(back_json)
    }
}

//设置系统模式
pub struct AT08;

impl DataApi for AT08 {

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
        let mode = json_i64!(msg; "body", "mode");
        CacheService.set_terminal_mode(mode, db).unwrap();

        let resend = json_i64!(msg; "body", "resend");
        let resend_bool;
        if resend > 0 {
            resend_bool = true;
        } else {
            resend_bool = false;
        }
        let _ = CacheService.set_resend_ticket_when_err(resend_bool, db);

        let back_json = json!("{}");
        Result::Ok(back_json)
    }
}

//设置出票间隔
pub struct AT09;

impl DataApi for AT09 {

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
        let gap = json_i64!(msg; "body", "gap");
        CacheService.set_print_gap(gap, db).unwrap();
        let back_json = json!("{}");
        Result::Ok(back_json)
    }
}

//把彩票机设置成管理机
pub struct AT10;

impl DataApi for AT10 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = KeyHelper::from_cache(db, head);
        KeyHelper::active(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let terminal_id = json_i64!(msg; "body", "id");
        let terminal_type = json_i64!(msg; "body", "type");
        let table = db.get_table("terminal").unwrap();
        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; terminal_id);

        let mut doc = json!("{}");
        let mut set_data = json!("{}");
        json_set!(&mut set_data; "type"; terminal_type);
        json_set!(&mut doc; "$set"; set_data);
        
        let op = json!("{}");

        let rst = table.update(&cond, &doc, &op);
        rst
    }
}

//更新游戏期次信息
pub struct AT11;

impl DataApi for AT11 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = KeyHelper::from_cache(db, head);
        KeyHelper::active(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let game_id = json_i64!(msg; "body", "game_id");

        let mut msg = json!("{}");
        json_set!(&mut msg; "cmd"; "T04");
        let mut body = json!("{}");
        json_set!(&mut body; "game_id"; game_id);
        json_set!(&mut body; "type"; 0);
        json_set!(&mut msg; "body"; body);
        let _ = CacheService.send_query_msg(&msg, &db);
        Result::Ok(json!("{}"))
    }
}

//设置彩票机出票间隔
pub struct AT12;

impl DataApi for AT12 {

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
        let table = db.get_table("terminal").unwrap();
		let cond = json_path!(msg; "body", "cond");
		let doc = json_path!(msg; "body", "doc");
        let rst = table.update(cond, doc, &json!("{}"));
        let _ = try!(rst);


        let group_id = json_i64!(msg; "body", "group_id");
        let table = db.get_table("customer").unwrap();
        let mut doc = json!("{}");
        let mut set_data = json!("{}");
        json_set!(&mut set_data; "group_id"; group_id);
        json_set!(&mut doc; "$set"; set_data);
        let rst = table.update(cond, &doc, &json!("{}"));
        rst
    }
}

//设置彩票机游戏返点
pub struct AT13;

impl DataApi for AT13 {

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
        let data = json_path!(msg; "body", "data");
        let id = json_i64!(data; "id");
        let scale = json_i64!(data; "scale");

        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; id);

        let mut set_data = json!("{}");
        json_set!(&mut set_data; "scale"; scale);
        let mut doc = json!("{}");
        json_set!(&mut doc; "$set"; set_data);

        let op = json!("{}");

        let table = db.get_table("terminal_game").unwrap();
        let rst = table.update(&cond, &doc, &op);
        rst
        //return Result::Ok(json!("{}"));
    }
}

