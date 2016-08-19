extern crate game;
use game::validate::ValidateFactory;
use game::base::Ticket;
use game::VF;
use game::DF;
use game::PF;

extern crate cons;
use cons::ErrCode;

extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::str::FromStr;

#[macro_use]
extern crate log;
extern crate elog;

#[macro_use]
extern crate easy_util;

extern crate util;
use util::NumberUtil;

use std::collections::BTreeMap;

fn main() {
	let _ = elog::init();
	
	info!("tests start...");

   		
    /*
	let ticket = Ticket::new(1_i64, 200, 10, 10, 3, 1, 600, "03,04,05,09,10|02,03;03,04,05,09,10|02,03;01,02,03,04,05|01,02");
	let rst = DF.draw(&ticket.to_json(), &draw_number, &gl_map);
	assert!(rst.is_ok());
	info!("{}", rst.unwrap());
	
	let ticket = Ticket::new(1_i64, 200, 60, 10, 3, 1, 900, "03,04,05,09,10|02,03;03,04,05,09,10|02,03;01,02,03,04,05|01,02");
	let rst = DF.draw(&ticket.to_json(), &draw_number, &gl_map);
	assert!(rst.is_ok());
	info!("{}", rst.unwrap());
	
	let ticket = Ticket::new(1_i64, 200, 10, 20, 3, 1, 600, "03,04,05,09,10|02,03,04");
	let rst = DF.draw(&ticket.to_json(), &draw_number, &gl_map);
	assert!(rst.is_ok());
	info!("{}", rst.unwrap());
	
	let ticket = Ticket::new(1_i64, 200, 10, 30, 3, 1, 600, "03,04,05,09,10|02$03,04,05");
	let rst = DF.draw(&ticket.to_json(), &draw_number, &gl_map);
	assert!(rst.is_ok());
	info!("{}", rst.unwrap());
            
    let back_number = PF.get_print_number(201, 1, 2, "20121115001:1;20121115002:3,1|2*1", "中国体育彩票演示票\n演示票竞彩足球胜平负   单场固定\n200243-945712-189922-67 FD362CCE 00546858\n─────────────────────\n周日001\n主队:皇家马德里 Vs 客队:巴塞罗那\n平@12.0元\n周日002\n主队:皇家马德里 Vs 客队:巴塞罗那\n胜@12.0元+平@12.0元\n").unwrap();
    info!("{}", back_number);
	
    let back_number = PF.get_print_number(201, 2, 2, "20121115001:1;20121115002:3,1|2*1", "中国体育彩票演示票\n演示票竞彩足球胜平负   单场固定\n200243-945712-189922-67 FD362CCE 00546858\n─────────────────────\n周日001\n主队:皇家马德里 Vs 客队:巴塞罗那\n平@12.0元\n周日002\n主队:皇家马德里 Vs 客队:巴塞罗那\n胜@12.0元+平@12.0元\n").unwrap();
    info!("{}", back_number);
    
    let back_number = PF.get_print_number(201, 3, 2, "20121115001:1;20121115002:1,7|2*1", "中国体育彩票演示票\n演示票竞彩足球胜平负   单场固定\n200243-945712-189922-67 FD362CCE 00546858\n─────────────────────\n周日001\n主队:皇家马德里 Vs 客队:巴塞罗那\n(1)@12.9元\n周日002\n主队:皇家马德里 Vs 客队:巴塞罗那\n(1)@12.0元+(7+)@2.0元\n").unwrap();
    info!("{}", back_number);

    let back_number = PF.get_print_number(201, 4, 2, "20121115001:09;20121115002:12,23|2*1", "中国体育彩票演示票\n演示票竞彩足球胜平负   单场固定\n200243-945712-189922-67 FD362CCE 00546858\n─────────────────────\n周日001\n主队:皇家马德里 Vs 客队:巴塞罗那\n(负其它)@12.9元\n周日002\n主队:皇家马德里 Vs 客队:巴塞罗那\n(1:2)@12.0元+(2:3)@2.0元\n").unwrap();
    info!("{}", back_number);

    let back_number = PF.get_print_number(201, 5, 2, "20121115001:33;20121115002:11,33|2*1", "中国体育彩票演示票\n演示票竞彩足球胜平负   单场固定\n200243-945712-189922-67 FD362CCE 00546858\n─────────────────────\n周日001\n主队:皇家马德里 Vs 客队:巴塞罗那\n胜胜@12.9元\n周日002\n主队:皇家马德里 Vs 客队:巴塞罗那\n平平@12.0元+胜胜@2.0元\n").unwrap();
    info!("{}", back_number);

    let back_number = PF.get_print_number(201, 10, 2, "20121115001:05:33;20121115002:01:1,3|2*1", "中国体育彩票演示票\n演示票竞彩足球胜平负   单场固定\n200243-945712-189922-67 FD362CCE 00546858\n─────────────────────\n周日001\n主队:皇家马德里 Vs 客队:巴塞罗那\n胜胜@12.9元\n周日002\n主队:皇家马德里 Vs 客队:巴塞罗那\n平@12.0元+胜@2.0元\n").unwrap();
    info!("{}", back_number);

	info!("all tests passed.");
    */
}

