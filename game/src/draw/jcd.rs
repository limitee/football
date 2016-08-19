extern crate util;
use util::NumberUtil;
use util::MathUtil;

extern crate cons;
use cons::ErrCode;

extern crate regex;
use self::regex::Regex;

extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::str::FromStr;

use super::super::base::DrawResult;
use super::super::base::Draw;
use super::super::base::Print;
use super::super::base::Game;
use super::super::base::PlayType;
use super::super::base::BetType;

use std::collections::BTreeMap;

type LvDetail = (i32, i32, i64, i64);

struct Helper;

impl Helper {

    ///获得号码的选项map，key为选项，value为赔率
    fn get_op_map(number:&str) -> BTreeMap<&str, f64> {
        let number_info_array:Vec<&str> = number.split('|').collect();
        let number_first_array:Vec<&str> = number_info_array[0].split(':').collect();
        let op_array:Vec<&str> = number_first_array[1].split(',').collect();
        let mut map = BTreeMap::new();
        for rate_op in op_array {
            let rate_op_array:Vec<&str> = rate_op.split('@').collect();
            map.insert(rate_op_array[0], f64::from_str(rate_op_array[1]).unwrap());
        }
        map
    }

    fn get_print_number(back_map:&BTreeMap<&str, &str>, number:&str) -> String {
        let number_info_array:Vec<&str> = number.split('|').collect();
        let number_first_array:Vec<&str> = number_info_array[0].split(':').collect();
        let op_array:Vec<&str> = number_first_array[1].split(',').collect();
        let mut rate_info = String::new();
        let mut count = 0;
        for rate_op in op_array {
            if count > 0 {
                rate_info.push_str(",");
            }
            let rate = format!("{}@{}", rate_op, back_map.get(rate_op).unwrap()); 
            rate_info.push_str(&rate);
            count += 1;
        }
        let print_number = format!("{}:{}|{}", number_first_array[0], rate_info, number_info_array[1]);
        print_number
    }
	
	pub fn get_rst(&self, detail:BTreeMap<i32, LvDetail>, bonus:i64, bonus_after_tax:i64) -> Json {
		let mut json = json!("{}");
		json_set!(&mut json; "bonus"; bonus);
		json_set!(&mut json; "bonus_after_tax"; bonus_after_tax);
		let mut detail_list = Vec::new();
		for (key, value) in detail.iter() {
		    let mut json = json!("{}");
		    json_set!(&mut json; "lev"; value.0);
		    json_set!(&mut json; "count"; value.1);
		    json_set!(&mut json; "bonus"; value.2);
		    json_set!(&mut json; "bonus_after_tax"; value.3);
		    detail_list.push(json);
		}
		json_set!(&mut json; "detail"; detail_list);
		json
	}
	
	///添加中奖明细
	pub fn append_rst(&self, detail:&mut BTreeMap<i32, LvDetail>, bonus:i64, lev:i32, count:i32) -> (i64, i64) {
		if count <= 0 {
			return (0_i64, 0_i64);
		}
		let bonus = bonus;
		let bonus_after_tax = bonus;
		
		if detail.contains_key(&lev) {
			let mut cur_lev_detail = detail.get_mut(&lev).unwrap();
			cur_lev_detail.1 += count;
			cur_lev_detail.2 += bonus;
			cur_lev_detail.3 += bonus_after_tax;	
		} else {
			detail.insert(lev, (lev, count, bonus, bonus_after_tax));
		}
		return (bonus, bonus_after_tax);
	}
	
}



pub struct DrawJcd0101;

impl Draw for DrawJcd0101 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, gl:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {
		let (mut bonus, mut bonus_after_tax) = (0_i64, 0_i64);
		let mut detail = BTreeMap::<i32, LvDetail>::new();
        let number = json_str!(ticket; "print_number");	
        let op_map = Helper::get_op_map(number);
        //场次取消
        if draw_number.len() == 0 {
            let rate = op_map.len() as f64;
            let (b, bat) = Helper.append_rst(&mut detail, (rate*200_f64) as i64, 11, 1);  
            bonus += b;
          	bonus_after_tax += bat;
		    return Result::Ok(Helper.get_rst(detail, bonus, bonus_after_tax));
        }
        let key;
        if draw_number[0] > draw_number[1] {
            key = "3"; 
        } else if draw_number[0] == draw_number[1] {
            key = "1";
        } else {
            key = "0";
        }
        let match_op = op_map.get(key);
        if let Some(rate) = match_op {
            let cur_bonus = NumberUtil::get_jc_rate(rate*200_f64);
            let (b, bat) = Helper.append_rst(&mut detail, cur_bonus, 11, 1);  
            bonus += b;
          	bonus_after_tax += bat;
        }
		Result::Ok(Helper.get_rst(detail, bonus, bonus_after_tax))
	}
}

