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
        let time_info = stub_info_array[start_index - 2];
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
        let match_rate = get_rate_by_play_type(match_code, cur_play_type, options, time_info, rate, set_play_type);
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

fn get_rate_by_play_type(match_code:&str, play_type:&str, options:&str, time_info:&str, rate:&str, set_play_type:bool) -> String {
    match play_type {
        "01" => {
            get_rate_0102(match_code, options, time_info, rate, set_play_type) 
        },
        "02" => {
            get_rate_0202(match_code, options, time_info, rate, set_play_type) 
        },
        "03" => {
            get_rate_0302(match_code, options, time_info, rate, set_play_type) 
        },
        "04" => {
            get_rate_0402(match_code, options, time_info, rate, set_play_type) 
        },
        _  => {
            String::new()
        },
    }
}

///根据赔率详情获得赔率字符串
fn get_rate_by_map_and_op(match_code:&str, rate_map:&BTreeMap<&str, &str>, options:&str, play_type:&str, set_play_type:bool, rang:&str) -> String {
    let op_array:Vec<&str> = options.split(',').collect();
    let mut rate_info = String::new();
    let mut count = 0;
    for rate_op in op_array {
        if count > 0 {
            rate_info.push_str(",");
        }
        let rate; 
        if play_type == "02" || play_type == "04" {
            rate = format!("{}({})@{}", rate_op, rang, rate_map.get(rate_op).unwrap()); 
        } else {
            rate = format!("{}@{}", rate_op, rate_map.get(rate_op).unwrap()); 
        }
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

///胜负玩法的赔率
fn get_rate_0102(match_code:&str, options:&str, time_info:&str, rate:&str, set_play_type:bool) -> String {
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
            "负" => {
                key = "0";
            },
             _ => {
                key = "-1";
            },
        }
        back_map.insert(key, rate_detail);
    }
    get_rate_by_map_and_op(match_code, &back_map, options, "01", set_play_type, "") 
}

///让分胜负玩法的赔率
fn get_rate_0202(match_code:&str, options:&str, time_info:&str, rate:&str, set_play_type:bool) -> String {
    info!("the rate is {}.", rate);
    let rang_index = time_info.find("固定奖让分:");
    let (_, rang) = time_info.split_at(rang_index.unwrap() + 19);
    //info!("the rang is {}.", rang);
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
            "负" => {
                key = "0";
            },
             _ => {
                key = "-1";
            },
        }
        back_map.insert(key, rate_detail);
    }
    get_rate_by_map_and_op(match_code, &back_map, options, "02", set_play_type, rang) 
}

///胜分差玩法的赔率
fn get_rate_0302(match_code:&str, options:&str, time_info:&str, rate:&str, set_play_type:bool) -> String {
    info!("the rate is {}.", rate);
    let rate = rate.replace("26+", "26");
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
            "(主1-5)" => {
                key = "01";
            },
            "(主6-10)" => {
                key = "02";
            },
            "(主11-15)" => {
                key = "03";
            },
            "(主16-20)" => {
                key = "04";
            },
            "(主21-25)" => {
                key = "05";
            },
            "(主26)" => {
                key = "06";
            },
            "(客1-5)" => {
                key = "11";
            },
            "(客6-10)" => {
                key = "12";
            },
            "(客11-15)" => {
                key = "13";
            },
            "(客16-20)" => {
                key = "14";
            },
            "(客21-25)" => {
                key = "15";
            },
            "(客26)" => {
                key = "16";
            },
            _ => {
                key = "-1";
            },
        }
        back_map.insert(key, rate_detail);
    }
    get_rate_by_map_and_op(match_code, &back_map, options, "03", set_play_type, "") 
}

///大小分玩法的赔率
fn get_rate_0402(match_code:&str, options:&str, time_info:&str, rate:&str, set_play_type:bool) -> String {
    info!("the rate is {}.", rate);

    let rang_index = time_info.find("固定奖预设总分:");
    let (_, zf) = time_info.split_at(rang_index.unwrap() + 22);

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
            "大" => {
                key = "1";
            },
            "小" => {
                key = "2";
            },
            _ => {
                key = "-1";
            },
        }
        back_map.insert(key, rate_detail);
    }
    get_rate_by_map_and_op(match_code, &back_map, options, "04", set_play_type, zf) 
}

