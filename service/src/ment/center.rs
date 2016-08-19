extern crate util;
use util::*;

use super::super::dc::DataBase;
use super::super::dc::MyDbPool;
use super::super::cons::CONS;
use super::super::cons::ErrCode;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::Read;

extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;
use std::str::FromStr;

extern crate regex;
use self::regex::Regex;

extern crate time;

use super::super::inter::{DataApi};
use super::super::sv_util::{KeyHelper, PageHelper};

extern crate sev_helper;
use sev_helper::PrintService;
use sev_helper::CustomerService;
use sev_helper::CacheService;
use sev_helper::TicketService;

struct Helper;

impl Helper {
	
	fn get_key(db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
		let rst = {
            let head_node = head.find("userType");
			let user_type = json_str!(head; "userType");
	   		if user_type == "center" {
	   			Result::Ok(1)
	   		} else {
	   			Result::Err(ErrCode::NotAllowed as i32)	
	   		}
		};
		let rst = rst.and_then(|_|{
			let rst = KeyHelper::from_db_by_id(db, head);
            rst
		});
		rst
	}
}

//center get terminal list
pub struct CRT01;

impl DataApi for CRT01 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        Helper::get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let customer_id = json_i64!(msg; "head", "userId");
        let body = json_path!(msg; "body");
        let table = db.get_table("customer").expect("customer table not exists.");
        let mut cond_json = PageHelper::get_cond(body);
        json_set!(&mut cond_json; "type"; CONS.code_to_id("user_type", "terminal").unwrap());
        json_set!(&mut cond_json; "group_id"; customer_id);

        let op = PageHelper::get_op(body);

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

        //如果列表为空
        if count == 0 {
            json_set!(&mut json; "ext"; Vec::<Json>::new());
            json_set!(&mut json; "games"; Vec::<Json>::new());
            return Result::Ok(json);
        }

        let mut in_data = PageHelper::get_in_data(&json, "id");

        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; in_data.clone());
        //加载额外信息
        let ac_rst = {
            let table = db.get_table("account").unwrap();
            table.find(&cond, &json!("{}"), &json!("{}"))
        };
        let ac = try!(ac_rst);
        let list = json_path!(&ac; "data");
        let array = list.as_array().unwrap();
        let mut ac_map = HashMap::new();
        for set in array {
            let terminal_id = json_i64!(set; "id"); 
            let balance = json_i64!(set; "balance"); 
            let client_balance = json_i64!(set; "client_balance"); 

            let mut ext = json!("{}");
            json_set!(&mut ext; "balance"; balance);
            json_set!(&mut ext; "client_balance"; client_balance);
            ac_map.insert(terminal_id, ext);
        }

        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; in_data.clone());
        //加载额外信息
        let ext_rst = {
            let table = db.get_table("terminal").unwrap();
            table.find(&cond, &json!("{}"), &json!("{}"))
        };
        let ext = try!(ext_rst);
        let ext_list = json_path!(&ext; "data");
        let ext_array = ext_list.as_array().unwrap();
        let mut ext_map = HashMap::new();
        for set in ext_array {
            let terminal_id = json_i64!(set; "id"); 
            let hard_type = json_i64!(set; "hard_type"); 
            let soft_type = json_i64!(set; "soft_type"); 
            let t_type = json_i64!(set; "type"); 
            let status = json_i64!(set; "status"); 
            let server_balance = json_i64!(set; "server_balance"); 
            let client_balance = json_i64!(set; "client_balance"); 

            let mut ext = json!("{}");
            json_set!(&mut ext; "server_balance"; server_balance);
            json_set!(&mut ext; "client_balance"; client_balance);
            json_set!(&mut ext; "hard_type"; hard_type);
            json_set!(&mut ext; "soft_type"; soft_type);
            json_set!(&mut ext; "type"; t_type);
            json_set!(&mut ext; "status"; status);
            ext_map.insert(terminal_id, ext);
        }
        
        let mut cond = json!("{}");
        json_set!(&mut cond; "terminal_id"; in_data);
        //加载游戏信息
        let game_rst = {
            let table = db.get_table("terminal_game").unwrap();
            table.find(&cond, &json!("{}"), &json!("{}"))
        };
        let games = try!(game_rst);
        let data = json_path!(&games; "data");
        let list = data.as_array().unwrap();
        let mut game_map = HashMap::<i64, Vec<Json>>::new();
        for set in list {
            let terminal_id = json_i64!(set; "terminal_id"); 
            let game_id = json_i64!(set; "game_id"); 
            let mut game = json!("{}");
            json_set!(&mut game; "game_id"; game_id);

            if game_map.contains_key(&terminal_id) {
                let mut vec = game_map.get_mut(&terminal_id).unwrap();
                vec.push(game);
            } else {
                let mut vec = Vec::new();
                vec.push(game);
                game_map.insert(terminal_id, vec);
            }
        }

        {
            let mut json_mut = json.as_object_mut().unwrap();
            let mut data = json_mut.get_mut("data").unwrap();
            let mut data_array = data.as_array_mut().unwrap();
            let len = data_array.len();
            for i in 0..len {
                let mut set = data_array.get_mut(i).unwrap();
                let id = {
                    let id_node = set.find("id").unwrap();
                    let id = id_node.as_i64().unwrap();
                    id
                };
                let mut set_obj = set.as_object_mut().unwrap();
                
                let ac_op = ac_map.remove(&id);
                if let Some(ac) = ac_op {
                    set_obj.insert("account".to_string(), ac);
                }

                let ext_op = ext_map.remove(&id);
                if let Some(ext) = ext_op {
                    set_obj.insert("ext".to_string(), ext);
                }

                let game_op = game_map.remove(&id);
                if let Some(games) = game_op {
                    set_obj.insert("games".to_string(), games.to_json());
                } else {
                    let games = Vec::<Json>::new();
                    set_obj.insert("games".to_string(), games.to_json());
                }
            }
        }
        Result::Ok(json)
    }
}

