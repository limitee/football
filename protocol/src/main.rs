extern crate protocol;
use protocol::Protocol;

use std::io::prelude::*;
use std::net::TcpStream;
use std::io::Error;
use std::io::Cursor;

use std::sync::{Arc};

extern crate byteorder;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;
use std::str::FromStr;

#[macro_use]
extern crate log;
extern crate elog;

#[macro_use]
extern crate easy_config;
use easy_config::CFG;

extern crate util;
use self::util::DigestUtil;

extern crate chrono;
use chrono::*;

use std::thread;

fn get_stub(game_id:i64, play_type:i64) -> String {
	let stub;
	if game_id == 202_i64 {
		match play_type {
			1 => {
				stub = "中国体育彩票演示票\n演示票竞彩足球胜平负   单场固定\n200243-945712-189922-67 FD362CCE 00546858\n─────────────────────\n周日001\n主队:皇家马德里 Vs 客队:巴塞罗那\n胜@2.045元+平@1.999元+负@12.012元\n(选项固定奖金额为每1元投注对应的奖金额)\n本票最高可能固定奖金:268.00元\n *  *  *\n *  *  *\n─────────────────────\n倍数:1  合计:    样票  2016-04-16 17:22:40\n演示票，请勿用于销售！！！\r\n中国竞彩网 http://www.sporttery.cn\n\r\n2001021D32303032343339343537313231383939323236372030303534363835".to_string();
			},
            2 => {
				stub = "中国体育彩票演示票\n演示票竞彩足球胜平负   单场固定\n200243-945712-189922-67 FD362CCE 00546858\n─────────────────────\n周日001\n主队:皇家马德里 Vs 客队:巴塞罗那\n胜@12.0元+平@12.0元+负@12.0元\n(选项固定奖金额为每1元投注对应的奖金额)\n本票最高可能固定奖金:268.00元\n *  *  *\n *  *  *\n─────────────────────\n倍数:1  合计:    样票  2016-04-16 17:22:40\n演示票，请勿用于销售！！！\r\n中国竞彩网 http://www.sporttery.cn\n\r\n2001021D32303032343339343537313231383939323236372030303534363835".to_string();
			},
            3 => {
				stub = "中国体育彩票演示票\n演示票竞彩足球胜平负   单场固定\n200243-945712-189922-67 FD362CCE 00546858\n─────────────────────\n周日001\n主队:皇家马德里 Vs 客队:巴塞罗那\n(1)@12.0元+(2)@12.0元+(3)@12.0元\n(选项固定奖金额为每1元投注对应的奖金额)\n本票最高可能固定奖金:268.00元\n *  *  *\n *  *  *\n─────────────────────\n倍数:1  合计:    样票  2016-04-16 17:22:40\n演示票，请勿用于销售！！！\r\n中国竞彩网 http://www.sporttery.cn\n\r\n2001021D32303032343339343537313231383939323236372030303534363835".to_string();
			},
            4 => {
				stub = "中国体育彩票演示票\n演示票竞彩足球胜平负   单场固定\n200243-945712-189922-67 FD362CCE 00546858\n─────────────────────\n周日001\n主队:皇家马德里 Vs 客队:巴塞罗那\n(1:0)@12.0元+(2:0)@12.0元+(1:3)@12.0元\n(选项固定奖金额为每1元投注对应的奖金额)\n本票最高可能固定奖金:268.00元\n *  *  *\n *  *  *\n─────────────────────\n倍数:1  合计:    样票  2016-04-16 17:22:40\n演示票，请勿用于销售！！！\r\n中国竞彩网 http://www.sporttery.cn\n\r\n2001021D32303032343339343537313231383939323236372030303534363835".to_string();
			},

			5 => {
				stub = "中国体育彩票演示票\n演示票竞彩足球半全场胜平负   单场固定\n200243-945712-189922-67 DB4D0D05 00546858\n─────────────────────\n周日001\n主队:皇家马德里 Vs 客队:巴塞罗那\n胜胜@12.0元+平平@12.0元+负负@12.0元\n(选项固定奖金额为每1元投注对应的奖金额)\n本票最高可能固定奖金:268.00元\n *  *  *\n *  *  *\n─────────────────────\n倍数:1  合计:    样票  2016-04-18 12:27:08\n演示票，请勿用于销售！！！\r\n中国竞彩网 http://www.sporttery.cn\n\r\n2001021D32303032343339343537313231383939323236372030303534363835 ".to_string();
			},
			_ => {
				stub = "".to_string();
			}
		}
	} else if game_id == 201_i64 {
		match play_type {
		    1 => {
				stub = "中国体育彩票演示票\n演示票竞彩足球胜平负   单场固定\n200243-945712-189922-67 FD362CCE 00546858\n─────────────────────\n周日001\n主队:皇家马德里 Vs 客队:巴塞罗那\n胜@1.909元+平@2.999元+负@2.918元\n周日002\n主队:皇家马德里 Vs 客队:巴塞罗那\n平@1.042元+胜@2.009元+负@2.004元\n".to_string();
		    },
			10 => {
				stub = "中国体育彩票演示票\n演示票竞彩足球胜平负   单场固定\n200243-945712-189922-67 FD362CCE 00546858\n─────────────────────\n周日001\n主队:皇家马德里 Vs 客队:巴塞罗那\n胜胜@12.9元\n周日002\n主队:皇家马德里 Vs 客队:巴塞罗那\n平@12.0元+胜@2.0元\n".to_string();
			},
            _ => {
                stub = "".to_string();
            }
		}
    } else if game_id == 200 {
		stub = "体彩<超级大乐透>\r\n110428-927000-040826-093304 394806 4hfksA\r\n第 16063期    共1期         16/06/01开奖\r\n09-090116-101 00133     16/05/31 12:43:06\r\n 1倍 合计2元\r\n         前区                      后区\r\n① 10+13+22+26+33 04+05\r\n②  .  .  .  .  .  .  .\r\n③  .  .  .  .  .  .  .\r\n④  .  .  .  .  .  .  .\r\n⑤  .  .  .  .  .  .  .\r\n竞彩欧洲杯 梦想GOAL精彩！！！\r\n孝感市开发区槐荫办事处董永社区\r\n\r\n590101110428927000040826093304394806473045022100E2402CC6C69523AD3AFA960BF7B99811E5F3410D726C8AED931DBBEA87A32F4F02202C00E5D5D7F8F9165315F468973B009F5BDC4E37959680921C0F6B8A007046".to_string();
	} else if game_id == 203 {
		stub = "体彩<排列3>\r\n110429-348100-048467-032700 376396 ggGQSg\r\n第 16063期    共1期         16/06/01开奖\r\n09-090116-101 00133     16/05/31 12:43:06\r\n 1倍 合计2元\r\n         前区                      后区\r\n① 10+13+22+26+33 04+05\r\n②  .  .  .  .  .  .  .\r\n③  .  .  .  .  .  .  .\r\n④  .  .  .  .  .  .  .\r\n⑤  .  .  .  .  .  .  .\r\n竞彩欧洲杯 梦想GOAL精彩！！！\r\n孝感市开发区槐荫办事处董永社区\r\n\r\n590101110428927000040826093304394806473045022100E2402CC6C69523AD3AFA960BF7B99811E5F3410D726C8AED931DBBEA87A32F4F02202C00E5D5D7F8F9165315F468973B009F5BDC4E37959680921C0F6B8A007046".to_string();
    } else {
		stub = "体彩<超级大乐透>\r\n110429-348100-048467-032700 376396 ggGQSg\r\n第 16063期    共1期         16/06/01开奖\r\n09-090116-101 00133     16/05/31 12:43:06\r\n 1倍 合计2元\r\n         前区                      后区\r\n① 10+13+22+26+33 04+05\r\n②  .  .  .  .  .  .  .\r\n③  .  .  .  .  .  .  .\r\n④  .  .  .  .  .  .  .\r\n⑤  .  .  .  .  .  .  .\r\n竞彩欧洲杯 梦想GOAL精彩！！！\r\n孝感市开发区槐荫办事处董永社区\r\n\r\n590101110428927000040826093304394806473045022100E2402CC6C69523AD3AFA960BF7B99811E5F3410D726C8AED931DBBEA87A32F4F02202C00E5D5D7F8F9165315F468973B009F5BDC4E37959680921C0F6B8A007046".to_string();
    }
	stub
}