impl Print for DrawJcd0101 {
	
	fn get_print_number(&self, number:&str, stub:&str) -> String {
        let stub_info_array:Vec<&str> = stub.split('\n').collect();
		let rate = stub_info_array[6];
        info!("the rate is {}.", rate);
        let rate_array:Vec<&str> = rate.split('+').collect();
        let mut back_map = BTreeMap::<&str, &str>::new(); 
        for rate_op in rate_array {
            let rate_op_array:Vec<&str> = rate_op.split('@').collect();
            let len = rate_op_array[1].len();
            //去掉元字符
            let (rate_detail, trim) = rate_op_array[1].split_at(len - 3);
            info!("{}---{}", rate_op_array[0], rate_detail);
            let key;
            match rate_op_array[0] {
                "胜" => {
                    key = "3";
                },
                "平" => {
                    key = "1";
                },
                "负" => {
                    key = "0";
                },
                _ => {
                    key = "-1";
                },
            }
            back_map.insert(key, rate_detail);
        }
        Helper::get_print_number(&back_map, number)
	}
}

pub struct DrawJcd0201;

impl Draw for DrawJcd0201 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, gl:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {
		let (mut bonus, mut bonus_after_tax) = (0_i64, 0_i64);
		let mut detail = BTreeMap::<i32, LvDetail>::new();
        let number = json_str!(ticket; "print_number");	
        let op_map = Helper::get_op_map(number);
        //场次取消
        if draw_number.len() == 0 {
            let rate = op_map.len() as f64;
            let (b, bat) = Helper.append_rst(&mut detail, (rate*200_f64) as i64, 11, 1);  
            bonus += b;
          	bonus_after_tax += bat;
		    return Result::Ok(Helper.get_rst(detail, bonus, bonus_after_tax));
        }
        let key;
        if draw_number[0] + draw_number[2] > draw_number[1] {
            key = "3"; 
        } else if draw_number[0] + draw_number[2] == draw_number[1] {
            key = "1";
        } else {
            key = "0";
        }
        let match_op = op_map.get(key);
        if let Some(rate) = match_op {
            let cur_bonus = NumberUtil::get_jc_rate(rate*200_f64);
            //let cur_bonus = (rate*200_f64).round() as i64;
            let (b, bat) = Helper.append_rst(&mut detail, cur_bonus, 11, 1);  
            bonus += b;
          	bonus_after_tax += bat;
        }
		Result::Ok(Helper.get_rst(detail, bonus, bonus_after_tax))
	}
}

pub struct DrawJcd0301;

impl Draw for DrawJcd0301 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, gl:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {
		let (mut bonus, mut bonus_after_tax) = (0_i64, 0_i64);
		let mut detail = BTreeMap::<i32, LvDetail>::new();
        let number = json_str!(ticket; "print_number");	
        let op_map = Helper::get_op_map(number);
        //场次取消
        if draw_number.len() == 0 {
            let rate = op_map.len() as f64;
            let (b, bat) = Helper.append_rst(&mut detail, (rate*200_f64) as i64, 11, 1);  
            bonus += b;
          	bonus_after_tax += bat;
		    return Result::Ok(Helper.get_rst(detail, bonus, bonus_after_tax));
        }

        let mut key = (draw_number[0] + draw_number[1])/10;
        if key > 7 {
            key = 7;
        }
        let key = format!("{}", key);
        let match_op = op_map.get(key.as_str());
        if let Some(rate) = match_op {
            let cur_bonus = NumberUtil::get_jc_rate(rate*200_f64);
            //let cur_bonus = (rate*200_f64).round() as i64;
            let (b, bat) = Helper.append_rst(&mut detail, cur_bonus, 11, 1);  
            bonus += b;
          	bonus_after_tax += bat;
        }
		Result::Ok(Helper.get_rst(detail, bonus, bonus_after_tax))
	}
}

impl Print for DrawJcd0301 {
	