//center add terminal
pub struct CRT02;

impl DataApi for CRT02 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        Helper::get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        //println!("{}", msg);
        let customer_id = json_i64!(msg; "head", "userId");
        let data = json_path!(msg; "body", "data");
        let hard_type = json_path!(data; "hard_type");
        let soft_type = json_path!(data; "soft_type");

        let mut data = data.clone();
        let now = time::get_time();
        json_set!(&mut data; "reg_time"; now.sec);
        json_set!(&mut data; "group_id"; customer_id);
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
			json_set!(&mut data; "hard_type"; hard_type);
			json_set!(&mut data; "soft_type"; soft_type);
			let op = json!("{}");
			let rst = table.save(&data, &op);
			rst
        });
		rst
    }
}


//center query terminal's charget report 
pub struct CRT03;

impl DataApi for CRT03 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        Helper::get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let customer_id = json_i64!(msg; "head", "userId");
        let terminal_id = json_i64!(msg; "body", "terminal_id");

        let mut msg = json!("{}");
        json_set!(&mut msg; "cmd"; "T05");
        let mut body = json!("{}");
        json_set!(&mut body; "type"; 0);
        json_set!(&mut msg; "body"; body);
        let rst = CacheService.send_terminal_query_msg(terminal_id, &msg, &db);
        try!(rst);

        Result::Ok(json!("{}"))
    }
}

//出票中心登录
pub struct CR01;

impl DataApi for CR01 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
		let rst = {
			let user_type = json_str!(head; "userType");
	    	if user_type == "center" {
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
        let mut cond = json!("{}");
        json_set!(&mut cond; "username"; username);
        let doc = json!("{}");
        let op = json!("{}");
        let fd_back = try!(table.find(&cond, &doc, &op));
        let user_id = json_i64!(&fd_back; "data", "0", "id");
        
        let mut back_body = json!("{}");
        json_set!(&mut back_body; "userId"; user_id);
        json_set!(&mut back_body; "st"; key);
        
        let rst = Result::Ok(back_body);
		rst
    }
}

//出票中心获得系统常量
pub struct CR02;

impl DataApi for CR02 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
	    Helper::get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let mut back_body = json!("{}");
    	json_set!(&mut back_body; "hard_type"; CONS.get_json_obj("terminal_hard_type"));
    	json_set!(&mut back_body; "soft_type"; CONS.get_json_obj("terminal_soft_type"));
        let rst = Result::Ok(back_body);
		rst
    }
}

//出票中心获得兑奖异常票
pub struct CR03;

