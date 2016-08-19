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
use sev_helper::PrintService;
use sev_helper::CustomerService;

//终端机登录
pub struct T01;

impl DataApi for T01 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = KeyHelper::from_db(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        //info!("{}", msg);
        let key = json_str!(msg; "key");
        let username = json_str!(msg; "head", "userId");
        
        let table = db.get_table("customer").expect("table not exists.");
        let mut cond = json!("{}");
        json_set!(&mut cond; "username"; username);
        let fd_back = try!(table.find_one(&cond, &json!("{}"), &json!("{}")));
        let user_id = json_i64!(&fd_back; "id");

        let table = db.get_table("terminal").unwrap();

        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; user_id);
        json_set!(&mut cond; "status"; CONS.code_to_id("terminal_status", "offline").unwrap());

        let mut doc = json!("{}");
        let mut set_data = json!("{}");
        json_set!(&mut set_data; "status"; CONS.code_to_id("terminal_status", "online").unwrap());
        json_set!(&mut doc; "$set"; set_data);

        let mut op = json!("{}");
        let mut ret = json!("{}");
        json_set!(&mut ret; "type"; 1);
        json_set!(&mut op; "ret"; ret);

        let fd_back = try!(table.update(&cond, &doc, &op));
        let rows = json_i64!(&fd_back; "rows");
        if rows == 0 {
            return Result::Err(ErrCode::UserInfoIsWrong as i32);
        }

        let mut back_body = json!("{}");
        json_set!(&mut back_body; "userId"; user_id);
 		Result::Ok(back_body)
    }
}

///终端机请求重新出票
pub struct T11;

impl DataApi for T11 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = KeyHelper::from_db(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let id = json_i64!(msg; "body", "id");
        let username = json_str!(msg; "head", "userId");
        let customer = try!(CustomerService.get_by_username(username, db));
        let customer_id = json_i64!(&customer; "id");

        PrintService.reprint(id, customer_id, db)
    }
}

///终端机上传票根信息
pub struct T12;

impl DataApi for T12 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = KeyHelper::from_db(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let username = json_str!(msg; "head", "userId");
        let customer = try!(CustomerService.get_by_username(username, db));
        let customer_id = json_i64!(&customer; "id");

        let body = json_path!(msg; "body");
        let rst = PrintService.back(customer_id, body, db);
        rst.and(Result::Ok(json!("{}")))
    }
}

///终端机抓取期次开期，开奖，销售报表等信息
pub struct T13;

impl DataApi for T13 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = KeyHelper::from_db(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let username = json_str!(msg; "head", "userId");
        let stub = json_str!(msg; "body", "stub");
        info!("{}", msg);
        Result::Ok(json!("{}"))
    }
}
