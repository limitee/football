extern crate util;
use util::{DigestUtil, json};

extern crate dc;
use dc::DataBase;
use dc::MyDbPool;

extern crate cons;
use cons::CONS;
use cons::ErrCode;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::Read;
use std::ops::Sub;

#[macro_use]
extern crate easy_util;
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

extern crate game;
use game::base::GF;
use game::PF;

#[macro_use]
extern crate log;
extern crate elog;

extern crate redis;
use redis::Commands;
use redis::RedisResult;

pub struct GameService;

impl GameService {
	
	pub fn get_game_list(&self) -> Vec<Json> {
		let game_list = GF.get_game_list();
        let mut list = Vec::<Json>::new();
        for (_, value) in game_list.iter() {
        	let encoded = json::encode(value).unwrap();
        	list.push(json!(&encoded));
        }
        list
	}
}

pub struct TicketService;

impl TicketService {

    /**
     * 已经发送的票据，重新获取机会
     */
    pub fn back_to_wait_list(&self, id:i64, version:i64, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
        let sql = format!("update print_pool set status=0 where id={} and status=1 and version={}", id, version);
        db.execute(&sql)
    }

    /**
     * 把目标{id}票据移出出票池
     */
    pub fn out_print_pool(&self, id:i64, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
        let sql = format!("delete from print_pool where id={}", id);
        let rst = db.execute(&sql);
        rst
    }

    /**
     * 进入出票池
     */
    pub fn in_print_pool(&self, ticket:&Json, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
        let table = db.get_table("print_pool").unwrap();
        let op = json!("{}");
        let rst = table.save(ticket, &op);
        rst
    }

    /**
     * 根据id获得票据
     */
    pub fn get_by_id(&self, id:i64, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
        let mut cond = json!("{}");
		json_set!(&mut cond; "id"; id);
        
        let doc = json!("{}");
		let op = json!("{}");
		let table = db.get_table("ticket").unwrap();
        let ticket_rst = table.find_one(&cond, &doc, &op);
        ticket_rst
    }

    /**
     * 根据id获得票据的状态
     */
    pub fn get_status_by_id(&self, id:i64, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
        let mut cond = json!("{}");
		json_set!(&mut cond; "id"; id);
        
        let mut doc = json!("{}");
		json_set!(&mut doc; "status"; 1);

		let op = json!("{}");
		let table = db.get_table("ticket").unwrap();
        let ticket_rst = table.find_one(&cond, &doc, &op);
        let ticket = try!(ticket_rst);

        let status_node = ticket.find("status").unwrap();
        Result::Ok(status_node.as_i64().unwrap() as i32)
    }
	
	/**
	 * 退款
	 */
	pub fn refund(&self, id:i64, customer_id:i64, amount:i64, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
        let mut cond = json!("{}");
		json_set!(&mut cond; "id"; id);
        let mut or_data = Vec::<Json>::new();
        let mut obj = json!("{}");
		json_set!(&mut obj; "status"; CONS.code_to_id("ticket_status", "printing").unwrap());
        or_data.push(obj);
        let mut obj = json!("{}");
		json_set!(&mut obj; "status"; CONS.code_to_id("ticket_status", "print_err").unwrap());
        or_data.push(obj);
		json_set!(&mut cond; "$or"; or_data);
		
		let mut doc = json!("{}");
		let mut set_data = json!("{}");
		json_set!(&mut set_data; "status"; CONS.code_to_id("ticket_status", "refund").unwrap());
		json_set!(&mut doc; "$set"; set_data);
		
		let mut op = json!("{}");
		let mut ret_data = json!("{}");
		json_set!(&mut ret_data; "id"; 1);
		json_set!(&mut op; "ret"; ret_data);
		
		let table = db.get_table("ticket").unwrap();
		let rst = table.update(&cond, &doc, &op);
        let rst_data = try!(rst);

        let rows = json_i64!(&rst_data; "rows");
        if rows > 0 {
            //移出出票池
            let _ = TicketService.out_print_pool(id, db);

            let order_id = format!("{}", id);
		    //返款
		    let rst = AccountService.handle(customer_id, &order_id, CONS.code_to_id("moneylog_type", "refund").unwrap(), 
			    amount, db
		    );
            rst
        } else {
            Result::Err(ErrCode::DataExpired as i32)
        }
	}

    ///竞彩算奖前的校验，是否所有的场次已经开奖
    pub fn jc_check(&self, term_code:i64, dn:&str, ticket:&Json, db:&DataBase<MyDbPool>) -> Result<String, i32> {
        let term_code_string = term_code.to_string();
        let code_str = term_code_string.as_str();
        //判定是否包含目标场次
        let rst = {
            let term_code_list = json_str!(ticket; "term_code_list");
            if term_code_list.contains(code_str) {
                Result::Ok(1) 
            } else {
                Result::Err(1)
            }
        };
        //是否已经有开奖信息
        let rst = rst.and_then(|_|{
            let draw_code_list = json_str!(ticket; "draw_code_list");
            if !draw_code_list.contains(code_str) {
                Result::Ok(draw_code_list.to_string())
            } else {
                Result::Err(1)
            }
        });
        //生成新的draw_code_list
        let rst = rst.and_then(|mut draw_code_list|{
            if draw_code_list.len() > 0 {
                draw_code_list.push_str(";"); 
            }
            let term_string = format!("{}:{}", term_code, dn);
            draw_code_list.push_str(term_string.as_str());
            Result::Ok(draw_code_list)
        });
        //更新数据库字段
        let rst = rst.and_then(|draw_code_list|{
		    let table = db.get_table("ticket").unwrap();
            let id = json_i64!(ticket; "id");
            let mut cond = json!("{}");
            json_set!(&mut cond; "id"; id);

            let mut doc = json!("{}");
            let mut set_data = json!("{}");
            json_set!(&mut set_data; "draw_code_list"; draw_code_list);
            json_set!(&mut doc; "$set"; set_data);
            let op = json!("{}");
		    let _ = table.update(&cond, &doc, &op);
            Result::Ok(draw_code_list)
        });
        rst
    }
	
	/**
	 * 算奖结果返回数据库
	 */
	pub fn draw(&self, id:i64, multiple:i64, rst:&Json, 
        game_id:i64, draw_number:&str, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
		let mut cond = json!("{}");
		json_set!(&mut cond; "id"; id);
		
        let bonus;
        let bonus_node = rst.find("bonus").unwrap();
        if bonus_node.is_f64() {
            let um_bonus = bonus_node.as_f64().unwrap(); 
            bonus = (um_bonus.round() as i64)*multiple;
        } else {
            let um_bonus = bonus_node.as_i64().unwrap(); 
            bonus = um_bonus*multiple;
        }

        let bonus_after_tax;
        let bat_node = rst.find("bonus_after_tax").unwrap();
        if bat_node.is_f64() {
            let um_bat = bat_node.as_f64().unwrap(); 
            bonus_after_tax = (um_bat.round() as i64)*multiple;
        } else {
            let um_bat = bat_node.as_i64().unwrap(); 
            bonus_after_tax = um_bat*multiple;
        }

		let detail = json_path!(rst; "detail");
		
		let mut doc = json!("{}");
		let mut set_data = json!("{}");
		if bonus > 0 {
            let status;
            if bonus >= 50000000 {  //大于50W，中超级大奖
                status = CONS.code_to_id("ticket_status", "bonus_super").unwrap();
            } else {
                status = CONS.code_to_id("ticket_status", "hit").unwrap();
            }
			json_set!(&mut set_data; "status"; status);	
			json_set!(&mut set_data; "bonus"; bonus);
			json_set!(&mut set_data; "bonus_after_tax"; bonus_after_tax);
		} else {
			json_set!(&mut set_data; "status"; CONS.code_to_id("ticket_status", "not_hit").unwrap());
		}
        if game_id != 201 && game_id != 301 { //非竞彩串关，更新开奖号码
		    json_set!(&mut set_data; "draw_code_list"; draw_number);
        }
		json_set!(&mut set_data; "bonus_detail"; detail.to_string());
		json_set!(&mut doc; "$set"; set_data);
		
		let op = json!("{}");
		
		let table = db.get_table("ticket").unwrap();
		table.update(&cond, &doc, &op)
	}
	