///获得一个场次的所有选择项
fn get_select_map(match_info:&str, play_type:&str) -> (i64, String, String) {
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
    (term_code, cur_play_type.to_string(), options.to_string())
}

///获得所有的期次数组，选项按期次，玩法的赔率map，m串n的玩法
fn get_select_vec(print_number:&str, play_type:&str) -> (Vec<(i64, String)>, BTreeMap<i64, String>, i64, i64) {
    let print_number_array:Vec<&str> = print_number.split("|").collect();
    let mn_array:Vec<&str> = print_number_array[1].split('*').collect();
    let m = i64::from_str(mn_array[0]).unwrap();
    let n = i64::from_str(mn_array[1]).unwrap();
    
    let mut map = BTreeMap::new();
    let mut vec = Vec::new();
    let match_info_array = print_number_array[0].split(";");
    for match_info in match_info_array {
        let (term_code, play_type, options) = get_select_map(match_info, play_type); 
        map.insert(term_code, options);
        vec.push((term_code, play_type));
    }
    (vec, map, m, n)
}

///获得本场次的赔率，主要区分正常场次和取消场次
fn check_get_rate(match_code:i64, play_type:&str, map:&BTreeMap<i64, String>, 
    gl:&BTreeMap<i64, String>) -> Option<f64> {

    //info!("match_code:{}", match_code);
    let options = map.get(&match_code).unwrap();
    //info!("options:{}", options);
    let draw_number = gl.get(&match_code).unwrap();
    //info!("draw_number:{}", draw_number);

    //info!("play_type:{}", play_type);
    let options_str_array:Vec<&str> = options.split(',').collect();
    //如果场次取消
    if draw_number == "*" {
        return Some(options_str_array.len() as f64); 
    }
    let draw_str_array:Vec<&str> = draw_number.split(',').collect();
    let master = i64::from_str(draw_str_array[1]).unwrap();
    let guest = i64::from_str(draw_str_array[0]).unwrap();
    if play_type == "01" {
        let mut rate_map = BTreeMap::<String, f64>::new();
        let ops:Vec<&str> = options.split(',').collect();
        for op in ops {
            let op_and_rate:Vec<&str> = op.split('@').collect();
            let rate = f64::from_str(op_and_rate[1]).unwrap();
            rate_map.insert(op_and_rate[0].to_string(), rate); 
        }

        let rst;
        if master > guest {
            rst = "3";
        } else {
            rst = "0";
        }
        info!("rst:{}", rst);
        let hit_op = rate_map.remove(rst);
        return hit_op;
    } else if play_type == "02" {
        let ops:Vec<&str> = options.split(',').collect();
        for op in ops {
            let op_and_rate:Vec<&str> = op.split('@').collect();
            let rate = f64::from_str(op_and_rate[1]).unwrap();
            let select_and_rang:Vec<&str> = op_and_rate[0].split('(').collect();
            let select = select_and_rang[0]; 
            let trim_chars:&[_] = &['+', ')'];
            let rang = select_and_rang[1].trim_matches(trim_chars);
            let rang_i64 = (f64::from_str(rang).unwrap()*10_f64) as i64;

            if select == "3" {
                if master*10 + rang_i64 > guest*10 {
                    return Some(rate);
                }
            } else {
                if master*10 + rang_i64 < guest*10 {
                    return Some(rate);
                }
            }
        }
    } else if play_type == "03" {
        let rst;
        let gap = master - guest;
        if gap > 0 {
            if gap <= 5 {
                rst = "01";
            } else if gap <= 10 {
                rst = "02";
            } else if gap <= 15 {
                rst = "03";
            } else if gap <= 20 {
                rst = "04";
            } else if gap <= 25 {
                rst = "05";
            } else {
                rst = "06";
            }
        } else {
            if gap >= -5 {
                rst = "11";
            } else if gap >= -10 {
                rst = "12";
            } else if gap >= -15 {
                rst = "13";
            } else if gap >= -20 {
                rst = "14";
            } else if gap >= -25 {
                rst = "15";
            } else {
                rst = "16";
            }
        }
        
        let mut rate_map = BTreeMap::<String, f64>::new();
        let ops:Vec<&str> = options.split(',').collect();
        for op in ops {
            let op_and_rate:Vec<&str> = op.split('@').collect();
            let rate = f64::from_str(op_and_rate[1]).unwrap();
            rate_map.insert(op_and_rate[0].to_string(), rate); 
        }

        info!("rst:{}", rst);
        let hit_op = rate_map.remove(rst);
        return hit_op;
    } else if play_type == "04" {
        let ops:Vec<&str> = options.split(',').collect();
        for op in ops {
            let op_and_rate:Vec<&str> = op.split('@').collect();
            let rate = f64::from_str(op_and_rate[1]).unwrap();
            let select_and_rang:Vec<&str> = op_and_rate[0].split('(').collect();
            let select = select_and_rang[0]; 
            let trim_chars:&[_] = &['+', ')'];
            let rang = select_and_rang[1].trim_matches(trim_chars);
            let rang_i64 = (f64::from_str(rang).unwrap()*10_f64) as i64;

            if select == "1" {
                if master*10 + guest*10 > rang_i64 {
                    return Some(rate);
                }
            } else {
                if master*10 + guest*10 < rang_i64 {
                    return Some(rate);
                }
            }
        }
    }
    None
}