fn get_term_stub(game_id:i64) -> String {
    if game_id == 201 {
        "竞彩足球对阵表\n赛事编号   主队VS客队  让球   比赛时间\n周一008   西班牙VS捷克\n主-1   16-06-13 21:00\n周一009   爱尔兰VS瑞典\n主+1   16-06-14 00:00\n周一010   比利时VS意大利\n主-1   16-06-14 03:00\n周一027   俄罗斯VS威尔士\n主-1   16-06-21 03:00\n周一028   斯洛伐克VS英格兰\n主+1   16-06-21 03:00\n周一101   首尔FCVS济州联\n主-1   16-06-06 16:50\n周一105   巴拿马VS玻利维亚\n主-1   16-06-07 07:00\n周一106   阿根廷VS智利\n主-1   16-06-07 10:00\n周二011   奥地利VS匈牙利\n主-1   16-06-15 00:00\n周二012   葡萄牙VS冰岛\n主-1   16-06-15 03:00\n周二029   乌克兰VS波兰\n主-1   16-06-22 00:00\n周二030   北爱尔兰VS德国\n主+1   16-06-22 00:00\n周二031   捷克VS土耳其\n主+1   16-06-22 03:00\n周二032   克罗地亚VS西班牙\n主+1   16-06-22 03:00\n周三013   俄罗斯VS斯洛伐克\n主-1   16-06-15 21:00\n周三014   罗马尼亚VS瑞士\n主+1   16-06-16 00:00\n周三015   法国VS阿尔巴尼亚\n主-1   16-06-16 03:00\n周三033   冰岛VS奥地利\n主+1   16-06-23 00:00\n周三034   匈牙利VS葡萄牙\n主+1   16-06-23 00:00\n周三035   意大利VS爱尔兰\n主-1   16-06-23 03:00\n周三036   瑞典VS比利时\n主+1   16-06-23 03:00\n周四016   英格兰VS威尔士\n主-1   16-06-16 21:00\n周四017   乌克兰VS北爱尔兰\n主-1   16-06-17 00:00\n周四018   德国VS波兰\n主-1   16-06-17 03:00\n周五001   法国VS罗马尼亚\n主-1   16-06-11 03:00\n周五019   意大利VS瑞典\n主-1   16-06-17 21:00\n周五020   捷克VS克罗地亚\n主+1   16-06-18 00:00\n周五021   西班牙VS土耳其\n主-1   16-06-18 03:00\n周五101   波黑VS丹麦\n主+1   16-06-03 15:00\n周五102   日本VS保加利亚\n主-1   16-06-03 18:40\n周五103   中国VS特立尼达和多巴哥\n主-1   16-06-03 19:35\n周五104   瑞士VS摩尔多瓦\n主-1   16-06-04 00:00\n周五105   罗马尼亚VS格鲁吉亚\n主-1   16-06-04 02:00\n周五106   阿尔巴尼亚VS乌克兰\n主+1   16-06-04 02:30\n周五107   美国VS哥伦比亚\n主+1   16-06-04 09:30\n周六002   阿尔巴尼亚VS瑞士\n主+1   16-06-11 21:00\n周六003   威尔士VS斯洛伐克\n主-1   16-06-12 00:00\n周六004   英格兰VS俄罗斯\n主-1   16-06-12 03:00\n周六022   比利时VS爱尔兰\n主-1   16-06-18 21:00\n周六023   冰岛VS匈牙利\n主-1   16-06-19 00:00\n周六024   葡萄牙VS奥地利\n主-1   16-06-19 03:00\n周六101   冈山绿雉VS熊本深红\n主-1   16-06-04 12:00\n周六102   岐阜FCVS长崎航海\n主+1   16-06-04 12:00\n周六103   山口雷诺法VS东京绿茵\n主-1   16-06-04 12:00\n周六104   山形山神VS横滨FC\n主-1   16-06-04 13:00\n周六105   松本山雅VS北九州向日葵\n主-1   16-06-04 14:00\n周六106   大阪樱花VS赞岐釜玉海\n主-1   16-06-04 15:00\n周六107   町田泽维亚VS德岛漩涡\n主-1   16-06-04 15:00\n周六108   水户蜀葵VS清水鼓动\n主+1   16-06-04 15:00\n周六109   金泽塞维根VS京都不死鸟\n主+1   16-06-04 17:00\n周六111   札幌冈萨多VS千叶市原\n主-1   16-06-04 18:00\n周六112   光州FCVS全北现代\n主+1   16-06-04 18:00\n周六113   群马温泉VS爱媛FC\n主+1   16-06-04 18:30\n周六120   哥斯达黎加VS巴拉圭\n主+1   16-06-05 05:00\n周六121   海地VS秘鲁\n主+1   16-06-05 07:30\n周六124   巴西VS厄瓜多尔\n主-1   16-06-05 10:00\n周日005   土耳其VS克罗地亚\n主+1   16-06-12 21:00\n周日006   波兰VS北爱尔兰\n主-1   16-06-13 00:00\n周日007   德国VS乌克兰\n主-1   16-06-13 03:00\n周日025   罗马尼亚VS阿尔巴尼亚\n主-1   16-06-20 03:00\n周日026   法国VS瑞士\n主-1   16-06-20 03:00\n周日101   磐田喜悦VS名古屋鲸八\n主-1   16-06-05 13:00\n周日102   大宫松鼠VS鹿岛鹿角\n主+1   16-06-05 13:00\n周日103   湘南海洋VS神户胜利船\n主+1   16-06-05 13:00\n周日104   福冈黄蜂VS新泻天鹅\n主+1   16-06-05 13:00\n周日105   鸟栖沙岩VS柏太阳神\n主+1   16-06-05 13:00\n周日106   仙台维加泰VS横滨水手\n主+1   16-06-05 13:00\n周日118   牙买加VS委内瑞拉\n主+1   16-06-06 05:00\n周日120   墨西哥VS乌拉圭\n主-1   16-06-06 08:00\n\n注:\n   1、让球仅适用于竞彩足球让球胜平负游戏\n   2、更多信息请查阅中国竞彩网\n   http://www.sporttery.cn。".to_string()
    }
    else if game_id==301 {
        "竞彩篮球对阵表\n场次    客队VS主队              比赛时间\n周日301   达拉斯小牛VS犹他爵士\n09-03-28 20:00\n周日302   费城76人VS波士顿凯尔特人\n09-03-28 20:00\n周日303   洛杉矶湖人VS菲尼克斯太阳\n09-03-28 20:00\n周日304   纽约尼克斯VS多伦多猛龙\n09-03-28 20:00\n周日305   洛杉矶快船VS萨克拉门托国王\n09-03-28 20:00\n周日306   亚特兰大老鹰VS布鲁克林篮网\n09-03-28 20:00\n周日307   丹佛掘金VS金州勇士\n09-03-28 20:00\n周日308   迈阿密热火VS奥兰多魔术\n09-03-28 20:00\n周日309   俄克拉荷马雷霆VS波特兰开拓者\n09-03-28 20:00\n周日310   夏洛特山猫VS华盛顿奇才\n09-03-28 20:00\n周日401   克利夫兰骑士VS明尼苏达森林狼\n09-03-25 10:00\n\n注:让分信息请查阅中国竞彩网\n   http://www.sporttery.cn。".to_string()
    } else {
        "".to_string()
    }
}