    ///返奖
	pub fn bonus(&self, ticket:&Json, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
        let id = json_i64!(ticket; "id");
        let customer_id = json_i64!(ticket; "customer_id");
        let terminal_id = json_i64!(ticket; "terminal_id");
        let bonus = json_i64!(ticket; "bonus");
        let amount = json_i64!(ticket; "bonus_after_tax");
        let seq = json_str!(ticket; "seq");
        let order_id = format!("{}", id);
		let rst = AccountService.handle(customer_id, &order_id, CONS.code_to_id("moneylog_type", "fund").unwrap(), 
			amount, db
		);
		let _ = try!(rst);
		
        //更新票据状态
		let mut cond = json!("{}");
		json_set!(&mut cond; "id"; id);
		
		let mut doc = json!("{}");
		let mut set_data = json!("{}");
        if bonus >= 1000000 {
		    json_set!(&mut set_data; "status"; CONS.code_to_id("ticket_status", "bonus_big").unwrap());
        } else {
		    json_set!(&mut set_data; "status"; CONS.code_to_id("ticket_status", "funded").unwrap());
        }
		json_set!(&mut doc; "$set"; set_data);
		
		let op = json!("{}");
		
		let table = db.get_table("ticket").unwrap();
		let rst = table.update(&cond, &doc, &op);
        let _ = try!(rst);

        //小奖加入兑奖队列
        if bonus < 1000000 {
            let mut ticket = json!("{}");  
		    json_set!(&mut ticket; "id"; id);
		    json_set!(&mut ticket; "bonus"; bonus);
		    json_set!(&mut ticket; "bonus_after_tax"; amount);
		    json_set!(&mut ticket; "seq"; seq);
            CacheService.bonus(terminal_id, &ticket, db)
        } else {
            Result::Ok(1) 
        }
    }

    //重新兑奖
    pub fn rebonus(&self, id:i64, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
        let mut cond = json!("{}");
		json_set!(&mut cond; "id"; id);
        let status = CONS.code_to_id("ticket_status", "bonus_err").unwrap();
		json_set!(&mut cond; "status"; status);
		
		let mut doc = json!("{}");
        let mut set_data = json!("{}");
        let target_status = CONS.code_to_id("ticket_status", "funded").unwrap();
		json_set!(&mut set_data; "status"; target_status);
		json_set!(&mut doc; "$set"; set_data);
		
		let mut op = json!("{}");
        let mut ret_data = json!("{}");
        json_set!(&mut ret_data; "id"; 1); 
        json_set!(&mut ret_data; "bonus"; 1); 
        json_set!(&mut ret_data; "bonus_after_tax"; 1); 
        json_set!(&mut ret_data; "seq"; 1); 
        json_set!(&mut ret_data; "terminal_id"; 1); 
        json_set!(&mut op; "ret"; ret_data); 

        let table = db.get_table("ticket").unwrap();
        let rst = table.update(&cond, &doc, &op);
        let mut back_json = try!(rst);
        info!("{}", back_json);

        let mut back_obj = back_json.as_object_mut().unwrap();
        let rows = {
            let node = back_obj.remove("rows").unwrap();
            node.as_i64().unwrap()
        };
        if rows <= 0 {
            return Result::Err(ErrCode::DataExpired as i32);
        }
        let mut ticket = {
            let mut node = back_obj.remove("data").unwrap();
            let mut array = node.as_array_mut().unwrap();
            array.remove(0)
        };
        let terminal_id = {
            let mut obj = ticket.as_object_mut().unwrap();
            let node = obj.remove("terminal_id").unwrap(); 
            node.as_i64().unwrap()
        };
        let _ = CacheService.bonus(terminal_id, &ticket, db);
        return Result::Ok(1);
    }

    //标识票据已经兑奖成功
    pub fn bonused(&self, id:i64, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
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
        rst
    }

    //中大奖
    pub fn bonus_big(&self, id:i64, db:&DataBase<MyDbPool>) -> Result<i32, i32> { 
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
        return Result::Ok(1);
    }

    //兑奖成功
    pub fn bonus_success(&self, id:i64, terminal_id:i64, bonus:i64, stub:&str, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
        //更新票据状态
		let mut cond = json!("{}");
		json_set!(&mut cond; "id"; id);
        let mut or_data = Vec::<Json>::new();
        let mut obj = json!("{}");
		json_set!(&mut obj; "status"; CONS.code_to_id("ticket_status", "funded").unwrap());
        or_data.push(obj);
        let mut obj = json!("{}");
		json_set!(&mut obj; "status"; CONS.code_to_id("ticket_status", "bonus_err").unwrap());
        or_data.push(obj);
		json_set!(&mut cond; "$or"; or_data);
		
		let mut doc = json!("{}");
		let mut set_data = json!("{}");
		json_set!(&mut set_data; "status"; CONS.code_to_id("ticket_status", "cashed").unwrap());
		json_set!(&mut set_data; "bonus_stub"; stub);
		json_set!(&mut set_data; "bonus_terminal_id"; terminal_id);
        let now = Local::now();
		json_set!(&mut set_data; "bonus_time"; now.timestamp());
		json_set!(&mut doc; "$set"; set_data);
		
		let mut op = json!("{}");
        let mut ret_data = json!("{}");
		json_set!(&mut ret_data; "id"; 1);
		json_set!(&mut op; "ret"; ret_data);
		
		let table = db.get_table("ticket").unwrap();
		let rst = table.update(&cond, &doc, &op);
        let up_data = try!(rst);
        let rows = json_i64!(&up_data; "rows");

        if rows > 0 {
            let order_id = format!("{}", id);
            let rst = AccountService.handle(terminal_id, &order_id, CONS.code_to_id("moneylog_type", "cash").unwrap(), 
			    bonus, db
		    );
		    let _ = try!(rst);
            let rst = AccountService.handle_extra(terminal_id, bonus, db);
            let _ = try!(rst);
        }
        return Result::Ok(1);
    }
    
    /**
     * 返回兑奖结果，status不传，及不等于2,都认为兑奖成功，否则为兑奖失败
     */
	pub fn cash(&self, terminal_id:i64, body:&Json, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
        let mut success = false;
        let status_node_op = body.find("status");
        if let Some(status_node) = status_node_op {
            let status = status_node.as_i64().unwrap();
            if status != 2 {
                success = true;
            }
        } else {
            success = true;
        }
        let ticket = json_path!(body; "ticket");
        if success {
            let id = json_i64!(ticket; "id");
            let stub = json_str!(ticket; "stub");
            let bonus = json_i64!(ticket; "bonus");
	        self.bonus_success(id, terminal_id, bonus, stub, db)	
        } else {
            let rst = PrintService.bonus_err(terminal_id, ticket, db);
            let _ = try!(rst);
            Result::Ok(1)
        }
    }

    //通过id查找
	pub fn find_by_id(&self, id:i64, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
        let mut cond = json!("{}");
		json_set!(&mut cond; "id"; id);

		let table = db.get_table("ticket").unwrap();
		let doc = json!("{}");
		let op = json!("{}");
		table.find_one(&cond, &doc, &op)
    }

}

pub struct TerminalService;

impl TerminalService {

	//所有终端机充值为离线状态
	pub fn all_offline(&self, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
        let table = db.get_table("terminal").unwrap();
        let cond = json!("{}");
        let mut doc = json!("{}");
        let mut set_data = json!("{}");
        json_set!(&mut set_data; "status"; CONS.code_to_id("terminal_status", "offline").unwrap());
        json_set!(&mut doc; "$set"; set_data);
        let op = json!("{}");
        table.update(&cond, &doc, &op)
	}

    pub fn get_account_by_id(&self, id:i64, 
        db:&DataBase<MyDbPool>) -> Result<Json, i32> {
        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; id);

        let doc = json!("{}");
        let op = json!("{}");

        let table = db.get_table("account").unwrap();
        table.find_one(&cond, &doc, &op)
    }

    pub fn set_out_balance(&self, id:i64, out_balance:i64, 
        db:&DataBase<MyDbPool>) -> Result<Json, i32> {
        let account_rst = self.get_account_by_id(id, db);
        let account = try!(account_rst);
        let server_balance = json_i64!(&account; "balance");

        let table = db.get_table("terminal").unwrap();
        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; id);
        
        let mut doc = json!("{}");
        let mut set_data = json!("{}");
        json_set!(&mut set_data; "server_balance"; server_balance);
        json_set!(&mut set_data; "client_balance"; out_balance);
        json_set!(&mut doc; "$set"; set_data);
        let op = json!("{}");
        let rst = table.update(&cond, &doc, &op);

        let _ = try!(rst);

        //设置account的副帐号余额
        let table = db.get_table("account").unwrap();
        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; id);

        let mut doc = json!("{}");
        let mut set_data = json!("{}");
        json_set!(&mut set_data; "client_balance"; out_balance);
        json_set!(&mut doc; "$set"; set_data);

        let op = json!("{}");
        let rst = table.update(&cond, &doc, &op);
        rst
    }

    //获得终端机出票间隔
    pub fn get_print_gap(&self, id:i64, db:&DataBase<MyDbPool>) -> Result<i64, i32> {
        let table = db.get_table("terminal").unwrap();
        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; id);

        let mut doc = json!("{}");
        json_set!(&mut doc; "print_gap"; 1);

        let op = json!("{}");
        let rst = table.find_one(&cond, &doc, &op);
        let data = try!(rst);

        let gap = json_i64!(&data; "print_gap");
        return Result::Ok(gap);
    }
}