///竞彩算奖算法
fn check(play_type:&str, ticket:&Json, gl:&BTreeMap<i64, String>) -> DrawResult {

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
                let mut hit = true;
                let mut cur_bonus = 200_f64;
                for vec_index in set {
                    let (term_code, ref play_type) = vec[vec_index as usize];
                    let rate_op = check_get_rate(term_code, play_type, &map, gl);
                    if let Some(rate) = rate_op {
                        cur_bonus *= rate;
                    } else {
                        hit = false;
                        cur_bonus = 0_f64;
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

///分解开奖号码，key为场次号，value为此场次的开奖号码
fn get_gl(ticket:&Json) -> BTreeMap<i64, String> {
    let mut map = BTreeMap::<i64, String>::new();
    let draw_code_list_node = ticket.find("draw_code_list").unwrap();
    let draw_code_list = draw_code_list_node.as_string().unwrap();
    let draw_code_array:Vec<&str> = draw_code_list.split(';').collect();
    for term_info in draw_code_array {
        let term_info_array:Vec<&str> = term_info.split(':').collect();
        let key = i64::from_str(term_info_array[0]).unwrap();
        map.insert(key, term_info_array[1].to_string());
    }
    map
}

pub struct DrawJcl0102;

impl Draw for DrawJcl0102 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, _:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {
        let gl = get_gl(ticket);
        check("01", ticket, &gl)
	}
}

impl Print for DrawJcl0102 {
	fn get_print_number(&self, number:&str, stub:&str) -> String {
        get_rate(number, "01", stub)
	}
}

pub struct DrawJcl0202;

impl Draw for DrawJcl0202 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, _:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {
        let gl = get_gl(ticket);
        check("02", ticket, &gl)
	}
}

impl Print for DrawJcl0202 {
	fn get_print_number(&self, number:&str, stub:&str) -> String {
        get_rate(number, "02", stub)
	}
}

pub struct DrawJcl0302;

impl Draw for DrawJcl0302 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, _:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {
        let gl = get_gl(ticket);
        check("03", ticket, &gl)
	}
}

impl Print for DrawJcl0302 {
	fn get_print_number(&self, number:&str, stub:&str) -> String {
        get_rate(number, "03", stub)
	}
}

pub struct DrawJcl0402;

impl Draw for DrawJcl0402 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, _:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {
        let gl = get_gl(ticket);
        check("04", ticket, &gl)
	}
}

impl Print for DrawJcl0402 {
	fn get_print_number(&self, number:&str, stub:&str) -> String {
        get_rate(number, "04", stub)
	}
}

pub struct DrawJcl1002;

impl Draw for DrawJcl1002 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, _:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {
        let gl = get_gl(ticket);
        check("10", ticket, &gl)
	}
}

impl Print for DrawJcl1002 {
	fn get_print_number(&self, number:&str, stub:&str) -> String {
        get_rate(number, "10", stub)
	}
}

lazy_static! {
	static ref HP:Helper = Helper::new();
}