fn main() {
	let _ = elog::init();
    let stream = TcpStream::connect("127.0.0.1:8888").unwrap();
    //let stream = TcpStream::connect("123.56.177.229:8888").unwrap();
    let key = DigestUtil::md5(cfg_str!("protocol", "key"));
    let user_id = cfg_str!("protocol", "user_id");
    let mut pro = Protocol::new(stream, key.to_string());
	
	//thread::sleep(std::time::Duration::new(1, 0));
	let mut body = json!("{}");
	json_set!(&mut body; "msg"; "我要登陆");
	let rst = pro.send_body(user_id, "T01", &body);
	let rst = rst.or_else(|err|{
		error!("{}", err);
		Result::Err(-1)
	});
	let rst = rst.and_then(|_| {
		let rst = pro.rec_msg();
		rst
	});
	let rst = rst.and_then(|(head, body)|{
		info!("head:{}", head);
		info!("body:{}", body);
		Result::Ok(())
	});
	loop {
		let rst = pro.rec_msg();
		let rst = rst.and_then(|(head, body)|{
			info!("head:{}", head);
			info!("body:{}", body);
			let head = json!(&head);
			let body = json!(&body);
			let cmd = json_str!(&head; "cmd");
			if cmd == "T02" {
        	    //thread::sleep(std::time::Duration::new(60, 0));
				let ticket = json_path!(&body; "ticket");
				let ticket_id = json_i64!(ticket; "id");
				let game_id = json_i64!(ticket; "game_id");
				let play_type = json_i64!(ticket; "play_type");
				let bet_type = json_i64!(ticket; "bet_type");
				let number = json_str!(ticket; "number");
			
				let mut back_body = json!("{}");
				let mut back_ticket = json!("{}");
				json_set!(&mut back_ticket; "id"; ticket_id);
				json_set!(&mut back_ticket; "game_id"; game_id);
				json_set!(&mut back_ticket; "play_type"; play_type);
				json_set!(&mut back_ticket; "bet_type"; bet_type);
				json_set!(&mut back_ticket; "number"; number);
				json_set!(&mut back_ticket; "status"; 1);
				let stub = get_stub(game_id, play_type);
                info!("{}", stub);
				json_set!(&mut back_ticket; "stub"; stub);
				json_set!(&mut back_body; "ticket"; back_ticket);
				
				pro.send_body(user_id, cmd, &back_body);
				Result::Ok(())
				//Result::Err(-1)
            }
            else if cmd == "T03" {
				let ticket = json_path!(&body; "ticket");
				let ticket_id = json_i64!(ticket; "id");
				let bonus = json_i64!(ticket; "bonus");
				let seq = json_str!(ticket; "seq");
				let bonus_after_tax = json_i64!(ticket; "bonus_after_tax");
			
				let mut back_body = json!("{}");
				let mut back_ticket = json!("{}");
				json_set!(&mut back_ticket; "id"; ticket_id);
				json_set!(&mut back_ticket; "bonus"; bonus);
				json_set!(&mut back_ticket; "bonus_after_tax"; bonus_after_tax);
				json_set!(&mut back_ticket; "seq"; seq);
				json_set!(&mut back_ticket; "stub"; seq);
				json_set!(&mut back_body; "ticket"; back_ticket);
				
				//pro.send_body(user_id, cmd, &back_body);
				Result::Err(-1)
				//Result::Ok(())
			} else if cmd == "T10" {
				let mut back_body = json!("{}");
				pro.send_body(user_id, cmd, &back_body);
				Result::Ok(())
            } else if cmd == "T04" {
				let game_id = json_i64!(&body; "game_id");
                let cur_type = json_i64!(&body; "type");
                let mut back_body = json!("{}");
                json_set!(&mut back_body; "game_id"; game_id);
                json_set!(&mut back_body; "type"; cur_type);
                if (game_id == 201 || game_id == 301) && cur_type == 0 {
                    json_set!(&mut back_body; "stub"; get_term_stub(game_id));
                }
				pro.send_body(user_id, cmd, &back_body);
				Result::Ok(())
			} else if cmd == "T05" {
                let cur_type = json_i64!(&body; "type");

                let mut back_body = json!("{}");
                let stub = "缴款报表\n\n门店编号：09-090147\n数据截止时间：2016-05-15 14:56:02\n当前账户余额：48100.36元\n账户信用额度：0.00元\n临时信用额度：0.00元\n当前可用额度：48100.36元\n最新应缴款：0.00元\n\n\n账户缴款明细：\n   缴款金额         缴款时间\n----------------------------------------\n+10.00元       2016-04-20 18:13:51\n湖北省建行：201604200559761147231863599\n----------------------------------------\n\n佣金转入额度明细：\n  佣金转额度        发放时间\n----------------------------------------\n+0.80元        2016-04-22 00:03:56\n----------------------------------------\n报表查询时间:2016-05-15 14:56:37";
                json_set!(&mut back_body; "stub"; stub);
                json_set!(&mut back_body; "type"; cur_type);
				pro.send_body(user_id, cmd, &back_body);
				Result::Ok(())
			} else {
				Result::Ok(())
			}
		});
		if rst.is_err() {
			break;
		}
	}
}
