use std::sync::{Arc, Mutex};

extern crate easydb;
use easydb::Column;
use easydb::Table;
use easydb::DbPool;

use std::collections::BTreeMap;

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::str::FromStr;

#[macro_use]
extern crate easy_config;
use easy_config::CFG;

extern crate postgres;
use postgres::{Connection, SslMode};
use postgres::types::Type;

extern crate rand;
use rand::distributions::{IndependentSample, Range};

#[macro_use]
extern crate log;

extern crate cons;
use cons::ErrCode;

extern crate redis;

pub struct MyDbPool {
    dsn:String,
    conns:Vec<Mutex<Connection>>,
}

impl MyDbPool {

    pub fn new(dsn:&str, size:u32) -> MyDbPool {
        let mut conns = vec![];
        for _ in 0..size {
            let conn = match Connection::connect(dsn, SslMode::None) {
                Ok(conn) => conn,
                Err(e) => {
                    println!("Connection error: {}", e);
                    break;
                }
            };
            conns.push(Mutex::new(conn));
        }
        MyDbPool {
            dsn:dsn.to_string(),
            conns:conns,
        }
    }

    /**
     * 获得dsn字符串
     */
    pub fn get_dsn(&self) -> String {
        self.dsn.clone()
    }

    pub fn get_back_json(&self, rows:postgres::rows::Rows) -> Json {
        rows_to_json(rows)
    }

    fn execute_conn(&self, conn:&Connection, sql:&str) -> Result<Json, i32> {
        let rst = conn.query(sql, &[]);
        let out_rst = rst.and_then(|rows| {
            Result::Ok(self.get_back_json(rows))
        });
        match out_rst {
            Ok(json) => {
                Result::Ok(json)
            },
            Err(err) => {
                println!("{}", err);
                Result::Err(-1)
            },
        }
    }

}

///convert the postgres rows to json
pub fn rows_to_json(rows:postgres::rows::Rows) -> Json {
    let mut rst_json = json!("{}");
    let mut data:Vec<Json> = Vec::new();
    for row in &rows {
        let mut back_json = json!("{}");
        let columns = row.columns();
        for column in columns {
            let name = column.name();
            let col_type = column.type_();
            match *col_type {
                Type::Int4 => {
                    let op:Option<postgres::Result<i32>> = row.get_opt(name);
                    let mut true_value:i32 = 0;
                    if let Some(rst) = op {
                        if let Ok(value) = rst {
                            true_value = value;
                        }
                    }
                    json_set!(&mut back_json; name; true_value);
                },
                Type::Int8 => {
                    let op:Option<postgres::Result<i64>> = row.get_opt(name);
                    let mut true_value:i64 = 0;
                    if let Some(rst) = op {
                        if let Ok(value) = rst {
                            true_value = value;
                        }
                    }
                    json_set!(back_json; name; true_value);
                },
                Type::Varchar | Type::Text => {
                    let op:Option<postgres::Result<String>> = row.get_opt(name);
                    let mut true_value:String = String::new();
                    if let Some(rst) = op {
                        if let Ok(value) = rst {
                            true_value = value;
                        }
                    }
                    json_set!(back_json; name; true_value);
                },
                _ => {
                    println!("ignore type:{}", col_type.name());
                },
            }
        }
        data.push(back_json);
    }
    json_set!(&mut rst_json; "data"; data);
    json_set!(&mut rst_json; "rows"; rows.len());
    rst_json
}

impl DbPool for MyDbPool {

    fn get_connection(&self) -> Result<Connection, i32> {
        let rst = match Connection::connect(self.dsn.as_str(), SslMode::None) {
            Ok(conn) => Result::Ok(conn),
            Err(e) => {
                println!("Connection error: {}", e);
                Result::Err(-1)
            }
        };
        rst
    }

    
    fn execute(&self, sql:&str) -> Result<Json, i32> {
        println!("{}", sql);
        let mut try_count = 0;
        let between = Range::new(0, self.conns.len());
        loop {
            try_count += 1;
            if try_count > 20 {
                break;
            }
            let mut rng = rand::thread_rng();
            let rand_int = between.ind_sample(&mut rng);
            let conn_rst = self.conns[rand_int].try_lock();
            if let Ok(conn) = conn_rst {
                return self.execute_conn(&conn, sql);
            }
        }
        let conn_rst = self.get_connection();
        if let Ok(conn) = conn_rst {
            return self.execute_conn(&conn, sql);
        } else {
            return Result::Err(ErrCode::NoDbConn as i32);
        }
    }
    