impl DataApi for CR03 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
	    Helper::get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let group_id = json_i64!(msg; "head", "userId");
        /*
        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; group_id);
        let doc = json!("{}");
        let op = json!("{}");
        let table = db.get_table("customer").unwrap();
        let rst = table.find_one(&cond, &doc, &op);
        let center = try!(rst);
        let province = json_i64!(&center; "province");
        */

        let table = db.get_table("customer").unwrap();
        let mut cond_json = json!("{}");
        json_set!(&mut cond_json; "type"; CONS.code_to_id("user_type", "terminal").unwrap());
        json_set!(&mut cond_json; "group_id"; group_id);

        let mut doc = json!("{}");
        json_set!(&mut doc; "id"; 1);

        let op = json!("{}");

        let mut ids_list = Vec::new();
        let rst = table.find(&cond_json, &doc, &op);
        let rst_data = try!(rst);
        let data = rst_data.find("data").unwrap();
        let list = data.as_array().unwrap();
        for terminal in list {
            let id_node = terminal.find("id").unwrap();
            let id = id_node.as_i64().unwrap();
            ids_list.push(id.to_json());
        }
        if ids_list.len() == 0 {
            let mut back_body = json!("{}");
            json_set!(&mut back_body; "tickets"; Vec::<Json>::new());
            let rst = Result::Ok(back_body);
            return rst;
        }

        let mut in_data = json!("{}");
        json_set!(&mut in_data; "$in"; ids_list);

        let mut cond = json!("{}");
        json_set!(&mut cond; "status"; CONS.code_to_id("ticket_status", "bonus_err").unwrap());
        json_set!(&mut cond; "terminal_id"; in_data);
        let mut gt_data = json!("{}");
        json_set!(&mut gt_data; "$gt"; 2);
        json_set!(&mut cond; "bonus_try_count"; gt_data);

        let mut doc = json!("{}");
        json_set!(&mut doc; "id"; 1);
        json_set!(&mut doc; "bonus"; 1);
        json_set!(&mut doc; "bonus_after_tax"; 1);
        json_set!(&mut doc; "seq"; 1);

        let mut op = json!("{}");
        json_set!(&mut op; "limit"; 500);

        let table = db.get_table("ticket").unwrap();
        let rst = table.find(&cond, &doc, &op);
        let mut rst_data = try!(rst);
        let mut rst_data_obj = rst_data.as_object_mut().unwrap();
        let data = rst_data_obj.remove("data").unwrap();

        let mut back_body = json!("{}");
        json_set!(&mut back_body; "tickets"; data);
        let rst = Result::Ok(back_body);
		rst
    }
}

///返回兑奖异常票处理结果
///status=1，已兑奖，status=2，兑奖成功，status=3，中大奖，status=4，未中奖
pub struct CR04;

impl DataApi for CR04 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
	    Helper::get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        //info!("{}", msg);
        //let group_id = json_i64!(msg; "head", "userId");
        let status = json_i64!(msg; "body", "status");
        let id = json_i64!(msg; "body", "id");
        let bonus = json_i64!(msg; "body", "bonus");
        match status {
            1 => {
                TicketService.bonused(id, db);
            },
            2 => {
                let terminal = json_str!(msg; "body", "terminal");
                let customer_rst = CustomerService.get_by_username(terminal, db);
                let customer = try!(customer_rst);
                let terminal_id = json_i64!(&customer; "id");
                TicketService.bonus_success(id, terminal_id, bonus, "fix", db);
            },
            3 => {
                TicketService.bonus_big(id, db);
            },
            _ => {

            },
        }
        let mut back_body = json!("{}");
        let rst = Result::Ok(back_body);
		rst
    }
}

///返回出票异常票票根
pub struct CR05;

impl DataApi for CR05 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
	    Helper::get_key(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);

        let body = json_path!(msg; "body");
        let terminal_id = json_str!(body; "terminal_id");
        let terminal = CustomerService.get_by_username(&terminal_id, db).unwrap();
        let id = json_i64!(&terminal; "id");

        let rst = PrintService.back(id, body, db);
        let _ = try!(rst);
        Result::Ok(json!("{}"))
    }
}

///根据票的id获取票根信息
pub struct CR06;

impl DataApi for CR06 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
	    Helper::get_key(db, head)
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
        let sql = format!("select * from ticket where id={}", id);
        let rst = try!(db.execute(&sql));
        let ticket = get_first_data(rst);

        let mut back = json();
        json_set!(&mut back; "ticket"; ticket);
        Result::Ok(back)
    }
}
