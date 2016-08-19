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
use std::collections::HashMap;

type LvDetail = (i32, i32, i64, i64);

struct Helper {
    bunch_map: HashMap<String, String>,
}

impl Helper {

    pub fn new() -> Helper {
        let mut bunch_map = HashMap::new();
        bunch_map.insert("11".to_string(), "10000000".to_string());
        bunch_map.insert("21".to_string(), "01000000".to_string());
        bunch_map.insert("31".to_string(), "00100000".to_string());
        bunch_map.insert("41".to_string(), "00010000".to_string());
        bunch_map.insert("51".to_string(), "00001000".to_string());
        bunch_map.insert("61".to_string(), "00000100".to_string());
        bunch_map.insert("71".to_string(), "00000010".to_string());
        bunch_map.insert("81".to_string(), "00000001".to_string());
        bunch_map.insert("23".to_string(), "11000000".to_string());
        bunch_map.insert("36".to_string(), "11000000".to_string());
        bunch_map.insert("37".to_string(), "11100000".to_string());
        bunch_map.insert("410".to_string(), "11000000".to_string());
        bunch_map.insert("414".to_string(), "11100000".to_string());
        bunch_map.insert("415".to_string(), "11110000".to_string());
        bunch_map.insert("515".to_string(), "11000000".to_string());
        bunch_map.insert("525".to_string(), "11100000".to_string());
        bunch_map.insert("530".to_string(), "11110000".to_string());
        bunch_map.insert("531".to_string(), "11111000".to_string());
        bunch_map.insert("621".to_string(), "11000000".to_string());
        bunch_map.insert("641".to_string(), "11100000".to_string());
        bunch_map.insert("656".to_string(), "11110000".to_string());
        bunch_map.insert("662".to_string(), "11111000".to_string());
        bunch_map.insert("663".to_string(), "11111100".to_string());
        bunch_map.insert("7127".to_string(), "11111110".to_string());
        bunch_map.insert("8255".to_string(), "11111111".to_string());
        bunch_map.insert("33".to_string(), "01000000".to_string());
        bunch_map.insert("34".to_string(), "01100000".to_string());
        bunch_map.insert("46".to_string(), "01000000".to_string());
        bunch_map.insert("411".to_string(), "01110000".to_string());
        bunch_map.insert("510".to_string(), "01000000".to_string());
        bunch_map.insert("520".to_string(), "01100000".to_string());
        bunch_map.insert("526".to_string(), "01111000".to_string());
        bunch_map.insert("615".to_string(), "01000000".to_string());
        bunch_map.insert("635".to_string(), "01100000".to_string());
        bunch_map.insert("650".to_string(), "01110000".to_string());
        bunch_map.insert("657".to_string(), "01111100".to_string());
        bunch_map.insert("7120".to_string(), "01111110".to_string());
        bunch_map.insert("8247".to_string(), "01111111".to_string());
        bunch_map.insert("44".to_string(), "00100000".to_string());
        bunch_map.insert("45".to_string(), "00110000".to_string());
        bunch_map.insert("516".to_string(), "00111000".to_string());
        bunch_map.insert("620".to_string(), "00100000".to_string());
        bunch_map.insert("642".to_string(), "00111100".to_string());
        bunch_map.insert("55".to_string(), "00010000".to_string());
        bunch_map.insert("56".to_string(), "00011000".to_string());
        bunch_map.insert("622".to_string(), "00011100".to_string());
        bunch_map.insert("735".to_string(), "00010000".to_string());
        bunch_map.insert("870".to_string(), "00010000".to_string());
        bunch_map.insert("66".to_string(), "00001000".to_string());
        bunch_map.insert("67".to_string(), "00001100".to_string());
        bunch_map.insert("721".to_string(), "00001000".to_string());
        bunch_map.insert("856".to_string(), "00001000".to_string());
        bunch_map.insert("77".to_string(), "00000100".to_string());
        bunch_map.insert("78".to_string(), "00000110".to_string());
        bunch_map.insert("828".to_string(), "00000100".to_string());
        bunch_map.insert("88".to_string(), "00000010".to_string());
        bunch_map.insert("89".to_string(), "00000011".to_string());
		
		Helper {
            bunch_map: bunch_map,
		}
	}

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
		let bonus_after_tax;
        if bonus >= 1000000_i64 {
            bonus_after_tax = bonus*4/5;
        } else {
		    bonus_after_tax = bonus;
        }
		
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

fn get_rate(number:&str, play_type:&str, stub:&str) -> String {
    let stub_info_array:Vec<&str> = stub.split('\n').collect();
    let number_info_array:Vec<&str> = number.split('|').collect();
    let number_first_array:Vec<&str> = number_info_array[0].split(';').collect();
    let mut start_index = 6;
    let step = 3;
    let mut index = 0;
    let mut back_rate = String::new();
    for match_number in number_first_array {
        let match_pos_array:Vec<&str> = match_number.split(':').collect();
        let rate = stub_info_array[start_index]; 
        start_index += step;

        let match_code;
        let cur_play_type;
        let options;
        let mut set_play_type = false;
        if match_pos_array.len() == 3 {
            match_code = match_pos_array[0];
            cur_play_type = match_pos_array[1];
            options = match_pos_array[2];
            set_play_type = true;
        } else {
            match_code = match_pos_array[0];
            cur_play_type = play_type;
            options = match_pos_array[1];
        }
        let match_rate = get_rate_by_play_type(match_code, cur_play_type, options, rate, set_play_type);
        if index > 0 {
            back_rate.push_str(";");
        }
        back_rate.push_str(match_rate.as_str());
        index += 1;
    }
    back_rate.push_str("|");
    back_rate.push_str(number_info_array[1]);
    back_rate
}

fn get_rate_by_play_type(match_code:&str, play_type:&str, options:&str, rate:&str, set_play_type:bool) -> String {
    match play_type {
        "01" => {
            get_rate_0102(match_code, options, rate, set_play_type) 
        },
        "02" => {
            get_rate_0202(match_code, options, rate, set_play_type) 
        },
        "03" => {
            get_rate_0302(match_code, options, rate, set_play_type) 
        },
        "04" => {
            get_rate_0402(match_code, options, rate, set_play_type) 
        },
        "05" => {
            get_rate_0502(match_code, options, rate, set_play_type) 
        },
        _  => {
            String::new()
        },
    }
}

///根据赔率详情获得赔率字符串
fn get_rate_by_map_and_op(match_code:&str, rate_map:&BTreeMap<&str, &str>, options:&str, play_type:&str, set_play_type:bool) -> String {
    let op_array:Vec<&str> = options.split(',').collect();
    let mut rate_info = String::new();
    let mut count = 0;
    for rate_op in op_array {
        if count > 0 {
            rate_info.push_str(",");
        }
        let rate = format!("{}@{}", rate_op, rate_map.get(rate_op).unwrap()); 
        rate_info.push_str(&rate);
        count += 1;
    }
    let print_number;
    if set_play_type {
        print_number = format!("{}:{}:{}", match_code, play_type, rate_info); 
    } else {
        print_number = format!("{}:{}", match_code, rate_info); 
    }
    print_number
}

///胜平负玩法的赔率
fn get_rate_0102(match_code:&str, options:&str, rate:&str, set_play_type:bool) -> String {
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
    get_rate_by_map_and_op(match_code, &back_map, options, "01", set_play_type) 
}

///让球胜平负玩法的赔率
fn get_rate_0202(match_code:&str, options:&str, rate:&str, set_play_type:bool) -> String {
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
    get_rate_by_map_and_op(match_code, &back_map, options, "02", set_play_type) 
}

///总进球玩法的赔率
fn get_rate_0302(match_code:&str, options:&str, rate:&str, set_play_type:bool) -> String {
    info!("the rate is {}.", rate);
    let rate = rate.replace("(7+)", "(7)");
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
    get_rate_by_map_and_op(match_code, &back_map, options, "03", set_play_type) 
}

///比分玩法的赔率
fn get_rate_0402(match_code:&str, options:&str, rate:&str, set_play_type:bool) -> String {
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
    get_rate_by_map_and_op(match_code, &back_map, options, "04", set_play_type) 
}

///半全场玩法的赔率
fn get_rate_0502(match_code:&str, options:&str, rate:&str, set_play_type:bool) -> String {
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
    get_rate_by_map_and_op(match_code, &back_map, options, "05", set_play_type) 
}


///获得一个场次的所有选择项
fn get_select_map(match_info:&str, play_type:&str) -> (i64, String, BTreeMap<String, f64>) {
    let match_info_array:Vec<&str> = match_info.split(":").collect();
    let term_code = i64::from_str(match_info_array[0]).unwrap();
    let options;
    let cur_play_type;
    if play_type == "10" {
        cur_play_type = match_info_array[1];
        options = match_info_array[2];
    } else {
        cur_play_type = play_type;
        options = match_info_array[1];
    }
    let mut op_map = BTreeMap::new();
    let option_array:Vec<&str> = options.split(",").collect();
    for option in option_array {
        let op_and_rate:Vec<&str> = option.split("@").collect();
        let rate = f64::from_str(op_and_rate[1]).unwrap();
        op_map.insert(op_and_rate[0].to_string(), rate);
    }
    (term_code, cur_play_type.to_string(), op_map)
}

///获得所有的期次数组，选项按期次，玩法的赔率map，m串n的玩法
fn get_select_vec(print_number:&str, play_type:&str) -> (Vec<(i64, String)>, BTreeMap<i64, BTreeMap<String, f64>>, i64, i64) {
    let print_number_array:Vec<&str> = print_number.split("|").collect();
    let mn_array:Vec<&str> = print_number_array[1].split('*').collect();
    let m = i64::from_str(mn_array[0]).unwrap();
    let n = i64::from_str(mn_array[1]).unwrap();
    
    let mut map = BTreeMap::new();
    let mut vec = Vec::new();
    let match_info_array = print_number_array[0].split(";");
    for match_info in match_info_array {
        let (term_code, play_type, rate_map) = get_select_map(match_info, play_type); 
        map.insert(term_code, rate_map);
        vec.push((term_code, play_type));
    }
    (vec, map, m, n)
}

///获得本场次的赔率，主要区分正常场次和取消场次
fn check_get_rate(match_code:i64, play_type:&str, map:&BTreeMap<i64, BTreeMap<String, f64>>, gl:&BTreeMap<i64, Json>) -> Option<f64> {
    let op_map = map.get(&match_code).unwrap();
    let hit_ops = gl.get(&match_code).unwrap();
    let hit_op = json_str!(hit_ops; play_type);

    //如果场次取消
    if hit_op == "*" {
        Some(op_map.len() as f64) 
    } else {
        let op = op_map.get(hit_op);
        match op {
            Some(rate) => {
                Some(*rate)
            },
            None => {
                None
            },
        }
    }
}

///竞彩算奖算法
fn check(play_type:&str, ticket:&Json, gl:&BTreeMap<i64, Json>) -> DrawResult {

    let mut json = json!("{}");
	let (mut bonus, mut bonus_after_tax) = (0_i64, 0_i64);
	let mut detail = BTreeMap::<i32, LvDetail>::new();

    let print_number = json_str!(ticket; "print_number");
    let (vec, map, m, n) = get_select_vec(print_number, play_type);
    let bunch_key = format!("{}{}", m, n);
    let bunch_flag = HP.bunch_map.get(bunch_key.as_str()).unwrap();
    let mut index = 0;
    for token in bunch_flag.chars() {
        if token == '1' {
            let dc = MathUtil::get_detailc(m, index + 1); 
            info!("{:?}", dc.data);
            for set in dc.data {
                let mut cur_bonus = 200_f64;
                let mut hit = true;
                for vec_index in set {
                    let (term_code, ref play_type) = vec[vec_index as usize];
                    let rate_op = check_get_rate(term_code, play_type, &map, gl);
                    if let Some(rate) = rate_op {
                        cur_bonus *= rate;
                    } else {
                        cur_bonus = 0_f64;
                        hit = false;
                        break;
                    }
                }
                if hit {
                    let count_bonus = NumberUtil::get_jc_rate(cur_bonus);
                    let lev = i32::from_str(bunch_key.as_str()).unwrap(); 
                    let (b, bat) = HP.append_rst(&mut detail, count_bonus, lev, 1);  
                    bonus += b;
          	        bonus_after_tax += bat;
                }
            }
        } 
        index += 1;
    }
	Result::Ok(HP.get_rst(detail, bonus, bonus_after_tax))
}

pub struct DrawJcc0102;

impl Draw for DrawJcc0102 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, gl:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {

