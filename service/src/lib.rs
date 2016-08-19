use std::collections::BTreeMap;
use std::io::Read;

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::str::FromStr;

extern crate util;
use self::util::DigestUtil;

extern crate sev_helper;

extern crate dc;
use self::dc::DataBase;
use self::dc::MyDbPool;

extern crate cons;
use self::cons::ErrCode;

extern crate chrono;

extern crate time;

extern crate regex;
use self::regex::Regex;

pub mod inter;
use self::inter::{DataApi};

pub mod sv_util;
use self::sv_util::{KeyHelper};
pub use self::sv_util::get_jcc_draw_info;

#[macro_use]
extern crate log;
extern crate elog;

extern crate game;
extern crate hyper;
extern crate xml;

mod ment;
use self::ment::admin::*;
use self::ment::admin_account::*;

use self::ment::admin_center::*;
use self::ment::admin_company::*;
use self::ment::admin_doc::*;
use self::ment::admin_game::*;
use self::ment::admin_report::*;
use self::ment::admin_terminal::*;
use self::ment::admin_ticket::*;

use self::ment::center::*;

use self::ment::company::*;
use self::ment::company_ticket::*;

use self::ment::user::*;
use self::ment::file::*;
use self::ment::terminal::*;

macro_rules! add_inter {
    ($o:expr, $k:expr, $v:expr) => {{
        $o.insert($k.to_string(), Box::new($v) as Box<DataApi>);
    }}
}

pub struct ApiFactory {
    map:BTreeMap<String, Box<DataApi>>,
}

impl ApiFactory {

    pub fn new() -> ApiFactory {
        let mut map = BTreeMap::new();
        
        add_inter!(map, "A01", A01);
        
        add_inter!(map, "AA03", AA03);
        add_inter!(map, "AA04", AA04);
        add_inter!(map, "AA05", AA05);
        add_inter!(map, "AA06", AA06); //终端缴费信息
        add_inter!(map, "AA07", AA07); //从缴费信息充值
        add_inter!(map, "AA08", AA08); //矫正余额
        
        add_inter!(map, "AC01", AC01);
        add_inter!(map, "AC02", AC02);
        add_inter!(map, "AC03", AC03);
        add_inter!(map, "AC04", AC04);

        add_inter!(map, "ACT01", ACT01);    //新增出票中心
        add_inter!(map, "ACT02", ACT02);    //获得出票中心列表
        add_inter!(map, "ACT03", ACT03);    //获得出票中心销售报表
        add_inter!(map, "ACT04", ACT04);    //获得中心详情
        add_inter!(map, "ACT05", ACT05);    //更新中心详情

        add_inter!(map, "AD01", AD01);
        add_inter!(map, "AD02", AD02);
        add_inter!(map, "AD03", AD03);
        
        add_inter!(map, "AG01", AG01);	//获得所有游戏列表
        add_inter!(map, "AGT01", AGT01);	//添加游戏期次
        add_inter!(map, "AGT02", AGT02);	//获得游戏期次列表
        add_inter!(map, "AGT03", AGT03);	//更新期次信息
        add_inter!(map, "AGT04", AGT04);	//根据id获得游戏期次信息
        add_inter!(map, "AGT05", AGT05);	//获得游戏期次的奖级信息
        add_inter!(map, "AGT06", AGT06);	//保存，游戏期次的奖级信息
        add_inter!(map, "AGT07", AGT07);	//期次开奖
        add_inter!(map, "AGT08", AGT08);	//dlt抓取开奖号码

        add_inter!(map, "AR01", AR01);     //获取系统概况报表
        add_inter!(map, "AR02", AR02);     //获取系统日销售报表
        add_inter!(map, "AR03", AR03);     //获取系统日销售报表，销售方角度
        add_inter!(map, "AR04", AR04);     //获取终端机返佣记录
        
        add_inter!(map, "AT01", AT01);
        add_inter!(map, "AT02", AT02);
        add_inter!(map, "AT03", AT03);
        add_inter!(map, "AT04", AT04);	//获得彩票机能出的游戏
        add_inter!(map, "AT05", AT05);	//添加彩票机能出的游戏
        add_inter!(map, "AT06", AT06);	//删除彩票机能出的游戏
        add_inter!(map, "AT07", AT07);	//查询系统模式
        add_inter!(map, "AT08", AT08);	//切换系统模式
        add_inter!(map, "AT09", AT09);	//设置系统出票间隔
        add_inter!(map, "AT10", AT10);	//设置彩票机类型
        add_inter!(map, "AT11", AT11);	//查询游戏在售期次
        add_inter!(map, "AT12", AT12);	//设置彩票机出票间隔
        add_inter!(map, "AT13", AT13);	//设置彩票机游戏返点
        
        add_inter!(map, "ATI01", ATI01);	//获得票据列表
        add_inter!(map, "ATI02", ATI02);	//获得票据详情
        add_inter!(map, "ATI03", ATI03);	//重新出票
        add_inter!(map, "ATI04", ATI04);	//退款
        add_inter!(map, "ATI05", ATI05);	//兑奖
        add_inter!(map, "ATI06", ATI06);	//出票错误
        add_inter!(map, "ATI07", ATI07);	//手动兑奖成功
        add_inter!(map, "ATI08", ATI08);	//兑奖异常置成中大奖
        add_inter!(map, "ATI09", ATI09);	//重出所有的异常票
        add_inter!(map, "ATI10", ATI10);	//重兑所有的兑奖异常票
        add_inter!(map, "ATI11", ATI11);	//一键退款
        add_inter!(map, "ATI12", ATI12);	//出票池列表
        add_inter!(map, "ATI13", ATI13);	//出票池重新发送到缓存
        
        add_inter!(map, "C01", C01);		

        add_inter!(map, "CR01", CR01);	    //出票中心登录
        add_inter!(map, "CR02", CR02);	    //获得系统常量
        add_inter!(map, "CR03", CR03);	    //获得兑奖异常票据
        add_inter!(map, "CR04", CR04);	    //返回兑奖异常票据处理结果
        add_inter!(map, "CR05", CR05);	    //返回异常票据票根
        add_inter!(map, "CR06", CR06);	    //根据id获取票据信息

        add_inter!(map, "CRT01", CRT01);	//出票中心获得终端机列表
        add_inter!(map, "CRT02", CRT02);	//出票中心添加彩票机
        add_inter!(map, "CRT03", CRT03);	//出票中心查询彩票机缴款报表

        add_inter!(map, "CT01", CT01);	//销售方投注接口
        add_inter!(map, "CT02", CT02);	//销售方查询票据状态
        add_inter!(map, "CT03", CT03);	//销售方查询票据状态(批量)
        add_inter!(map, "CT04", CT04);	//销售方余额查询
        
        add_inter!(map, "F01", F01);
        add_inter!(map, "F02", F02);
        add_inter!(map, "F03", F03);
        add_inter!(map, "F04", F04);
        add_inter!(map, "U01", U01);
        
        add_inter!(map, "T01", T01);
        add_inter!(map, "T11", T11);
        add_inter!(map, "T12", T12);
        add_inter!(map, "T13", T13); //获得终端机上报的期次等信息
        
        ApiFactory {
            map:map,
        }
    }