    fn stream<F>(&self, sql:&str, mut f:F) -> Result<i32, i32> where F:FnMut(Json) -> bool + 'static {
		let conn = try!(self.get_connection());
        let rst = conn.query("BEGIN", &[]);

        //begin
        let rst = rst.and_then(|rows| {
            let json = self.get_back_json(rows);
            println!("{}", json);
            Result::Ok(1)
        }).or_else(|err|{
            println!("{}", err);
            Result::Err(-1)
        });

        //cursor
        let rst = rst.and_then(|_| {
        		let cursor_sql = format!("DECLARE myportal CURSOR FOR {}", sql);
        		println!("{}", cursor_sql);
        		let rst = conn.query(&cursor_sql, &[]);
        		rst.and_then(|rows|{
	            let json = self.get_back_json(rows);
	            println!("{}", json);
	            Result::Ok(1)
	        }).or_else(|err|{
	            println!("{}", err);
	            Result::Err(-1)
	        })
        });

        let rst = rst.and_then(|_| {
            let fetch_sql = "FETCH NEXT in myportal";
            println!("{}", fetch_sql);

            let mut flag = 0;
            loop {
                let rst = conn.query(&fetch_sql, &[]);
                let _ = rst.and_then(|rows|{
                    let json = self.get_back_json(rows);
                    let rows = json_i64!(&json; "rows");
                    if rows < 1 {
                        flag = -2;
                    } else {
                        let f_back = f(json);
                        if !f_back {
                            flag = -2;
                        }
                    }
                    Result::Ok(flag)
                }).or_else(|err|{
                    println!("{}", err);
                    flag = -1;
                    Result::Err(flag)
                });
                if flag < 0 {
                    break;
                }
            }
            match flag {
                -1 => {
                    Result::Err(-1)
                },
                _ => {
                    Result::Ok(1)
                },
            }
        });

        //close the portal
        let rst = rst.and_then(|_|{
       		let close_sql = "CLOSE myportal";
	        println!("{}", close_sql);
	        let rst = conn.query(&close_sql, &[]);
	        rst.and_then(|rows|{
	            let json = self.get_back_json(rows);
	            println!("{}", json);
	            Result::Ok(1)
	        }).or_else(|err|{
	            println!("{}", err);
	            Result::Err(-1)
	        })
        });

        //end the cursor
        let rst = rst.and_then(|_|{
        	let end_sql = "END";
	        println!("{}", end_sql);
	        let rst = conn.query(&end_sql, &[]);
	        rst.and_then(|rows|{
	            let json = self.get_back_json(rows);
	            println!("{}", json);
	            Result::Ok(1)
	        }).or_else(|err|{
	            println!("{}", err);
	            Result::Err(-1)
	        })		
       	});

        rst
	}
}

pub struct MyRedisPool {
	dsn:String,
    conns:Vec<Mutex<redis::Connection>>,
}

impl MyRedisPool {
	
	pub fn new(dsn:&str, size:i64) -> MyRedisPool {
		let client = redis::Client::open(dsn).unwrap();
		let mut conns = vec![];
        for _ in 0..size {
            let conn = client.get_connection().unwrap();
            conns.push(Mutex::new(conn));
        }
        MyRedisPool {
            dsn:dsn.to_string(),
            conns:conns,
        }
	}
	
	pub fn get_conn(&self) -> Option<&Mutex<redis::Connection>> {
		let between = Range::new(0, self.conns.len());
        let mut rng = rand::thread_rng();
        let rand_int = between.ind_sample(&mut rng);
        self.conns.get(rand_int)
	}
}