        check("01", ticket, gl)
	}
}

impl Print for DrawJcc0102 {
	fn get_print_number(&self, number:&str, stub:&str) -> String {
        get_rate(number, "01", stub)
	}
}

pub struct DrawJcc0202;

impl Draw for DrawJcc0202 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, gl:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {
        check("02", ticket, gl)
	}
}

impl Print for DrawJcc0202 {
	fn get_print_number(&self, number:&str, stub:&str) -> String {
        get_rate(number, "02", stub)
	}
}

pub struct DrawJcc0302;

impl Draw for DrawJcc0302 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, gl:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {
        check("03", ticket, gl)
	}
}

impl Print for DrawJcc0302 {
	fn get_print_number(&self, number:&str, stub:&str) -> String {
        get_rate(number, "03", stub)
	}
}

pub struct DrawJcc0402;

impl Draw for DrawJcc0402 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, gl:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {
        check("04", ticket, gl)
	}
}

impl Print for DrawJcc0402 {
	fn get_print_number(&self, number:&str, stub:&str) -> String {
        get_rate(number, "04", stub)
	}
}

pub struct DrawJcc0502;

impl Draw for DrawJcc0502 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, gl:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {
        check("05", ticket, gl)
	}
}

impl Print for DrawJcc0502 {
	fn get_print_number(&self, number:&str, stub:&str) -> String {
        get_rate(number, "05", stub)
	}
}

pub struct DrawJcc1002;

impl Draw for DrawJcc1002 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, gl:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {
         check("10", ticket, gl)
	}
}

impl Print for DrawJcc1002 {
	fn get_print_number(&self, number:&str, stub:&str) -> String {
        get_rate(number, "10", stub)
	}
}

lazy_static! {
	static ref HP:Helper = Helper::new();
}
