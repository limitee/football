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

use super::super::base::ValidateResult;
use super::super::base::Validate;
use super::super::base::Game;
use super::super::base::PlayType;
use super::super::base::BetType;

use std::collections::HashSet;
use std::collections::HashMap;

struct Helper {
	bf_items: HashSet<String>,
	ha_items: HashSet<String>,
    bunch_map: HashMap<String, String>,
}

impl Helper {
	
	pub fn new() -> Helper {
		let mut set = HashSet::new();
		set.insert("10".to_string());
		set.insert("20".to_string());
		set.insert("30".to_string());
		set.insert("40".to_string());
		set.insert("50".to_string());
		set.insert("21".to_string());
		set.insert("31".to_string());
		set.insert("41".to_string());
		set.insert("51".to_string());
		set.insert("32".to_string());
		set.insert("42".to_string());
		set.insert("52".to_string());
		set.insert("90".to_string());
		
		set.insert("00".to_string());
		set.insert("11".to_string());
		set.insert("22".to_string());
		set.insert("33".to_string());
		set.insert("99".to_string());
		
		set.insert("01".to_string());
		set.insert("02".to_string());
		set.insert("03".to_string());
		set.insert("04".to_string());
		set.insert("05".to_string());
		set.insert("12".to_string());
		set.insert("13".to_string());
		set.insert("14".to_string());
		set.insert("15".to_string());
		set.insert("23".to_string());
		set.insert("24".to_string());
		set.insert("25".to_string());
		set.insert("09".to_string());
		
		let mut ha = HashSet::new();
		ha.insert("33".to_string());
		ha.insert("31".to_string());
		ha.insert("30".to_string());
		ha.insert("03".to_string());
		ha.insert("01".to_string());
		ha.insert("00".to_string());
		ha.insert("13".to_string());
		ha.insert("11".to_string());
		ha.insert("10".to_string());

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
			bf_items: set,
			ha_items: ha,
            bunch_map: bunch_map,
		}
	}
	
	pub fn check_format(num:&str) -> Result<i32, i32> {
		let re = Regex::new(r"^[0-9]{11}:[0-9]{1,2}(,[0-9]{1,2}){0,}(;[0-9]{11}:[0-9]{1,2}(,[0-9]{1,2}){0,}){0,}\|\d\*\d$").unwrap();
		if !re.is_match(num) {
            return Result::Err(ErrCode::NumberIsWrong as i32);
        }
		Result::Ok(1)
	}

    //校验混投玩法
	pub fn check_ht_format(num:&str) -> Result<i32, i32> {
		let re = Regex::new(r"^[0-9]{11}:\d{2}:[0-9]{1,2}(,[0-9]{1,2}){0,}(;[0-9]{11}:\d{2}:[0-9]{1,2}(,[0-9]{1,2}){0,}){0,}\|\d\*\d$").unwrap();
		if !re.is_match(num) {
            return Result::Err(ErrCode::NumberIsWrong as i32);
        }
		Result::Ok(1)
	}

    ///竞彩，根据每一场的选项数，及串关特征，获得注数信息
    pub fn get_count(choice_length:&Vec<i32>, mn:&str) -> i32 {
        let mut count = 0;
        let mn_array:Vec<&str> = mn.split('*').collect();
        let m = i64::from_str(mn_array[0]).unwrap();
        let n = i64::from_str(mn_array[1]).unwrap();
        let bunch_key = format!("{}{}", m, n);
        let bunch_flag = HP.bunch_map.get(bunch_key.as_str()).unwrap();
        let mut index = 0;
        for token in bunch_flag.chars() {
            if token == '1' {
                let dc = MathUtil::get_detailc(m, index + 1); 
                for set in dc.data {
                    let mut cur_count = 1;
                    for choice_index in set {
                        cur_count *= choice_length[choice_index as usize]; 
                    }
                    count += cur_count;
                }
            } 
            index += 1;
        }
        println!("{}", count);
        println!("{:?}", choice_length);
        count
    }
}

pub struct ValidateJcc0102;

impl Validate for ValidateJcc0102 {
	
	fn validate(&self, ticket: &Json, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult {
		let number = json_str!(ticket; "number");
		try!(Helper::check_format(number));
		let number_array:Vec<&str> = number.split('|').collect();
        let number_cc_array:Vec<&str> = number_array[0].split(';').collect();

        //校验选项是否正确，并获得每一场的选项数量
        let mut choice_length = Vec::<i32>::new();
        for number_array in number_cc_array {
            let number_array:Vec<&str> = number_array.split(':').collect();
		    let select_array:Vec<&str> = number_array[1].split(',').collect();
            choice_length.push(select_array.len() as i32);
		    for select in select_array {
			    if select == "3" || select == "1" || select == "0" {

			    } else {
				    return Result::Err(ErrCode::NumberIsWrong as i32);
			    }
		    }
        }
        
        let count = Helper::get_count(&choice_length, number_array[1]);
       	Result::Ok((count))
	}
}

pub struct ValidateJcc0202;

impl Validate for ValidateJcc0202 {
	