pub struct DataBase<T> {
    pub name:String,
    pub table_list:BTreeMap<String, Table<T>>,
    pub dc:Arc<T>,   //data center
    pub cache: MyRedisPool,
}

impl<T:DbPool> DataBase<T> {

    fn get_table_define(name:&str, vec:Vec<Column>, dc:Arc<T>) -> Table<T>
    {
        let mut map = BTreeMap::new();
        for col in vec {
            map.insert(col.name.clone(), col);
        }
        Table::new(name, map, dc)
    }

    pub fn new(name:&str, dc:Arc<T>) -> DataBase<T>
    {
        let mut table_list = BTreeMap::new();
        {   //the user's st
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigint", -1, "unique not null", false),
                Column::new("st", "varchar", 32, "not null default ''", false),
                Column::new("fix_st", "varchar", 32, "not null default ''", false),
                Column::new("role", "integer", -1, "default -1", false),
                Column::new("last_active_time", "bigint", -1, "default -1", false),
            ];
            let table = DataBase::get_table_define("st", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {
            //终端佣金表
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigserial", -1, "", false),
                Column::new("terminal_id", "bigint", -1, "default -1", false),
                Column::new("sale_date", "int", -1, "default -1", false),
                //销售票据数
                Column::new("sale_count", "bigint", -1, "default 0", false),
                //销售额
                Column::new("sale_amount", "bigint", -1, "default 0", false),
                Column::new("term_sale_count", "bigint", -1, "default 0", false),
                Column::new("term_sale_amount", "bigint", -1, "default 0", false),
                Column::new("bonus_count", "bigint", -1, "default 0", false),
                Column::new("bonus_amount", "bigint", -1, "default 0", false),
                Column::new("bonus_err_count", "bigint", -1, "default 0", false),
                Column::new("bonus_err_amount", "bigint", -1, "default 0", false),
                Column::new("term_bonus_count", "bigint", -1, "default 0", false),
                Column::new("term_bonus_amount", "bigint", -1, "default 0", false),
                //应返佣金
                Column::new("back_amount", "bigint", -1, "default 0", false),
                //佣金返还状态
                Column::new("back_status", "int", -1, "default 0", false),
                Column::new("create_time", "bigint", -1, "default -1", false),
            ];
            let table = DataBase::get_table_define("terminal_sale", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {   //the customer
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigserial", -1, "", false),
                Column::new("username", "varchar", 40, "unique not null", false),
                Column::new("nickname", "varchar", 40, "not null default ''", true),
                Column::new("password", "varchar", 40, "not null", false),
                Column::new("reg_time", "bigint", -1, "default -1", false),
                Column::new("type", "integer", -1, "default -1", false),
                Column::new("avatar_id", "bigint", -1, "default -1", false),
                //customer's group
                Column::new("group_id", "bigint", -1, "default -1", false),
                Column::new("province", "bigint", -1, "default -1", false),
            ];
            let table = DataBase::get_table_define("customer", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        //the terminal table 
        {	
            let dc = dc.clone();
            let vec = vec![
                //用户id
                Column::new("id", "bigint", -1, "primary key", false),
                //终端机状态 
                Column::new("status", "int", -1, "default 0", false),
                //出票间隔
                Column::new("print_gap", "int", -1, "default 6", false),
                //运行模式
                Column::new("mode", "int", -1, "default 100", false),
                //类型，管理机，普通机
                Column::new("type", "int", -1, "default 0", false),
                //硬件型号
                Column::new("hard_type", "int", -1, "default -1", false),
                //软件型号
                Column::new("soft_type", "int", -1, "default -1", false),
                //服务器端额度
                Column::new("server_balance", "bigint", -1, "default 0", false),
                //客户端额度
                Column::new("client_balance", "bigint", -1, "default 0", false),
                //保证金，0，出完所有的额度
                Column::new("guarantee_amount", "int", -1, "default 100000", false),
                //代替其它终端兑奖
                Column::new("help_bonus_id", "bigint", -1, "default -1", false),
            ];
            let table = DataBase::get_table_define("terminal", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        //the terminal charge report 
        {	
            let dc = dc.clone();
            let vec = vec![
                //id
                Column::new("id", "bigserial", -1, "primary key", false),
                //终端机id 
                Column::new("terminal_id", "bigint", -1, "default -1", false),
                //unix 时间戳 
                Column::new("timestamp", "bigint", -1, "default -1", false),
                //金额
                Column::new("amount", "bigint", -1, "default 0", false),
                //订单信息
                Column::new("order_info", "varchar", 200, "default ''", true),
                //状态
                Column::new("status", "int", -1, "default 0", false),
            ];
            let table = DataBase::get_table_define("charge_report", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        //the customer account
        {
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigserial", -1, "primary key", false),	//customer_id
                Column::new("balance", "bigint", -1, "default 0", false),	//账户余额，单位(分)
                Column::new("client_balance", "bigint", -1, "default 0", false),	//终端账户余额，单位(分)
                Column::new("versoin", "bigint", -1, "default 0", false),	//记录的版本
            ];
            let table = DataBase::get_table_define("account", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {
        		let dc = dc.clone();
        		let vec = vec![
                Column::new("id", "bigserial", -1, "not null primary key", false),
                Column::new("customer_id", "bigint", -1, "default -1", false),
                Column::new("title", "varchar", 200, "not null", true),
                Column::new("content", "text", -1, "not null", true),
                Column::new("create_time", "bigint", -1, "default -1", false),
            ];
            let table = DataBase::get_table_define("document", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {   //the file table
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigserial", -1, "", false),
                Column::new("name", "varchar", 80, "not null", false),
                Column::new("create_time", "bigint", -1, "default -1", false),
                Column::new("type", "integer", -1, "default -1", false),
                Column::new("size", "bigint", -1, "default -1", false),
                Column::new("customer_id", "bigint", -1, "default -1", false),
            ];
            let table = DataBase::get_table_define("file", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {   //the file block table
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "varchar", 80, "PRIMARY KEY", false),
                Column::new("file_id", "bigint", -1, "", false),
                Column::new("customer_id", "bigint", -1, "", false),
                Column::new("start", "bigint", -1, "", false),
                Column::new("index", "int", -1, "", false),
                Column::new("size", "int", -1, "", false),
                Column::new("content", "text", -1, "not null", false),
            ];
            let table = DataBase::get_table_define("file_block", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {   //the order table
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigserial", -1, "PRIMARY KEY", false),
                Column::new("price", "bigint", -1, "default 0", false),
                Column::new("customer_id", "bigint", -1, "default -1", false),
                Column::new("status", "integer", -1, "default 0", false),
                Column::new("create_time", "bigint", -1, "default -1", false),
            ];
            let table = DataBase::get_table_define("forder", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {   //the money log table
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigserial", -1, "PRIMARY KEY", false),
                Column::new("amount", "bigint", -1, "default 0", false),
                Column::new("mbefore", "bigint", -1, "default 0", false),
                Column::new("mafter", "bigint", -1, "default 0", false),
                Column::new("type", "int", -1, "default 0", false),
                Column::new("create_time", "bigint", -1, "default -1", false),	
                Column::new("order_id", "varchar", 80, "default ''", false),	//订单的id，可用来追溯记录的来源
                Column::new("customer_id", "bigint", -1, "default -1", false),
                Column::new("status", "integer", -1, "default 0", false),
            ];
            let table = DataBase::get_table_define("moneylog", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {   //the term table
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigserial", -1, "PRIMARY KEY", false),
                Column::new("game_id", "bigint", -1, "default 0", false),
                Column::new("code", "bigint", -1, "default 0", false),
                Column::new("play_types", "varchar", 40, "default ''", false),
                Column::new("dc_play_types", "varchar", 40, "default ''", false),
                Column::new("sale_time", "bigint", -1, "default -1", false),
                Column::new("end_time", "bigint", -1, "default -1", false),
                Column::new("draw_time", "bigint", -1, "default -1", false),
                Column::new("draw_number", "varchar", 200, "default ''", false),	//开奖号码
                //主队
                Column::new("master", "varchar", 80, "default ''", true), 	
                //客队
                Column::new("guest", "varchar", 80, "default ''", true), 	
                //让球数,15=1.5,20=2,正数表示主队让球，负数表示客队让球
                Column::new("give", "integer", -1, "default 0", false),
                Column::new("status", "integer", -1, "default -1", false),
                Column::new("version", "bigint", -1, "default 0", false),
            ];
            let table = DataBase::get_table_define("term", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {   //the ticket table
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigserial", -1, "PRIMARY KEY", false),
                Column::new("game_id", "bigint", -1, "default 0", false),
                Column::new("term_code", "bigint", -1, "default 0", false),
                Column::new("term_code_list", "text", -1, "default ''", false),	//竞彩期次列表
                //算奖期次列表
                Column::new("draw_code_list", "text", -1, "default ''", false),
                Column::new("icount", "bigint", -1, "default -1", false),
                Column::new("amount", "bigint", -1, "default -1", false),
                Column::new("multiple", "int", -1, "default 1", false),
                Column::new("play_type", "int", -1, "default 0", false),
                Column::new("bet_type", "int", -1, "default 0", false),
                Column::new("out_id", "varchar", 80, "not null default ''", false),
                Column::new("number", "text", -1, "not null default ''", false),	//投注号码
                Column::new("print_number", "text", -1, "not null default ''", false),	//jc出票之后的号码
                //票根信息
                Column::new("stub", "text", -1, "not null default ''", true),
                //加密之后的票根
                Column::new("crypto", "text", -1, "not null default ''", false),
                Column::new("bonus_stub", "text", -1, "not null default ''", true),	//票根信息
                Column::new("seq", "varchar", 120, "not null default ''", false),	//出票序列号
                Column::new("create_time", "bigint", -1, "default -1", false),
                //截止时间
                Column::new("end_time", "bigint", -1, "default -1", false),
                Column::new("print_time", "bigint", -1, "default -1", false),
                Column::new("customer_id", "bigint", -1, "default 0", false),
                Column::new("terminal_id", "bigint", -1, "default 0", false),
                Column::new("bonus_terminal_id", "bigint", -1, "default -1", false),
                Column::new("status", "int", -1, "default 0", false),
                Column::new("bonus", "bigint", -1, "default 0", false),
                Column::new("bonus_after_tax", "bigint", -1, "default 0", false),
                Column::new("bonus_detail", "text", -1, "default ''", false),
                Column::new("bonus_try_count", "int", -1, "default 0", false),
                Column::new("bonus_time", "bigint", -1, "default -1", false),
                Column::new("version", "int", -1, "default 0", false),
            ];
            let table = DataBase::get_table_define("ticket", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        //出票池
        {
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigint", -1, "PRIMARY KEY", false),
                Column::new("game_id", "bigint", -1, "default 0", false),
                Column::new("icount", "bigint", -1, "default -1", false),
                Column::new("amount", "bigint", -1, "default -1", false),
                Column::new("multiple", "int", -1, "default 1", false),
                Column::new("play_type", "int", -1, "default 0", false),
                Column::new("bet_type", "int", -1, "default 0", false),
                Column::new("number", "text", -1, "not null default ''", false),	//投注号码
                //截止时间
                Column::new("end_time", "bigint", -1, "default -1", false),
                //发送时间
                Column::new("send_time", "bigint", -1, "default -1", false),
                Column::new("status", "int", -1, "default 0", false),
                Column::new("version", "int", -1, "default 0", false),
            ];
            let table = DataBase::get_table_define("print_pool", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {   //the terminal game table
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigserial", -1, "PRIMARY KEY", false),
                Column::new("terminal_id", "bigint", -1, "default 0", false),
                Column::new("game_id", "bigint", -1, "default 0", false),
                Column::new("create_time", "bigint", -1, "default -1", false),
                //佣金比例,80/1000
                Column::new("scale", "int", -1, "default 80", false),
                Column::new("version", "int", -1, "default 0", false),
            ];
            let table = DataBase::get_table_define("terminal_game", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {   //游戏奖级表
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigserial", -1, "PRIMARY KEY", false),
                Column::new("game_id", "bigint", -1, "default 0", false),	//游戏编号
                Column::new("term_code", "bigint", -1, "default 0", false),	//期次号
                Column::new("lev", "int", -1, "default 0", false),	//奖级
                Column::new("descrip", "varchar", 80, "default ''", true),	//奖级描述
                Column::new("bonus", "bigint", -1, "default 0", false),	//奖金额（税前）
                Column::new("bonus_after_tax", "bigint", -1, "default 0", false),	//奖金额（税后）
                Column::new("create_time", "bigint", -1, "default -1", false),
                Column::new("version", "int", -1, "default 0", false),
            ];
            let table = DataBase::get_table_define("game_level", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        for (_, table) in table_list.iter() {
            info!("{}", table.to_ddl_string());
        }
        let redis_dsn = cfg_str!("redis", "dsn");
    		let redis_conn_limit = cfg_i64!("redis", "conn_limit");
    		let cache = MyRedisPool::new(redis_dsn, redis_conn_limit);
        
        DataBase {
            name: name.to_string(),
            table_list: table_list,
            dc: dc,
            cache: cache,
        }
    }

    pub fn get_table(&self, name:&str) -> Option<&Table<T>>
    {
        self.table_list.get(name)
    }

    pub fn execute(&self, sql:&str) -> Result<Json, i32> {
        self.dc.execute(&sql)
    }
	
	pub fn stream<F>(&self, sql:&str, f:F) -> Result<i32, i32> where F:FnMut(Json) -> bool + 'static {
		self.dc.stream(sql, f)
	}
		
    //get a new connection from db
    pub fn get_connection(&self) -> Result<Connection, i32> {
        self.dc.get_connection()
    }
}

///call sql with stream result, once a row
pub fn stream<F>(conn:Connection, sql:&str, mut f:F) -> Result<i32, i32> where F:FnMut(Json) -> bool + 'static {
    let rst = conn.query("BEGIN", &[]);
    let rst = rst.or_else(|err| {
        println!("{}", err);
        Result::Err(-1)
    });
    let rows = try!(rst);
    let json = rows_to_json(rows);
    println!("{}", json);

  	let cursor_sql = format!("DECLARE myportal CURSOR FOR {}", sql);
   	println!("{}", cursor_sql);
   	let rst = conn.query(&cursor_sql, &[]);
    let rst = rst.or_else(|err| {
        println!("{}", err);
        Result::Err(-1)
    });
    let rows = try!(rst);
    let json = rows_to_json(rows);
    println!("{}", json);

    let fetch_sql = "FETCH NEXT in myportal";
    println!("{}", fetch_sql);

    loop {
        let rst = conn.query(&fetch_sql, &[]);
        let rst = rst.or_else(|err| {
            println!("{}", err);
            Result::Err(-1)
        });
        let rows = try!(rst);
        let mut json = rows_to_json(rows);
        let mut json_obj = json.as_object_mut().unwrap();
        let rows_node = json_obj.remove("rows").unwrap();
        let row_count = rows_node.as_i64().unwrap();
        if row_count < 1 {
            break;
        }
        let mut data_node = json_obj.remove("data").unwrap();
        let mut array = data_node.as_array_mut().unwrap();
        let data = array.remove(0);
        let f_back = f(data);
        if !f_back {
            break;
        }
    }

    //close the portal
   	let close_sql = "CLOSE myportal";
    println!("{}", close_sql);
    let rst = conn.query(&close_sql, &[]);
    let rst = rst.or_else(|err| {
        println!("{}", err);
        Result::Err(-1)
    });
    let rows = try!(rst);
    let json = rows_to_json(rows);
    println!("{}", json);

    //end the cursor
    let end_sql = "END";
	println!("{}", end_sql);
	let rst = conn.query(&end_sql, &[]);
    let rst = rst.or_else(|err| {
        println!("{}", err);
        Result::Err(-1)
    });
    let rows = try!(rst);
    let json = rows_to_json(rows);
    println!("{}", json);

    Result::Ok(1)
}