	fn get_print_number(&self, number:&str, stub:&str) -> String {
        let stub_info_array:Vec<&str> = stub.split('\n').collect();
		let rate = stub_info_array[6];
        let rate = rate.replace("(7+)", "(7)");
        info!("the rate is {}.", rate);
        let rate_array:Vec<&str> = rate.split('+').collect();
        let mut back_map = BTreeMap::<&str, &str>::new(); 
        for rate_op in rate_array {
            let rate_op_array:Vec<&str> = rate_op.split('@').collect();
            let len = rate_op_array[1].len();
            //去掉元字符
            let (rate_detail, trim) = rate_op_array[1].split_at(len - 3);
            info!("{}---{}", rate_op_array[0], rate_detail);
            let key;
            match rate_op_array[0] {
                "(0)" => {
                    key = "0";
                },
                "(1)" => {
                    key = "1";
                },
                "(2)" => {
                    key = "2";
                },
                "(3)" => {
                    key = "3";
                },
                "(4)" => {
                    key = "4";
                },
                "(5)" => {
                    key = "5";
                },
                "(6)" => {
                    key = "6";
                },
                "(7)" => {
                    key = "7";
                },
                _ => {
                    key = "-1";
                },
            }
            back_map.insert(key, rate_detail);
        }
        Helper::get_print_number(&back_map, number)
	}
}

pub struct DrawJcd0401;

impl Draw for DrawJcd0401 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, gl:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {
		let (mut bonus, mut bonus_after_tax) = (0_i64, 0_i64);
		let mut detail = BTreeMap::<i32, LvDetail>::new();
        let number = json_str!(ticket; "print_number");	
        let op_map = Helper::get_op_map(number);
        //场次取消
        if draw_number.len() == 0 {
            let rate = op_map.len() as f64;
            let (b, bat) = Helper.append_rst(&mut detail, (rate*200_f64) as i64, 11, 1);  
            bonus += b;
          	bonus_after_tax += bat;
		    return Result::Ok(Helper.get_rst(detail, bonus, bonus_after_tax));
        }

        let key;
        if draw_number[0] > draw_number[1] {
            if draw_number[0] > 50 || draw_number[1] > 20 {
                key = "90".to_string();
            } else {
                key = format!("{}{}", draw_number[0]/10, draw_number[1]/10);
            }
        } else if draw_number[0] < draw_number[1] {
            if draw_number[0] > 20 || draw_number[1] > 50 {
                key = "09".to_string();
            } else {
                key = format!("{}{}", draw_number[0]/10, draw_number[1]/10);
            }
        } else {
            if draw_number[0] > 30 {
                key = "99".to_string();
            } else {
                key = format!("{}{}", draw_number[0]/10, draw_number[1]/10);
            }
        }
        let match_op = op_map.get(key.as_str());
        if let Some(rate) = match_op {
            let cur_bonus = NumberUtil::get_jc_rate(rate*200_f64);
            //let cur_bonus = (rate*200_f64).round() as i64;
            let (b, bat) = Helper.append_rst(&mut detail, cur_bonus, 11, 1);  
            bonus += b;
          	bonus_after_tax += bat;
        }
		Result::Ok(Helper.get_rst(detail, bonus, bonus_after_tax))
	}
}

impl Print for DrawJcd0401 {
	
