use std::collections::BTreeMap;
use std::sync::{Arc};

#[macro_use]
extern crate easy_config;

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::str::FromStr;

#[macro_use]
extern crate log;
extern crate elog;

extern crate util;
use util::*;

extern crate cons;
use cons::CONS;

extern crate game;
use game::DF;

extern crate sev_helper;
use sev_helper::TermService;
use sev_helper::TicketService;

extern crate dc;
use dc::MyDbPool;
use dc::DataBase;

extern crate easydb;
use easydb::DbPool;

extern crate hyper;
extern crate regex;

extern crate chrono;
use chrono::*;

pub fn run_bonus(date:i64, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
    let year = (date/10000) as i32;
    let month_date = (date%10000) as u32;
    let month = month_date/100;
    let day = (month_date%100) as u32;
    info!("{},{},{}", year, month, day);
    let start_date = Local.ymd(year, month, day).and_hms(0, 0, 0);
    info!("{}", start_date);
    let end_date = start_date + Duration::days(1);
    info!("{}", end_date);

    let start_timestamp = start_date.timestamp();
    let end_timestamp = end_date.timestamp();
    let status_cond = "status = 70";
    let sql = format!("select bonus_terminal_id as terminal_id,sum(amount)::bigint as bonus_amount,count(*)::bigint as bonus_count from ticket where bonus_time >= {} and bonus_time < {} and {} group by bonus_terminal_id", start_timestamp, end_timestamp, status_cond);
    info!("{}", sql);
    let rst = try!(db.execute(&sql));
    let mut data = get_data(rst);
    let mut list = data.as_array_mut().unwrap();
    loop {
        let row_op = list.pop();
        if row_op.is_none() {
            break;
        }
        let row = row_op.unwrap();
        let terminal_id = {
            let node = row.find("terminal_id").unwrap();
            node.as_i64().unwrap()
        };
        let bonus_amount = {
            let node = row.find("bonus_amount").unwrap();
            node.as_i64().unwrap()
        };
        let bonus_count = {
            let node = row.find("bonus_count").unwrap();
            node.as_i64().unwrap()
        };
        let sql = format!("insert into terminal_sale(terminal_id, sale_date, bonus_count, bonus_amount) values({}, {}, {}, {}) ON CONFLICT (terminal_id, sale_date) do update set bonus_count={}, bonus_amount={}", terminal_id, date, bonus_count, bonus_amount, bonus_count, bonus_amount);
        info!("{}", sql);
        let rst = try!(db.execute(&sql));
        info!("{}", rst);
    }
    Result::Ok(1)
}

pub fn run_bonus_err(date:i64, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
    let status_cond = "status=65";
    let sql = format!("select terminal_id,sum(bonus)::bigint as bonus_err_amount,count(*)::bigint as bonus_err_count from ticket where {} group by terminal_id", status_cond);
    info!("{}", sql);
    let rst = try!(db.execute(&sql));
    let mut data = get_data(rst);
    let mut list = data.as_array_mut().unwrap();
    loop {
        let row_op = list.pop();
        if row_op.is_none() {
            break;
        }
        let row = row_op.unwrap();
        let terminal_id = {
            let node = row.find("terminal_id").unwrap();
            node.as_i64().unwrap()
        };
        let bonus_err_amount = {
            let node = row.find("bonus_err_amount").unwrap();
            node.as_i64().unwrap()
        };
        let bonus_err_count = {
            let node = row.find("bonus_err_count").unwrap();
            node.as_i64().unwrap()
        };
        let sql = format!("insert into terminal_sale(terminal_id, sale_date, bonus_err_count, bonus_err_amount) values({}, {}, {}, {}) ON CONFLICT (terminal_id, sale_date) do update set bonus_err_count={}, bonus_err_amount={}", terminal_id, date, bonus_err_count, bonus_err_amount, bonus_err_count, bonus_err_amount);
        info!("{}", sql);
        let rst = try!(db.execute(&sql));
        info!("{}", rst);
    }
    Result::Ok(1)
}

pub fn run_sale(date:i64, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
    let year = (date/10000) as i32;
    let month_date = (date%10000) as u32;
    let month = month_date/100;
    let day = (month_date%100) as u32;
    info!("{},{},{}", year, month, day);
    let start_date = Local.ymd(year, month, day).and_hms(0, 0, 0);
    info!("{}", start_date);
    let end_date = start_date + Duration::days(1);
    info!("{}", end_date);

    let start_timestamp = start_date.timestamp();
    let end_timestamp = end_date.timestamp();
    let status_cond = "status != 10 and status != 15 and status != 50";
    let sql = format!("select terminal_id,sum(amount)::bigint as sale_amount,count(*)::bigint as sale_count from ticket where print_time >= {} and print_time < {} and {} group by terminal_id", start_timestamp, end_timestamp, status_cond);
    info!("{}", sql);
    let rst = try!(db.execute(&sql));
    let mut data = get_data(rst);
    let mut list = data.as_array_mut().unwrap();
    loop {
        let row_op = list.pop();
        if row_op.is_none() {
            break;
        }
        let row = row_op.unwrap();
        let terminal_id = {
            let node = row.find("terminal_id").unwrap();
            node.as_i64().unwrap()
        };
        let sale_amount = {
            let node = row.find("sale_amount").unwrap();
            node.as_i64().unwrap()
        };
        let sale_count = {
            let node = row.find("sale_count").unwrap();
            node.as_i64().unwrap()
        };
        let sql = format!("insert into terminal_sale(terminal_id, sale_date, sale_count, sale_amount) values({}, {}, {}, {}) ON CONFLICT (terminal_id, sale_date) do update set sale_count={}, sale_amount={}", terminal_id, date, sale_count, sale_amount, sale_count, sale_amount);
        info!("{}", sql);
        let rst = try!(db.execute(&sql));
        info!("{}", rst);
    }
    Result::Ok(1)
}