    /**
     * get the digest key by head.
     */
    pub fn get_key(&self, db:&DataBase<MyDbPool>, head:&Json) -> Result<String, i32> {
        let name = json_str!(head; "cmd");
        let cmd_op = self.map.get(name);
        let rst = cmd_op.ok_or(ErrCode::ApiNotExits as i32);
        let api = try!(rst);
        api.get_key(db, head)
    }

    /**
     * check the digest. If success return Some, else return None.
     */
    pub fn check(&self, db:&DataBase<MyDbPool>, param:&BTreeMap<String, String>) -> Result<Json, i32> {
        let head = param.get("head").unwrap();
        let head_node = json!(head);
        let digest = json_str!(&head_node; "digest");
        let time_stamp = json_str!(&head_node; "timeStamp");

        let key_rst = self.get_key(db, &head_node);
        key_rst.and_then(|key| {
        	info!("the key is {}.", key);
            let body_str = param.get("body").unwrap();
            let digest_content = format!("{}{}{}", key, body_str, time_stamp);
            if digest == DigestUtil::md5(&digest_content) {
                let body_node = json!(body_str);
                let mut back_obj = json!("{}");
                json_set!(&mut back_obj; "head"; head_node.clone());
                json_set!(&mut back_obj; "body"; body_node);
                json_set!(&mut back_obj; "key"; key);
                Result::Ok(back_obj)
            }
            else
            {
                Result::Err(ErrCode::DigestFailure as i32)
            }
        })
    }

    pub fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let name = json_str!(msg; "head", "cmd");
        let api_rst = self.map.get(name).ok_or(ErrCode::ApiNotExits as i32);
        let api = try!(api_rst);
        let rst = api.check(db, msg);
        let rst = rst.and_then(|_|{
       		api.run(db, msg)
        });
        rst
    }

    pub fn back(&self, mut head:Json, key:String, body:String) -> Json {
       	let time = json_str!(&head; "timeStamp").to_string();
        let digest_content = format!("{}{}{}", key, body, time);
        info!("{}", digest_content);
        let digest = DigestUtil::md5(&digest_content);
        info!("{}", digest);
        json_set!(&mut head; "digest"; digest);
        let mut back_msg = json!("{}");
        json_set!(&mut back_msg; "head"; head);
        json_set!(&mut back_msg; "body"; body);
        back_msg
    }
    
    pub fn get_back_head(&self, msg:&Json, body:&str) -> Json {
        let head = json_path!(msg; "head");
        let time = json_str!(head; "timeStamp");
        let key = json_str!(msg; "key");

        let digest_content = format!("{}{}{}", key, body, time);
        info!("{}", digest_content);
        let digest = DigestUtil::md5(&digest_content);
        info!("{}", digest);

        let mut back_head = head.clone();
        {
            json_set!(&mut back_head; "digest"; digest);
        }
        back_head
    }

    pub fn back_err(&self, head:&Json, body:String) -> Json {
        let mut back_head = head.clone();
        {
            let time = json_str!(head; "timeStamp");
            let key = DigestUtil::empty_key();
            let digest_content = format!("{}{}{}", key, body, time);
            let digest = DigestUtil::md5(&digest_content);

            json_set!(&mut back_head; "digestType"; "md5-empty");
            json_set!(&mut back_head; "digest"; digest);
        }
        let mut back_obj = json!("{}");
        json_set!(&mut back_obj; "head"; back_head);
        json_set!(&mut back_obj; "body"; body);
        back_obj
    }
}