fn get_gl_map() -> BTreeMap<i64, Json> {
	let mut gl_map = BTreeMap::<i64, Json>::new();
	{
		let mut lev = json!(r#"
			{"data":[{"bonus":1000,"bonus_after_tax":800,"create_time":1460639394,"descrip":"一等奖","game_id":200,"id":1,"lev":1,"term_code":2015001,"version":0},{"bonus":800,"bonus_after_tax":600,"create_time":1460639394,"descrip":"二等奖","game_id":200,"id":2,"lev":2,"term_code":2015001,"version":0},{"bonus":400,"bonus_after_tax":200,"create_time":1460639394,"descrip":"三等奖","game_id":200,"id":3,"lev":3,"term_code":2015001,"version":0},{"bonus":20000,"bonus_after_tax":20000,"create_time":1460639394,"descrip":"四等奖","game_id":200,"id":4,"lev":4,"term_code":2015001,"version":0},{"bonus":1000,"bonus_after_tax":1000,"create_time":1460639394,"descrip":"五等奖","game_id":200,"id":5,"lev":5,"term_code":2015001,"version":0},{"bonus":500,"bonus_after_tax":500,"create_time":1460639394,"descrip":"六等奖","game_id":200,"id":6,"lev":6,"term_code":2015001,"version":0},{"bonus":1000,"bonus_after_tax":800,"create_time":1460639394,"descrip":"一等奖追加","game_id":200,"id":7,"lev":7,"term_code":2015001,"version":0},{"bonus":800,"bonus_after_tax":600,"create_time":1460639394,"descrip":"二等奖追加","game_id":200,"id":8,"lev":8,"term_code":2015001,"version":0},{"bonus":400,"bonus_after_tax":200,"create_time":1460639394,"descrip":"三等奖追加","game_id":200,"id":9,"lev":9,"term_code":2015001,"version":0},{"bonus":10000,"bonus_after_tax":10000,"create_time":1460639394,"descrip":"四等奖追加","game_id":200,"id":10,"lev":10,"term_code":2015001,"version":0},{"bonus":500,"bonus_after_tax":500,"create_time":1460639394,"descrip":"五等奖追加","game_id":200,"id":11,"lev":11,"term_code":2015001,"version":0}]}
		"#);
		let mut lev_obj = lev.as_object_mut().unwrap();
		let data = lev_obj.remove("data").unwrap();
		let data_array = data.as_array().unwrap();
		for set in data_array {
			let lev = json_i64!(set; "lev");
			gl_map.insert(lev as i64, set.clone());
		}
	}
	gl_map
}