	fn validate(&self, ticket: &Json, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult {
        let number = json_str!(ticket; "number");
		try!(Helper::check_format(number));
		let number_array:Vec<&str> = number.split('|').collect();
        let number_cc_array:Vec<&str> = number_array[0].split(';').collect();

        //校验选项是否正确，并获得每一场的选项数量
        let mut choice_length = Vec::<i32>::new();
        for number_array in number_cc_array {
            let number_array:Vec<&str> = number_array.split(':').collect();
		    let select_array:Vec<&str> = number_array[1].split(',').collect();
            choice_length.push(select_array.len() as i32);
		    for select in select_array {
			    if select == "3" || select == "1" || select == "0" {

			    } else {
				    return Result::Err(ErrCode::NumberIsWrong as i32);
			    }
		    }
        }
        
        let count = Helper::get_count(&choice_length, number_array[1]);
       	Result::Ok((count))
	}
}

pub struct ValidateJcc0302;

impl Validate for ValidateJcc0302 {
	
	fn validate(&self, ticket: &Json, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult {
        let number = json_str!(ticket; "number");
		try!(Helper::check_format(number));
		let number_array:Vec<&str> = number.split('|').collect();
        let number_cc_array:Vec<&str> = number_array[0].split(';').collect();

        //校验选项是否正确，并获得每一场的选项数量
        let mut choice_length = Vec::<i32>::new();
        for number_array in number_cc_array {
            let number_array:Vec<&str> = number_array.split(':').collect();
		    let select_array:Vec<&str> = number_array[1].split(',').collect();
            choice_length.push(select_array.len() as i32);
		    for select in select_array {
                let select_int = i64::from_str(select).unwrap();
			    if select_int >= 0_i64 && select_int <= 7_i64 {
			    } else {
				    return Result::Err(ErrCode::NumberIsWrong as i32);
			    }
		    }
        }
        
        let count = Helper::get_count(&choice_length, number_array[1]);
       	Result::Ok((count))
	}
}

pub struct ValidateJcc0402;

impl Validate for ValidateJcc0402 {
	
	fn validate(&self, ticket: &Json, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult {
        let number = json_str!(ticket; "number");
		try!(Helper::check_format(number));
		let number_array:Vec<&str> = number.split('|').collect();
        let number_cc_array:Vec<&str> = number_array[0].split(';').collect();

        //校验选项是否正确，并获得每一场的选项数量
        let mut choice_length = Vec::<i32>::new();
        for number_array in number_cc_array {
            let number_array:Vec<&str> = number_array.split(':').collect();
		    let select_array:Vec<&str> = number_array[1].split(',').collect();
            choice_length.push(select_array.len() as i32);
		    for select in select_array {
			    if HP.bf_items.contains(select) {

			    } else {
				    return Result::Err(ErrCode::NumberIsWrong as i32);
			    }
		    }
        }
        
        let count = Helper::get_count(&choice_length, number_array[1]);
       	Result::Ok((count))
	}
}

pub struct ValidateJcc0502;

impl Validate for ValidateJcc0502 {
	
	fn validate(&self, ticket: &Json, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult {
        let number = json_str!(ticket; "number");
		try!(Helper::check_format(number));
		let number_array:Vec<&str> = number.split('|').collect();
        let number_cc_array:Vec<&str> = number_array[0].split(';').collect();

        //校验选项是否正确，并获得每一场的选项数量
        let mut choice_length = Vec::<i32>::new();
        for number_array in number_cc_array {
            let number_array:Vec<&str> = number_array.split(':').collect();
		    let select_array:Vec<&str> = number_array[1].split(',').collect();
            choice_length.push(select_array.len() as i32);
		    for select in select_array {
			    if HP.ha_items.contains(select) {

			    } else {
				    return Result::Err(ErrCode::NumberIsWrong as i32);
			    }
		    }
        }
        
        let count = Helper::get_count(&choice_length, number_array[1]);
       	Result::Ok((count))
	}
}

pub struct ValidateJcc1002;

impl Validate for ValidateJcc1002 {
	
	fn validate(&self, ticket: &Json, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult {
        let number = json_str!(ticket; "number");
		try!(Helper::check_ht_format(number));
		let number_array:Vec<&str> = number.split('|').collect();
        let number_cc_array:Vec<&str> = number_array[0].split(';').collect();

        //校验选项是否正确，并获得每一场的选项数量
        let mut choice_length = Vec::<i32>::new();
        for number_array in number_cc_array {
            let number_array:Vec<&str> = number_array.split(':').collect();
            let play_type = number_array[1]; 
		    let select_array:Vec<&str> = number_array[2].split(',').collect();
            choice_length.push(select_array.len() as i32);
		    for select in select_array {
                match play_type {
                    "01" => {
			            if select != "3" && select != "1" && select != "0" {
				            return Result::Err(ErrCode::NumberIsWrong as i32);
                        }
                    },
                    "02" => {
                        if select != "3" && select != "1" && select != "0" {
				            return Result::Err(ErrCode::NumberIsWrong as i32);
                        }
                    },
                    "03" => {
                        let select_int = i64::from_str(select).unwrap();
			            if select_int < 0_i64 || select_int > 7_i64 {
				            return Result::Err(ErrCode::NumberIsWrong as i32);
			            }
                    },
                    "04" => {
			            if !HP.bf_items.contains(select) {
				            return Result::Err(ErrCode::NumberIsWrong as i32);
                        }
                    },
                    "05" => {
			            if !HP.ha_items.contains(select) {
				            return Result::Err(ErrCode::NumberIsWrong as i32);
                        }
                    },
                    _ => {
				        return Result::Err(ErrCode::NumberIsWrong as i32);
                    }
                }
		    }
        }
        
        let count = Helper::get_count(&choice_length, number_array[1]);
       	Result::Ok((count))
	}
}

lazy_static! {
	static ref HP:Helper = Helper::new();
}