	fn get_print_number(&self, number:&str, stub:&str) -> String {
        let stub_info_array:Vec<&str> = stub.split('\n').collect();
		let rate = stub_info_array[6];
        info!("the rate is {}.", rate);
        let rate_array:Vec<&str> = rate.split('+').collect();
        let mut back_map = BTreeMap::<&str, &str>::new(); 
        for rate_op in rate_array {
            let rate_op_array:Vec<&str> = rate_op.split('@').collect();
            let len = rate_op_array[1].len();
            //去掉元字符
            let (rate_detail, trim) = rate_op_array[1].split_at(len - 3);
            info!("{}---{}", rate_op_array[0], rate_detail);
            let key;
            match rate_op_array[0] {
                "(1:0)" => {
                    key = "10";
                },
                "(2:0)" => {
                    key = "20";
                },
                "(2:1)" => {
                    key = "21";
                },
                "(3:0)" => {
                    key = "30";
                },
                "(3:1)" => {
                    key = "31";
                },
                "(3:2)" => {
                    key = "32";
                },
                "(4:0)" => {
                    key = "40";
                },
                "(4:1)" => {
                    key = "41";
                },
                "(4:2)" => {
                    key = "42";
                },
                "(5:0)" => {
                    key = "50";
                },
                "(5:1)" => {
                    key = "51";
                },
                "(5:2)" => {
                    key = "52";
                },
                "(胜其它)" => {
                    key = "90";
                },
                "(0:0)" => {
                    key = "00";
                },
                "(1:1)" => {
                    key = "11";
                },
                "(2:2)" => {
                    key = "22";
                },
                "(3:3)" => {
                    key = "33";
                },
                "(平其它)" => {
                    key = "99";
                },
                "(0:1)" => {
                    key = "01";
                },
                "(0:2)" => {
                    key = "02";
                },
                "(1:2)" => {
                    key = "12";
                },
                "(0:3)" => {
                    key = "03";
                },
                "(1:3)" => {
                    key = "13";
                },
                "(2:3)" => {
                    key = "23";
                },
                "(0:4)" => {
                    key = "04";
                },
                "(1:4)" => {
                    key = "14";
                },
                "(2:4)" => {
                    key = "24";
                },
                "(0:5)" => {
                    key = "05";
                },
                "(1:5)" => {
                    key = "15";
                },
                "(2:5)" => {
                    key = "25";
                },
                "(负其它)" => {
                    key = "09";
                },
                _ => {
                    key = "-1";
                },
            }
            back_map.insert(key, rate_detail);
        }
        Helper::get_print_number(&back_map, number)
	}
}

pub struct DrawJcd0501;

impl Draw for DrawJcd0501 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, gl:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {
		let (mut bonus, mut bonus_after_tax) = (0_i64, 0_i64);
		let mut detail = BTreeMap::<i32, LvDetail>::new();
        let number = json_str!(ticket; "print_number");	
        let op_map = Helper::get_op_map(number);

        //场次取消
        if draw_number.len() == 0 {
            let rate = op_map.len() as f64;
            let (b, bat) = Helper.append_rst(&mut detail, (rate*200_f64) as i64, 11, 1);  
            bonus += b;
          	bonus_after_tax += bat;
		    return Result::Ok(Helper.get_rst(detail, bonus, bonus_after_tax));
        }

        //半场
        let first; 
        if draw_number[3] > draw_number[4] {
            first = "3"; 
        } else if draw_number[3] == draw_number[4] {
            first = "1";
        } else {
            first = "0";
        }
        //全场
        let second; 
        if draw_number[0] > draw_number[1] {
            second = "3"; 
        } else if draw_number[0] == draw_number[1] {
            second = "1";
        } else {
            second = "0";
        }
        let key = format!("{}{}", first , second);
        let match_op = op_map.get(key.as_str());
        if let Some(rate) = match_op {
            let cur_bonus = NumberUtil::get_jc_rate(rate*200_f64);
            //let cur_bonus = (rate*200_f64).round() as i64;
            let (b, bat) = Helper.append_rst(&mut detail, cur_bonus, 11, 1);  
            bonus += b;
          	bonus_after_tax += bat;
        }
		Result::Ok(Helper.get_rst(detail, bonus, bonus_after_tax))
	}
}

impl Print for DrawJcd0501 {
	
	fn get_print_number(&self, number:&str, stub:&str) -> String {
        let stub_info_array:Vec<&str> = stub.split('\n').collect();
		let rate = stub_info_array[6];
        info!("the rate is {}.", rate);
        let rate_array:Vec<&str> = rate.split('+').collect();
        let mut back_map = BTreeMap::<&str, &str>::new(); 
        for rate_op in rate_array {
            let rate_op_array:Vec<&str> = rate_op.split('@').collect();
            let len = rate_op_array[1].len();
            //去掉元字符
            let (rate_detail, trim) = rate_op_array[1].split_at(len - 3);
            info!("{}---{}", rate_op_array[0], rate_detail);
            let key;
            match rate_op_array[0] {
                "胜胜" => {
                    key = "33";
                },
                "胜平" => {
                    key = "31";
                },
                "胜负" => {
                    key = "30";
                },
                "平胜" => {
                    key = "13";
                },
                "平平" => {
                    key = "11";
                },
                "平负" => {
                    key = "10";
                },
                "负胜" => {
                    key = "03";
                },
                "负平" => {
                    key = "01";
                },
                "负负" => {
                    key = "00";
                },
                _ => {
                    key = "-1";
                },
            }
            back_map.insert(key, rate_detail);
        }
        Helper::get_print_number(&back_map, number)
	}
}