pub struct TermService;

impl TermService {

	//通过id查找
	pub fn find_by_id(&self, id:i64, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
        let mut cond = json!("{}");
		json_set!(&mut cond; "id"; id);

		let table = db.get_table("term").unwrap();
		let doc = json!("{}");
		let op = json!("{}");
		table.find_one(&cond, &doc, &op)
    }

	/**
	 * 查找需要开售的期次，如果无，返回null，如果有，返回期次信息
	 */
	pub fn find_to_sale(&self, db:&DataBase<MyDbPool>) -> Option<Json> {
		let mut cond = json!("{}");
		let timestamp = Local::now().timestamp();
		json_set!(&mut cond; "status"; CONS.code_to_id("term_status", "init").unwrap());
		
		let mut lt_data = json!("{}");
		json_set!(&mut lt_data; "$lt"; timestamp);
		json_set!(&mut cond; "sale_time"; lt_data);
		
		let doc = json!("{}");
		let op = json!("{}");
		
		let table = db.get_table("term").expect("term table not exist.");
		let rst = table.find(&cond, &doc, &op);
		let rst = rst.and_then(|back_json|{
			//info!("{}", back_json);
			let rows = json_i64!(&back_json; "rows");
			if rows > 0 {
				let data = json_path!(&back_json; "data", "0");
				Result::Ok(data.clone())
			} else {
				info!("no term need to sale.");
				Result::Err(-1)
			}
		});
		let rst = rst.and_then(|term| {
			
			let id = json_i64!(&term; "id");
			let version = json_i64!(&term; "version");
			
			let mut cond = json!("{}");
			json_set!(&mut cond; "id"; id);
			json_set!(&mut cond; "version"; version);
			
			let mut doc = json!("{}");
			let mut set_data = json!("{}");
			json_set!(&mut set_data; "status"; CONS.code_to_id("term_status", "ready_to_sale").unwrap());
			json_set!(&mut doc; "$set"; set_data);
			
			let mut op = json!("{}");
			let mut ret_data = json!("{}");
			json_set!(&mut ret_data; "id"; 1);
			json_set!(&mut ret_data; "game_id"; 1);
			json_set!(&mut ret_data; "code"; 1);
			json_set!(&mut ret_data; "status"; 1);
			json_set!(&mut op; "ret"; ret_data);
			
			let rst = table.update(&cond, &doc, &op);
			rst
		});
		let rst = rst.and_then(|back_json|{
			let rows = json_i64!(&back_json; "rows");
			if(rows > 0) {
				let data = json_path!(&back_json; "data", "0");
				Result::Ok(data.clone())
			} else {
				info!("no terms to handle...");
				Result::Err(-1)
			}
		});
		rst.ok()
	}
	
	///查找需要停售的期次
	pub fn find_to_end(&self, db:&DataBase<MyDbPool>) -> Option<Json> {
		let mut cond = json!("{}");
		let timestamp = Local::now().timestamp();
		json_set!(&mut cond; "status"; CONS.code_to_id("term_status", "sale").unwrap());
		
		let mut lt_data = json!("{}");
		json_set!(&mut lt_data; "$lt"; timestamp);
		json_set!(&mut cond; "end_time"; lt_data);
		
		let doc = json!("{}");
		let op = json!("{}");
		
		let table = db.get_table("term").expect("term table not exist.");
		let rst = table.find(&cond, &doc, &op);
		let rst = rst.and_then(|back_json|{
			//info!("{}", back_json);
			let rows = json_i64!(&back_json; "rows");
			if rows > 0 {
				let data = json_path!(&back_json; "data", "0");
				Result::Ok(data.clone())
			} else {
				info!("no term need to end.");
				Result::Err(-1)
			}
		});
		let rst = rst.and_then(|term| {
			
			let id = json_i64!(&term; "id");
			let version = json_i64!(&term; "version");
			
			let mut cond = json!("{}");
			json_set!(&mut cond; "id"; id);
			json_set!(&mut cond; "version"; version);
			
			let mut doc = json!("{}");
			let mut set_data = json!("{}");
			json_set!(&mut set_data; "status"; CONS.code_to_id("term_status", "ready_to_end").unwrap());
			json_set!(&mut doc; "$set"; set_data);
			
			let mut op = json!("{}");
			let mut ret_data = json!("{}");
			json_set!(&mut ret_data; "id"; 1);
			json_set!(&mut ret_data; "game_id"; 1);
			json_set!(&mut ret_data; "code"; 1);
			json_set!(&mut ret_data; "status"; 1);
			json_set!(&mut op; "ret"; ret_data);
			
			let rst = table.update(&cond, &doc, &op);
			rst
		});
		let rst = rst.and_then(|back_json|{
			let rows = json_i64!(&back_json; "rows");
			if(rows > 0) {
				let data = json_path!(&back_json; "data", "0");
				Result::Ok(data.clone())
			} else {
				info!("no terms to handle...");
				Result::Err(-1)
			}
		});
		rst.ok()
	}
	
	///查找需要算奖的期次
	pub fn find_to_draw(&self, db:&DataBase<MyDbPool>) -> Option<Json> {
		let mut cond = json!("{}");
		let timestamp = Local::now().timestamp();
		json_set!(&mut cond; "status"; CONS.code_to_id("term_status", "open").unwrap());
		
		let doc = json!("{}");
		let op = json!("{}");
		
		let table = db.get_table("term").expect("term table not exist.");
		let rst = table.find(&cond, &doc, &op);
		let rst = rst.and_then(|back_json|{
			//info!("{}", back_json);
			let rows = json_i64!(&back_json; "rows");
			if rows > 0 {
				let data = json_path!(&back_json; "data", "0");
				Result::Ok(data.clone())
			} else {
				info!("no term need to draw.");
				Result::Err(-1)
			}
		});
		let rst = rst.and_then(|term| {
			
			let id = json_i64!(&term; "id");
			let version = json_i64!(&term; "version");
			
			let mut cond = json!("{}");
			json_set!(&mut cond; "id"; id);
			json_set!(&mut cond; "version"; version);
			
			let mut doc = json!("{}");
			let mut set_data = json!("{}");
			json_set!(&mut set_data; "status"; CONS.code_to_id("term_status", "drawing").unwrap());
			json_set!(&mut doc; "$set"; set_data);
			
			let mut op = json!("{}");
			let mut ret_data = json!("{}");
			json_set!(&mut ret_data; "id"; 1);
			json_set!(&mut ret_data; "game_id"; 1);
			json_set!(&mut ret_data; "code"; 1);
			json_set!(&mut ret_data; "status"; 1);
			json_set!(&mut ret_data; "draw_number"; 1);
			json_set!(&mut op; "ret"; ret_data);
			
			let rst = table.update(&cond, &doc, &op);
			rst
		});
		let rst = rst.and_then(|back_json|{
			let rows = json_i64!(&back_json; "rows");
			if(rows > 0) {
				let data = json_path!(&back_json; "data", "0");
				Result::Ok(data.clone())
			} else {
				info!("no terms to handle...");
				Result::Err(-1)
			}
		});
		rst.ok()
	}
    
    ///查找需要返奖的期次
	pub fn find_to_fund(&self, db:&DataBase<MyDbPool>) -> Option<Json> {
		let mut cond = json!("{}");
		json_set!(&mut cond; "status"; CONS.code_to_id("term_status", "drawed").unwrap());
		let doc = json!("{}");
		let op = json!("{}");
		let table = db.get_table("term").unwrap();
		let rst = table.find_one(&cond, &doc, &op);
		let rst = rst.and_then(|term| {
			let id = json_i64!(&term; "id");
			let version = json_i64!(&term; "version");
			
			let mut cond = json!("{}");
			json_set!(&mut cond; "id"; id);
			json_set!(&mut cond; "version"; version);
			
			let mut doc = json!("{}");
			let mut set_data = json!("{}");
			json_set!(&mut set_data; "status"; CONS.code_to_id("term_status", "funding").unwrap());
			json_set!(&mut doc; "$set"; set_data);
			
			let mut op = json!("{}");
			let mut ret_data = json!("{}");
			json_set!(&mut ret_data; "id"; 1);
			json_set!(&mut ret_data; "game_id"; 1);
			json_set!(&mut ret_data; "code"; 1);
			json_set!(&mut ret_data; "status"; 1);
			json_set!(&mut op; "ret"; ret_data);
			
			let rst = table.update(&cond, &doc, &op);
			rst
		});
		let rst = rst.and_then(|back_json|{
			let rows = json_i64!(&back_json; "rows");
			if(rows > 0) {
				let data = json_path!(&back_json; "data", "0");
				Result::Ok(data.clone())
			} else {
				info!("no terms to handle...");
				Result::Err(-1)
			}
		});
		rst.ok()
	}

	
	///保存奖级信息
	pub fn save_gl(&self, gl:&Json, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
		let table = db.get_table("game_level").expect("game_level table not exists.");
		
		let game_id = json_i64!(gl; "game_id");
		let term_code = json_i64!(gl; "term_code");
		let lev = json_i64!(gl; "lev");
		let bonus = json_i64!(gl; "bonus");
		let bonus_after_tax = json_i64!(gl; "bonus_after_tax");
		let descrip = json_str!(gl; "descrip");
		
		let mut conflict = json!("{}");
		json_set!(&mut conflict; "game_id"; 1);
		json_set!(&mut conflict; "term_code"; 1);
		json_set!(&mut conflict; "lev"; 1);
		
		let mut data = json!("{}");
		json_set!(&mut data; "game_id"; game_id);
		json_set!(&mut data; "term_code"; term_code);
		json_set!(&mut data; "descrip"; descrip);
		json_set!(&mut data; "lev"; lev);
		json_set!(&mut data; "bonus"; bonus);
		json_set!(&mut data; "bonus_after_tax"; bonus_after_tax);
		let now = time::get_time();
		json_set!(&mut data; "create_time"; now.sec);
		
		let mut up_data = json!("{}");
		let mut set_data = json!("{}");
		json_set!(&mut set_data; "create_time"; now.sec);
		json_set!(&mut set_data; "bonus"; bonus);
		json_set!(&mut set_data; "bonus_after_tax"; bonus_after_tax);
		json_set!(&mut set_data; "descrip"; descrip);
		json_set!(&mut up_data; "$set"; set_data);
		
		let op = json!("{}");
		
		let rst = table.upsert(&conflict, &data, &up_data, &op);
		rst
	}
	
	///开奖
	pub fn draw(&self, id:i64, version:i64, db:&DataBase<MyDbPool>) -> Result<Json, i32> {	
		let mut cond = json!("{}");
		json_set!(&mut cond; "id"; id);
		json_set!(&mut cond; "version"; version);
		json_set!(&mut cond; "status"; CONS.code_to_id("term_status", "end").unwrap());
		let table = db.get_table("term").unwrap();

        //check the term draw number 
        let mut doc = json!("{}");
		json_set!(&mut doc; "id"; 1);
		json_set!(&mut doc; "game_id"; 1);
		json_set!(&mut doc; "draw_number"; 1);

        let rst = table.find_one(&cond, &doc, &json!("{}"));
        let rst = rst.or(Err(ErrCode::DataExpired as i32));
        let term = try!(rst);
        let game_id = term.find("game_id").unwrap().as_i64().unwrap();
        let draw_number = term.find("draw_number").unwrap().as_string().unwrap();
        match game_id {
            200 => {
                let re = Regex::new(r"^([0-9]{2},){4}[0-9]{2}\|[0-9]{2},[0-9]{2}$").unwrap();
                if !re.is_match(draw_number) {
                    return Result::Err(ErrCode::DrawNumberIsWrong as i32);
                }
            },
            201 | 202 => {
                let re = Regex::new(r"^([0-9|-]{1,3},){4}[0-9|-]{1,3}$").unwrap();
                if !re.is_match(draw_number) {
                    return Result::Err(ErrCode::DrawNumberIsWrong as i32);
                }
            },
            301 => {
                let re = Regex::new(r"^[0-9]{1,3},[0-9]{1,3}$").unwrap();
                if !re.is_match(draw_number) {
                    return Result::Err(ErrCode::DrawNumberIsWrong as i32);
                }
            },
            _ => {
            }
        }
		
		let mut doc = json!("{}");
		let mut set_data = json!("{}");
		json_set!(&mut set_data; "status"; CONS.code_to_id("term_status", "open").unwrap());
		json_set!(&mut doc; "$set"; set_data);
        let mut inc_data = json();
		json_set!(&mut inc_data; "version"; 1);
		json_set!(&mut doc; "$inc"; inc_data);
		
		let op = json!("{}");
		
		table.update(&cond, &doc, &op)
	}
}

pub struct AccountService;

impl AccountService {
	
    //处理副账户
    pub fn handle_extra(&self, customer_id:i64, amount:i64, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
		let mut cond = json!("{}");
        json_set!(&mut cond; "id"; customer_id);
		
		let mut doc = json!("{}");
		let mut inc_data = json!("{}");
		json_set!(&mut inc_data; "client_balance"; amount);
		json_set!(&mut doc; "$inc"; inc_data);
		
		let mut op = json!("{}");

        //账户操作结果
		let table = db.get_table("account").unwrap();
		let rst = table.update(&cond, &doc, &op);
        rst
    }

	/**
	 * 扣／收款，并处理账户流水
	 */
	pub fn handle(&self, customer_id:i64, order_id:&str, log_type:i32, amount:i64, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
		let mut cond = json!("{}");
		if amount < 0 {
			let abs_amount = amount*-1_i64;
			let mut gt_data = json!("{}");
			json_set!(&mut gt_data; "$gt"; abs_amount);
			json_set!(&mut cond; "balance"; gt_data);
		}
		json_set!(&mut cond; "id"; customer_id);
		
		let mut doc = json!("{}");
		let mut inc_data = json!("{}");
		json_set!(&mut inc_data; "balance"; amount);
		json_set!(&mut doc; "$inc"; inc_data);
		
		let mut op = json!("{}");
		let mut ret = json!("{}");
		json_set!(&mut ret; "balance"; 1);
		json_set!(&mut op; "ret"; ret);
		
		//账户操作结果
		let table = db.get_table("account").unwrap();
		let rst = table.update(&cond, &doc, &op);
		let rst = rst.and_then(|ban_json|{
			let rows = json_i64!(&ban_json; "rows");
			if rows > 0 {
				let balance = json_i64!(&ban_json; "data", "0", "balance");
				Result::Ok(balance)
			} else {
				Result::Err(ErrCode::BalanceNotAllowed as i32)
			}
		});
		let rst = rst.and_then(|balance|{
			let mbefore = balance - amount;
			let mafter = balance;
			
			let mut moneylog = json!("{}");
			json_set!(&mut moneylog; "amount"; amount);
			json_set!(&mut moneylog; "mbefore"; mbefore);
			json_set!(&mut moneylog; "mafter"; mafter);
			json_set!(&mut moneylog; "type"; log_type);
			json_set!(&mut moneylog; "customer_id"; customer_id);
			json_set!(&mut moneylog; "order_id"; order_id);
			let now = time::get_time();
    			json_set!(&mut moneylog; "create_time"; now.sec);
    		
    			let table = db.get_table("moneylog").expect("moneylog table not exists.");
    			table.save(&moneylog, &json!("{}"))
		});
		rst
	}
	
	///账户可以被扣成负值
	pub fn handle_static(&self, customer_id:i64, order_id:&str, log_type:i32, amount:i64, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
		let mut cond = json!("{}");
		json_set!(&mut cond; "id"; customer_id);
		
		let mut doc = json!("{}");
		let mut inc_data = json!("{}");
		json_set!(&mut inc_data; "balance"; amount);
		json_set!(&mut doc; "$inc"; inc_data);
		
		let mut op = json!("{}");
		let mut ret = json!("{}");
		json_set!(&mut ret; "balance"; 1);
		json_set!(&mut op; "ret"; ret);
		
		//账户操作结果
		let table = db.get_table("account").unwrap();
		let rst = table.update(&cond, &doc, &op);
		let rst = rst.and_then(|ban_json|{
			let rows = json_i64!(&ban_json; "rows");
			if rows > 0 {
				let balance = json_i64!(&ban_json; "data", "0", "balance");
				Result::Ok(balance)
			} else {
				Result::Err(ErrCode::BalanceNotAllowed as i32)
			}
		});
		let rst = rst.and_then(|balance|{
			let mbefore = balance - amount;
			let mafter = balance;
			
			let mut moneylog = json!("{}");
			json_set!(&mut moneylog; "amount"; amount);
			json_set!(&mut moneylog; "mbefore"; mbefore);
			json_set!(&mut moneylog; "mafter"; mafter);
			json_set!(&mut moneylog; "type"; log_type);
			json_set!(&mut moneylog; "customer_id"; customer_id);
			json_set!(&mut moneylog; "order_id"; order_id);
			let now = time::get_time();
    			json_set!(&mut moneylog; "create_time"; now.sec);
    		
    			let table = db.get_table("moneylog").expect("moneylog table not exists.");
    			table.save(&moneylog, &json!("{}"))
		});
		rst
	}
	
}

pub struct QueryService;

impl QueryService {

    //获得周n与日期的对应关系
    pub fn get_week_map(&self) -> HashMap<&str, i64> {
        let mut map = HashMap::new();
        let dt = Local::now();
        for i in 0..6 {
            let cur_dt = dt + chrono::Duration::days(i);
            let weekday = cur_dt.weekday();
            let weekday_str = self.get_week_date_str(weekday);
            let day = i64::from_str(cur_dt.format("%Y%m%d").to_string().as_str()).unwrap();
            map.insert(weekday_str, day);
        }
        map
    }

    pub fn get_week_date_str<'a>(&'a self, weekday:Weekday) -> &'a str {
        match weekday {
            Weekday::Mon => {
                "周一"
            },
            Weekday::Tue => {
                "周二"
            },
            Weekday::Wed => {
                "周三"
            },
            Weekday::Thu => {
                "周四"
            },
            Weekday::Fri => {
                "周五"
            },
            Weekday::Sat => {
                "周六"
            },
            Weekday::Sun => {
                "周日"
            },
        }
    }

    //get the day from week str and the end time
    pub fn get_day(&self, week:&str, end_time:&DateTime<Local>) -> String {
        info!("{}", week); 
        info!("{}", end_time);
        let mut gap = 0;
        loop {
            let du = chrono::duration::Duration::days(gap);
            let date = end_time.sub(du);
            let weekday = date.weekday();
            let cur_week_str = self.get_week_date_str(weekday);
            if week == cur_week_str {
                let day = date.format("%Y%m%d").to_string();
                info!("{}", day);
                return day;
            }
            gap += 1;
        }
    }

    pub fn get_jcl_term(&self, team_info:&str, time_info:&str) -> Result<Json, i32> {
        let (date, team) = team_info.split_at(9);
        let team = team.trim_left().trim();
        let (week, index) = date.split_at(6);
        let team_array:Vec<&str> = team.split("VS").collect();
        let master = team_array[0];
        let guest = team_array[1];
        let time = time_info.trim_left().trim();
        let time = format!("20{}:00", time);
        let end_time = Local.datetime_from_str(time.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();
        
        let date_string = self.get_day(week, &end_time);

        let mut term = json!("{}");
        json_set!(&mut term; "game_id"; 301);
        let code = i64::from_str(format!("{}{}", date_string, index).as_str()).unwrap();
        json_set!(&mut term; "code"; code);
        let dt = Local::now();
        json_set!(&mut term; "sale_time"; dt.timestamp());
        json_set!(&mut term; "end_time"; end_time.timestamp());
        json_set!(&mut term; "master"; master);
        json_set!(&mut term; "guest"; guest);
        json_set!(&mut term; "play_types"; "01,02,03,04");
        json_set!(&mut term; "dc_play_types"; "01,02,03,04");
        json_set!(&mut term; "give"; 0);
        json_set!(&mut term; "status"; CONS.code_to_id("term_status", "init").unwrap());
        return Result::Ok(term);
    }


    pub fn get_jcc_term(&self, team_info:&str, time_info:&str) -> Result<Json, i32> {
        let (date, team) = team_info.split_at(9);
        let team = team.trim_left().trim();
        let (week, index) = date.split_at(6);
        let team_array:Vec<&str> = team.split("VS").collect();
        let master = team_array[0];
        let guest = team_array[1];
        let (give, time) = time_info.split_at(6);
        let (_, give) = give.split_at(3);
        let give = give.trim();
        let give = i64::from_str(give).unwrap()*10;
        let time = time.trim_left().trim();
        let time = format!("20{}:00", time);
        let end_time = Local.datetime_from_str(time.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();
        
        let date_string = self.get_day(week, &end_time);

        let mut term = json!("{}");
        json_set!(&mut term; "game_id"; 201);
        let code = i64::from_str(format!("{}{}", date_string, index).as_str()).unwrap();
        json_set!(&mut term; "code"; code);
        let dt = Local::now();
        json_set!(&mut term; "sale_time"; dt.timestamp());
        json_set!(&mut term; "end_time"; end_time.timestamp());
        json_set!(&mut term; "master"; master);
        json_set!(&mut term; "guest"; guest);
        json_set!(&mut term; "play_types"; "01,02,03,04,05");
        json_set!(&mut term; "dc_play_types"; "03,04,05");
        json_set!(&mut term; "give"; give);
        json_set!(&mut term; "status"; CONS.code_to_id("term_status", "init").unwrap());
        return Result::Ok(term);
    }
    
    pub fn term_info(&self, terminal_id:i64, body:&Json, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
        let game_id = json_i64!(body; "game_id");
        let stub = json_str!(body; "stub");
        info!("{}", stub);
        if game_id == 201 {
            let stub_array:Vec<&str> = stub.lines().collect();
            let stub_len = stub_array.len();
            if stub_len <= 3 {
                return Result::Ok(1);
            }
            let mut index = 2_usize;
            let table = db.get_table("term").unwrap();
            let op = json!("{}");
            loop {
                let team_info = stub_array[index];
                if !team_info.starts_with("周") {
                    break;
                }
                index += 1;
                let time_info = stub_array[index];
                let term_rst = self.get_jcc_term(team_info, time_info);
                if let Ok(term) = term_rst {
                    let _ = table.save(&term, &op);
                }
                index += 1;
                if index + 1 >= stub_len {
                    break;
                }
            }
        } else if game_id == 301 {
            let stub_array:Vec<&str> = stub.lines().collect();
            let stub_len = stub_array.len();
            if stub_len <= 3 {
                return Result::Ok(1);
            }
            let mut index = 2_usize;
            let table = db.get_table("term").unwrap();
            let op = json!("{}");
            loop {
                let team_info = stub_array[index];
                if !team_info.starts_with("周") {
                    break;
                }
                index += 1;
                let time_info = stub_array[index];
                let term_rst = self.get_jcl_term(team_info, time_info);
                if let Ok(term) = term_rst {
                    let _ = table.save(&term, &op);
                }
                index += 1;
                if index + 1 >= stub_len {
                    break;
                }
            }
        }

        Result::Ok(1)
    }

    pub fn charge_info(&self, terminal_id:i64, body:&Json, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
        let stub = json_str!(body; "stub");
        let stub_array:Vec<&str> = stub.lines().collect();
        let len = stub_array.len();
        let mut index = 0_usize;
        loop {
            if index >= len {
                break;
            }
            let line = stub_array[index];

            if line.starts_with("当前可用额度：") {
                let (start_info, balance_info) = line.split_at(21);
                let trim_chars:&[_] = &['元', ' '];
                let balance_info = balance_info.trim_matches(trim_chars);
                let balance_rst = f64::from_str(balance_info);
                let rst = balance_rst.or(Result::Err(-1));
                let balance_f64 = try!(rst);
                let balance = (balance_f64*100_f64) as i64;
                info!("the balance is {}.", balance);

                TerminalService.set_out_balance(terminal_id, balance, db);
            }

            if line.starts_with("账户缴款明") {
                index += 3; 
                loop {
                    if index >= len {
                       break;
                    }
                    let line = stub_array[index];
                    if line.starts_with('+') {
                        index += 1;
                        if index >= len {
                            break;
                        }
                        let order_info = stub_array[index];
                        let moneylog_type = CONS.code_to_id("moneylog_type", "charge").unwrap(); 
                        self.save_charge_info(terminal_id, moneylog_type, line, order_info, db);
                    } else {
                        break;
                    }
                    index += 1;
                }
            }

            if line.starts_with("佣金转入额度") {
                index += 3; 
                loop {
                    if index >= len {
                       break;
                    }
                    let line = stub_array[index];
                    if line.starts_with('+') {
                        let order_info = "佣金转入";
                        let moneylog_type = CONS.code_to_id("moneylog_type", "repay").unwrap(); 
                        self.save_charge_info(terminal_id, moneylog_type, line, order_info, db);
                    } else {
                        break;
                    }
                    index += 1;
                }
            }

            index += 1;
            if index >= len {
                break;
            }
        }
        Result::Ok(1)
    }

    pub fn save_charge_info(&self, terminal_id:i64, moneylog_type:i32, line:&str, order_info:&str, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
        let line_length = line.len();
        if line_length < 35 {
            return Result::Err(-1);
        }
        let split_index = line_length - 19;
        let (amount_info, time_info) = line.split_at(split_index);
        let time_rst = Local.datetime_from_str(time_info, "%Y-%m-%d %H:%M:%S");
        if time_rst.is_err() {
            return Result::Err(-1);
        }
        let time = time_rst.unwrap();
        let mut charge_report = json!("{}");
        json_set!(&mut charge_report; "timestamp"; time.timestamp());

        let trim_chars:&[_] = &['+', '元', ' '];
        let amount_info = amount_info.trim_matches(trim_chars);
        let amount = f64::from_str(amount_info).unwrap();
        let amount = (amount*100_f64) as i64;
        json_set!(&mut charge_report; "amount"; amount);

        json_set!(&mut charge_report; "terminal_id"; terminal_id);
        json_set!(&mut charge_report; "order_info"; order_info);
        let status = CONS.code_to_id("charge_report_status", "unhandle").unwrap();
        json_set!(&mut charge_report; "status"; status);

        let mut op = json!("{}");
		let mut ret_data = json!("{}");
		json_set!(&mut ret_data; "id"; 1);
		json_set!(&mut op; "ret"; ret_data);

        let table = db.get_table("charge_report").unwrap();
        let rst = table.save(&charge_report, &op);
        if rst.is_err() {
            return Result::Ok(1);
        }

        let save_json = rst.unwrap();
        let report_id = json_i64!(&save_json; "data", "0", "id");
			
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
        let order_id = format!("charge_report_{}", report_id);
        let rst = AccountService.handle(terminal_id, order_id.as_str(), 
            moneylog_type,
		    amount, db
		);
       
        Result::Ok(1)
    }

}

pub struct CacheService;

impl CacheService {
	
    ///发送查询消息，msg由两部分构成，{"cmd":"T04", "body":{"game_id":201, "type":0}}
    ///cmd表示命令码，body为消息的主体
    pub fn send_query_msg(&self, msg:&Json, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
        let msg_string = msg.to_string();
		let conn = db.cache.get_conn().unwrap().lock().unwrap();
        let rst:RedisResult<()> = conn.lpush("query_msg_list", msg_string);
		rst.and(Result::Ok(1)).or(Result::Err(-1))	
    }

    //发送属于某一台终端的查询指令
    pub fn send_terminal_query_msg(&self, terminal_id:i64, msg:&Json, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
        let key = format!("query_msg_list::{}", terminal_id);
        let msg_string = msg.to_string();
		let conn = db.cache.get_conn().unwrap().lock().unwrap();
        let rst:RedisResult<()> = conn.lpush(key.as_str(), msg_string);
		rst.and(Result::Ok(1)).or(Result::Err(-1))	
    }

    ///获得查询信息
    pub fn get_query_msg(&self, db:&DataBase<MyDbPool>) -> Result<String, i32> {
		let conn = db.cache.get_conn().unwrap().lock().unwrap();
		let rst:RedisResult<(String)> = conn.rpop("query_msg_list");
		rst.or(Result::Err(-1))
	}

    //获得属于某一台终端的查询指令
    pub fn get_terminal_query_msg(&self, terminal_id:i64, db:&DataBase<MyDbPool>) -> Result<String, i32> {
        let key = format!("query_msg_list::{}", terminal_id);
		let conn = db.cache.get_conn().unwrap().lock().unwrap();
		let rst:RedisResult<(String)> = conn.rpop(key.as_str());
		rst.or(Result::Err(-1))
	}

	///打印一张票，处于队列的末尾位置
	pub fn print(&self, ticket:&Json, terminal_id:i64, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
        let now = time::get_time();
        let timestamp = now.sec;
        let end_time = {
            let end_time_node = ticket.find("end_time").unwrap(); 
            end_time_node.as_i64().unwrap()
        };
        let tense:bool; //是否是紧急票
        if end_time - timestamp < 35*60 {
            tense = true;
        } else {
            tense = false; 
        }
		let value = ticket.to_string();
		let conn = db.cache.get_conn().unwrap().lock().unwrap();
        if terminal_id <= 0 {
		    let rst:RedisResult<()>;
            if tense {
                rst = conn.rpush("print_list", value);
            } else {
                rst = conn.lpush("print_list", value);
            }
		    rst.and(Result::Ok(1)).or(Result::Err(-1))	
        } else {
            let key = format!("print_list::{}", terminal_id);
		    let rst:RedisResult<()>;
            if tense {
                rst = conn.rpush(key.as_str(), value);
            } else {
                rst = conn.lpush(key.as_str(), value);
            }
		    rst.and(Result::Ok(1)).or(Result::Err(-1))	
        }
	}

	///票加入兑奖队列的末尾位置
	pub fn bonus(&self, terminal_id:i64, ticket:&Json, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
        let key = format!("bonus::list::{}", terminal_id);
		let value = ticket.to_string();
		let conn = db.cache.get_conn().unwrap().lock().unwrap();
		let rst:RedisResult<()> = conn.lpush(key.as_str(), value);
		rst.and(Result::Ok(1)).or(Result::Err(-1))	
	}

    //设置打印间隔
    pub fn set_print_gap(&self, gap:i64, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
        let conn = db.cache.get_conn().unwrap().lock().unwrap();
		let rst:RedisResult<()> = conn.set("print::gap", gap);
		rst.and(Result::Ok(1)).or(Result::Err(-1))	
    }

    ///获得系统出票间隔
    pub fn get_print_gap(&self, db:&DataBase<MyDbPool>) -> i64 {
		let conn = db.cache.get_conn().unwrap().lock().unwrap();
		let rst:RedisResult<(i64)> = conn.get("print::gap");
        match rst {
            Ok(gap) => {
                gap
            },
            Err(_) => {
                2_i64 
            },
        }
	}

    ///系统模式,0,出票,1,兑奖
    pub fn set_terminal_mode(&self, mode:i64, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
		let conn = db.cache.get_conn().unwrap().lock().unwrap();
		let rst:RedisResult<()> = conn.set("terminal::mode", mode);
		rst.and(Result::Ok(1)).or(Result::Err(-1))	
    }

    ///获得系统模式
    pub fn get_terminal_mode(&self, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
		let conn = db.cache.get_conn().unwrap().lock().unwrap();
		let rst:RedisResult<(i32)> = conn.get("terminal::mode");
		rst.and_then(|back_str|{
			info!("terminal mode is {}", back_str);
			Result::Ok(back_str)
		}).or(Result::Err(-1))
	}

    pub fn get_com_out_id(&self, com_id:i64, out_id:&str, db:&DataBase<MyDbPool>) -> Result<bool, i32> {
        let key = format!("company::out_id::{}::{}", com_id, out_id);
		let conn = db.cache.get_conn().unwrap().lock().unwrap();
		let rst:RedisResult<(bool)> = conn.exists(key.as_str());
		rst.and_then(|flag|{
			info!("the company out_id flag is {}", flag);
			Result::Ok(flag)
		}).or(Result::Err(-1))
	}

    pub fn set_com_out_id(&self, com_id:i64, out_id:&str, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
        let key = format!("company::out_id::{}::{}", com_id, out_id);
		let conn = db.cache.get_conn().unwrap().lock().unwrap();
		let rst:RedisResult<(i32)> = conn.set(key.as_str(), 1);
		rst.and_then(|flag|{
			Result::Ok(flag)
		}).or(Result::Err(-1))
	}

    pub fn get_ticket_to_print_by_terminal(&self, terminal_id:i64, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
		let conn = db.cache.get_conn().unwrap().lock().unwrap();
		let rst:RedisResult<(String)> = conn.rpop("print_list");
		rst.and_then(|json_str|{
			info!("{}", json_str);
			Result::Ok(json!(&json_str))
		}).or(Result::Err(-1))
	}

	pub fn get_ticket_to_print(&self, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
		let conn = db.cache.get_conn().unwrap().lock().unwrap();
		let rst:RedisResult<(String)> = conn.rpop("print_list");
		rst.and_then(|json_str|{
			info!("{}", json_str);
			Result::Ok(json!(&json_str))
		}).or(Result::Err(-1))
	}

    ///获得当前出票队列的长度
    pub fn get_print_list_len(&self, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
        let conn = db.cache.get_conn().unwrap().lock().unwrap();
		let rst:RedisResult<(i32)> = conn.llen("print_list");
		rst.or(Result::Err(-1))
    }
    
    ///彩票机获得自己的兑奖票
    pub fn get_ticket_to_bonus(&self, terminal_id:i64, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
        let key = format!("bonus::list::{}", terminal_id);
		let conn = db.cache.get_conn().unwrap().lock().unwrap();
		let rst:RedisResult<(String)> = conn.rpop(key.as_str());
		rst.and_then(|json_str|{
			info!("{}", json_str);
			Result::Ok(json!(&json_str))
		}).or(Result::Err(-1))
	}

    ///获得未兑奖票的数量
    pub fn get_bonus_length(&self, terminal_id:i64, db:&DataBase<MyDbPool>) -> Result<i64, i32> {
        let key = format!("bonus::list::{}", terminal_id);
		let conn = db.cache.get_conn().unwrap().lock().unwrap();
		let rst:RedisResult<(i64)> = conn.llen(key.as_str());
		rst.and_then(|len|{
			Result::Ok(len)
		}).or(Result::Err(-1))
    }
    
    //当出票错误时，是否转发到其它的终端
    pub fn resend_ticket_when_err(&self, db:&DataBase<MyDbPool>) -> bool {
        let key = "resend_ticket_when_err";
		let conn = db.cache.get_conn().unwrap().lock().unwrap();
		let rst:RedisResult<(i32)> = conn.get(key);
        if rst.is_err() {
            return true;
        }
        let resend = rst.unwrap();
        if resend > 0 {
            return true;
        } else {
            return false;
        }
    }

    //设置当出票错误时，是否转发到其它终端
    pub fn set_resend_ticket_when_err(&self, is_resend:bool, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
        let key = "resend_ticket_when_err";
        let value;
        if is_resend {
            value = 1;
        } else {
            value = 0;
        }
        let conn = db.cache.get_conn().unwrap().lock().unwrap();
		let rst:RedisResult<()> = conn.set(key, value);
		rst.and(Result::Ok(1)).or(Result::Err(-1))	
    }
}

pub struct PrintService;

impl PrintService {
	
	
	///获得终端机的可出游戏列表
	pub fn get_terminal_game(&self, terminal_id:i64, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
		let table = db.get_table("terminal_game").unwrap();
		let mut cond = json!("{}");
		json_set!(&mut cond; "terminal_id"; terminal_id);
		
		let doc = json!("{}");
		let op = json!("{}");
		
		let rst = table.find(&cond, &doc, &op);
		rst
	}
	
	///获得票据的序列号
	pub fn get_ticket_seq(&self, game_id:i64, stub:&str) -> String {
		if game_id == 202_i64 || game_id == 201_i64 || game_id == 301_i64 {
			let num_array:Vec<&str> = stub.split('\n').collect();
			num_array[2].to_string()	
		} else {
			let re = Regex::new(r"([0-9]{6}-[0-9]{6}-[0-9]{6}-[0-9]{6} [0-9]{6})").unwrap();
			let cap_op = re.captures(stub);
			match cap_op {
				Some(x) => {
					let sep = x.at(0).unwrap_or("");
					sep.to_string()
				},
				None => {
					String::new()
				},
			}	
		}
	}
	
	///竞彩获得出票返回的赔率信息
	pub fn get_print_number(&self, game_id:i64, play_type:i64, bet_type:i64, number:&str, stub:&str) -> String {
		if game_id == 202_i64 || game_id == 201_i64 || game_id == 301_i64 {
            let back_number = PF.get_print_number(game_id, play_type, bet_type, number, stub).unwrap();
            back_number 
		} else {
			String::new()
		}
	}
	
	/**
	 * 打印成功
	 */
	pub fn success(&self, customer_id:i64, ticket:&Json, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
		//info!("{} print {} success.", customer_id, ticket);
		let id = json_i64!(ticket; "id");
		let game_id = json_i64!(ticket; "game_id");
		let play_type = json_i64!(ticket; "play_type");
		let bet_type = json_i64!(ticket; "bet_type");
		let stub = json_str!(ticket; "stub");
		let number = json_str!(ticket; "number");
		
		let mut cond = json!("{}");
		json_set!(&mut cond; "id"; id);
        let mut or_data = Vec::<Json>::new();
        let mut obj = json!("{}");
		json_set!(&mut obj; "status"; CONS.code_to_id("ticket_status", "printing").unwrap());
        or_data.push(obj);
        let mut obj = json!("{}");
		json_set!(&mut obj; "status"; CONS.code_to_id("ticket_status", "print_err").unwrap());
        or_data.push(obj);
		json_set!(&mut cond; "$or"; or_data);
		
        let mut status = CONS.code_to_id("ticket_status", "print_success").unwrap();
        if game_id == 200 {
            if !stub.contains("体彩<超级大乐透>") {
                status = CONS.code_to_id("ticket_status", "print_err").unwrap();
            }
        } else if game_id == 203 {
            if !stub.contains("体彩<排列3>") {
                status = CONS.code_to_id("ticket_status", "print_err").unwrap();
            }
        } else if game_id == 201 || game_id == 202 || game_id == 301 {
            if !stub.contains("中国体育彩票") {
                status = CONS.code_to_id("ticket_status", "print_err").unwrap();
            }
        }
		let mut doc = json!("{}");
		let mut set_data = json!("{}");
        
        if status == CONS.code_to_id("ticket_status", "print_success").unwrap() {
		    let seq = self.get_ticket_seq(game_id, stub);
		    json_set!(&mut set_data; "seq"; seq);

		    let print_number = self.get_print_number(game_id, play_type, bet_type, number, stub);
		    json_set!(&mut set_data; "print_number"; print_number);
        }

        //设置加密的票根
        let crypto_node_op = ticket.find("crypto");
        if let Some(crypto_node) = crypto_node_op {
            let crypto = crypto_node.as_string().unwrap();
		    json_set!(&mut set_data; "crypto"; crypto);
        }

		json_set!(&mut set_data; "stub"; stub);
		json_set!(&mut set_data; "terminal_id"; customer_id);
		json_set!(&mut set_data; "status"; status);
		let now = time::get_time();
		json_set!(&mut set_data; "print_time"; now.sec);
		json_set!(&mut doc; "$set"; set_data);
		
		let mut op = json!("{}");
		let mut ret_data = json!("{}");
		json_set!(&mut ret_data; "amount"; 1);
		json_set!(&mut op; "ret"; ret_data);
		
		let table = db.get_table("ticket").unwrap();
		let rst = table.update(&cond, &doc, &op);

        if status != CONS.code_to_id("ticket_status", "print_success").unwrap() {
            return Result::Err(-1);
        }

        //删除出票池中的数据
        let del_sql = format!("delete from print_pool where id={}", id);
        let del_rst = db.execute(&del_sql);
        let _ = try!(del_rst);
		
		let back_json = try!(rst);
		let order_id = format!("{}", id);
		let amount = json_i64!(&back_json; "data", "0", "amount");
		let rst = AccountService.handle_static(customer_id, &order_id, CONS.code_to_id("moneylog_type", "print").unwrap(), amount*-1_i64, db);

        let _ = try!(rst);

        //处理副账户的余额
        let rst = AccountService.handle_extra(customer_id, amount*-1_i64, db);
		rst
	}

    ///打印错误
    pub fn print_err(&self, customer_id:i64, ticket:&Json, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
		let id = json_i64!(ticket; "id");

        //移出出票池
        let _ = TicketService.out_print_pool(id, db);
		
		let mut cond = json!("{}");
		json_set!(&mut cond; "id"; id);
		json_set!(&mut cond; "status"; CONS.code_to_id("ticket_status", "printing").unwrap());
		
		let mut doc = json!("{}");
		let mut set_data = json!("{}");
		json_set!(&mut set_data; "terminal_id"; customer_id);
		json_set!(&mut set_data; "status"; CONS.code_to_id("ticket_status", "print_err").unwrap());
		json_set!(&mut doc; "$set"; set_data);
		
		let mut op = json!("{}");
		
		let table = db.get_table("ticket").unwrap();
		let rst = table.update(&cond, &doc, &op);
		rst
	}

    ///兑奖异常
    pub fn bonus_err(&self, customer_id:i64, ticket:&Json, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
		let id = json_i64!(ticket; "id");
		
		let mut cond = json!("{}");
		json_set!(&mut cond; "id"; id);
		json_set!(&mut cond; "status"; CONS.code_to_id("ticket_status", "funded").unwrap());
		
		let mut doc = json!("{}");
		let mut set_data = json!("{}");
		json_set!(&mut set_data; "status"; CONS.code_to_id("ticket_status", "bonus_err").unwrap());
		json_set!(&mut doc; "$set"; set_data);
        
        let mut inc_data = json!("{}");
		json_set!(&mut inc_data; "bonus_try_count"; 1);
		json_set!(&mut doc; "$inc"; inc_data);
		
		let mut op = json!("{}");
		
		let table = db.get_table("ticket").unwrap();
		let rst = table.update(&cond, &doc, &op);
		rst
	}

    ///重新打印
    pub fn reprint(&self, ticket_id:i64, terminal_id:i64, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
        let table = db.get_table("ticket").unwrap();
		let mut cond = json!("{}");
		json_set!(&mut cond; "id"; ticket_id);
		json_set!(&mut cond; "status"; CONS.code_to_id("ticket_status", "print_err").unwrap());

		let mut doc = json!("{}");
        let mut set_data = json!("{}");
		json_set!(&mut set_data; "status"; CONS.code_to_id("ticket_status", "printing").unwrap());
       	json_set!(&mut doc; "$set"; set_data);

        let mut op = json!("{}");
        let mut set_data = json!("{}");
		json_set!(&mut set_data; "id"; 1);
		json_set!(&mut set_data; "game_id"; 1);
		json_set!(&mut set_data; "play_type"; 1);
		json_set!(&mut set_data; "bet_type"; 1);
		json_set!(&mut set_data; "icount"; 1);
		json_set!(&mut set_data; "multiple"; 1);
		json_set!(&mut set_data; "number"; 1);
		json_set!(&mut set_data; "amount"; 1);
		json_set!(&mut set_data; "end_time"; 1);
       	json_set!(&mut op; "ret"; set_data);
        let rst = table.update(&cond, &doc, &op);
        let mut data = try!(rst);
        let rows = json_i64!(&data; "rows");
        if rows == 0 {
            return Result::Err(ErrCode::DataExpired as i32);
        }
        let mut data_obj = data.as_object_mut().unwrap();
        let mut data_obj = data_obj.remove("data").unwrap();
        let mut ticket_array = data_obj.as_array_mut().unwrap();
        let mut ticket = ticket_array.remove(0);
        
        let table = db.get_table("print_pool").unwrap();
        let op = json!("{}");
        let rst = table.save(&ticket, &op);
        rst
    }
	
	///打印回执
	pub fn back(&self, customer_id:i64, body:&Json, db:&DataBase<MyDbPool>) -> Result<i64, i32> {
		let ticket = json_path!(body; "ticket");
		let status = json_i64!(ticket; "status");
		if status == 1 {
			self.success(customer_id, ticket, db);
		}
		Result::Ok(1)
	}
	
}

pub struct CustomerService;

impl CustomerService {

    pub fn get_by_username(&self, username:&str, db:&DataBase<MyDbPool>) -> Result<Json, i32> {
		let table = db.get_table("customer").unwrap();
        let mut cond = json!("{}");
        json_set!(&mut cond; "username"; username);
		let doc = json!("{}");
		let op = json!("{}");
		table.find_one(&cond, &doc, &op).or(Result::Err(ErrCode::ValueNotExist as i32))
    }
}

pub struct BetService;

impl BetService {
	
	/**
	 * 投注
	 */
	pub fn bet(&self, customer_id:i64, mut ticket:&mut Json, db:&DataBase<MyDbPool>) -> Result<i64, i32> {
		let amount = {
            let amount_node = ticket.find("amount").unwrap();
            amount_node.as_i64().unwrap()
        };
        let game_id = {
            let game_id_node = ticket.find("game_id").unwrap();
            game_id_node.as_i64().unwrap()
        };
        let play_type = {
            let play_type_node = ticket.find("play_type").unwrap();
            play_type_node.as_i64().unwrap()
        };    
        let bet_type = {
            let bet_type_node = ticket.find("bet_type").unwrap();
            bet_type_node.as_i64().unwrap()
        };    
        let term_code = {
            let term_code_node = ticket.find("term_code").unwrap();
            term_code_node.as_i64().unwrap()
        };    
        let order_id = {
            let order_id_node = ticket.find("out_id").unwrap();
            order_id_node.as_string().unwrap().to_string()
        };
		
		//校验期次状态
		let rst;
        match game_id {
            202_i64 | 201_i64 | 301_i64 => {
			    rst = self.check_jc_term_status(&ticket, db);
            },
            _  => {
			    rst = self.check_term_status(game_id, play_type, bet_type, term_code, db);
            },
        }

        let rst = rst.or(Err(ErrCode::TermStatusNotAllowed as i32));

        //截止时间
        let end_time = try!(rst);
        {
            let mut ticket_obj = ticket.as_object_mut().unwrap();
            ticket_obj.insert("end_time".to_string(), end_time.to_json());
        }
		
		//扣款
		let rst = AccountService.handle(customer_id, order_id.as_str(), 
            CONS.code_to_id("moneylog_type", "bet").unwrap(), 
			amount*-1_i64, db
		);
		//保存票据
		let rst = rst.and_then(|_|{
			let table = db.get_table("ticket").unwrap();
			
			let mut op = json!("{}");
        	let mut ret_data = json!("{}");
        	json_set!(&mut ret_data; "id"; 1);
        	json_set!(&mut op; "ret"; ret_data);
        		
        	table.save(ticket, &op)
		});
		//获得id
		let rst = rst.and_then(|json|{
			let ticket_id = json_i64!(&json; "data", "0", "id");
			Result::Ok(ticket_id)
		});
		//保存到出票队列
		let rst = rst.and_then(|ticket_id|{
            let icount_node = ticket.find("icount").unwrap();
			let icount = icount_node.as_i64().unwrap();
            let multiple_node = ticket.find("multiple").unwrap();
			let multiple = multiple_node.as_i64().unwrap();
            let number_node = ticket.find("number").unwrap();
			let number = number_node.as_string().unwrap();
			
            /*
			let play_type_str;
			if play_type < 10 {
				play_type_str = format!("0{}", play_type);
			} else {
				play_type_str = format!("{}", play_type);
			}
			
			let bet_type_str;
			if bet_type < 10 {
				bet_type_str = format!("0{}", bet_type);
			} else {
				bet_type_str = format!("{}", bet_type);
			}
            */
			
			let mut print_ticket = json!("{}");
			json_set!(&mut print_ticket; "id"; ticket_id);
			json_set!(&mut print_ticket; "game_id"; game_id);
			json_set!(&mut print_ticket; "play_type"; play_type);
			json_set!(&mut print_ticket; "bet_type"; bet_type);
			json_set!(&mut print_ticket; "icount"; icount);
			json_set!(&mut print_ticket; "multiple"; multiple);
			json_set!(&mut print_ticket; "number"; number);
			json_set!(&mut print_ticket; "amount"; amount);
			json_set!(&mut print_ticket; "end_time"; end_time);
            let _ = TicketService.in_print_pool(&print_ticket, db);
			//json_set!(&mut print_ticket; "try_count"; 1);
			
			//CacheService.print(&print_ticket, -1, db);
			Result::Ok(ticket_id)
		});
		rst
	}

    //检验期次的玩法状态
    fn check_play_type(&self, game_id:i64, play_type:i64, bet_type:i64, term:&Json) -> Result<i32, i32> {
        if game_id == 201_i64 || game_id == 202_i64 || game_id == 301_i64 {
            let play_type_string = format!("0{}", play_type);
            let play_types;
            if game_id == 201_i64 {
                play_types = json_str!(term; "play_types");
            } else if game_id == 202_i64 {
                play_types = json_str!(term; "dc_play_types");
            } else {
                if bet_type == 1 {
                    play_types = json_str!(term; "dc_play_types");
                } else {
                    play_types = json_str!(term; "play_types");
                }
            }
            if play_types.len() == 0 {
		        return Result::Err(ErrCode::TermStatusNotAllowed as i32);
            } else {
                if play_types.contains(play_type_string.as_str()) {
                    return Result::Ok(1);
                } else {
			        return Result::Err(ErrCode::TermStatusNotAllowed as i32);
                }
            }
        }
        return Result::Ok(1);
    }
	
	/**
	 * 校验票据的期次状态是否可以销售，如果可以销售，返回期次的截止时间
	 */
	fn check_term_status(&self, game_id:i64, play_type:i64, bet_type:i64, term_code:i64, db:&DataBase<MyDbPool>) -> Result<i64, i32> {
		let table = db.get_table("term").unwrap();
		let mut cond = json!("{}");
        let mut term_game_id = game_id;
        if term_game_id == 202_i64 {
            term_game_id = 201_i64;
        }
		json_set!(&mut cond; "game_id"; term_game_id);
		json_set!(&mut cond; "code"; term_code);
		
		let rst = table.find_one(&cond, &json!("{}"), &json!("{}"));
		let rst = rst.and_then(|json|{
			let status = json_i64!(&json; "status");
			if status == CONS.code_to_id("term_status", "sale").unwrap() as i64 {
				Result::Ok(json)
			} else {
				Result::Err(ErrCode::TermStatusNotAllowed as i32)
			}
		});
        let term = try!(rst);
        let rst = self.check_play_type(game_id, play_type, bet_type, &term);
        let _ = try!(rst);
        let end_time_node = term.find("end_time").unwrap();
        let end_time = end_time_node.as_i64().unwrap();
        return Result::Ok(end_time);
	}
	
	///校验竞彩期次状态
	fn check_jc_term_status(&self, ticket:&Json, db:&DataBase<MyDbPool>) -> Result<i64, i32> {
		let game_id = json_i64!(ticket; "game_id");
		let play_type = json_i64!(ticket; "play_type");
		let bet_type = json_i64!(ticket; "bet_type");
        let mut end_time = -1_i64;
        if play_type == 10 {
            let number = json_str!(ticket; "number");
            let number_array:Vec<&str> = number.split(';').collect();
            for match_number in number_array {
                let match_info:Vec<&str> = match_number.split(':').collect();
                let term_code = i64::from_str(match_info[0]).unwrap();
                let play_type = i64::from_str(match_info[1]).unwrap();
			    let cur_end_time = try!(self.check_term_status(game_id, play_type, bet_type, term_code, db));
                if end_time < 0 || cur_end_time < end_time {
                    end_time = cur_end_time;
                }
            }
        } else {
		    let term_code_list = json_str!(ticket; "term_code_list");
		    let term_array:Vec<&str> = term_code_list.split(';').collect();
		    for term_code_str in term_array {
			    let term_code = i64::from_str(term_code_str).unwrap();
			    let cur_end_time = try!(self.check_term_status(game_id, play_type, bet_type, term_code, db));
                if end_time < 0 || cur_end_time < end_time {
                    end_time = cur_end_time;
                }

		    } 
        }
		Result::Ok(end_time)
	}
	
}
