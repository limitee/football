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

struct Helper {
	bf_items: HashSet<String>,
	ha_items: HashSet<String>,
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
		
		Helper {
			bf_items: set,
			ha_items: ha,
		}
	}
	
	pub fn check_format(num:&str) -> Result<i32, i32> {
		let re = Regex::new(r"^[0-9]{11}:[0-9]{1,2}(,[0-9]{1,2}){0,}\|1\*1$").unwrap();
		if !re.is_match(num) {
            return Result::Err(ErrCode::NumberIsWrong as i32);
        }
		Result::Ok(1)
	}
	
	
}

pub struct ValidateJcd0101;

impl Validate for ValidateJcd0101 {
	
	fn validate(&self, ticket: &Json, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult {
		let number = json_str!(ticket; "number");
		try!(Helper::check_format(number));
		let number_array:Vec<&str> = number.split('|').collect();
		let number_array:Vec<&str> = number_array[0].split(':').collect();
		
		let select_array:Vec<&str> = number_array[1].split(',').collect();
		let mut count = 0;
		for select in select_array {
			if select == "3" || select == "1" || select == "0" {
				count += 1;
			} else {
				return Result::Err(ErrCode::NumberIsWrong as i32);
			}
		}
		Result::Ok((count))
	}
}

pub struct ValidateJcd0201;

impl Validate for ValidateJcd0201 {
	
	fn validate(&self, ticket: &Json, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult {
		let number = json_str!(ticket; "number");
		try!(Helper::check_format(number));
		let number_array:Vec<&str> = number.split('|').collect();
		let number_array:Vec<&str> = number_array[0].split(':').collect();
		
		let select_array:Vec<&str> = number_array[1].split(',').collect();
		let mut count = 0;
		for select in select_array {
			if select == "3" || select == "1" || select == "0" {
				count += 1;
			} else {
				return Result::Err(ErrCode::NumberIsWrong as i32);
			}
		}
		Result::Ok((count))
	}
}

pub struct ValidateJcd0301;

impl Validate for ValidateJcd0301 {
	
	fn validate(&self, ticket: &Json, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult {
		let number = json_str!(ticket; "number");
		try!(Helper::check_format(number));
		let number_array:Vec<&str> = number.split('|').collect();
		let number_array:Vec<&str> = number_array[0].split(':').collect();
		
		let select_array:Vec<&str> = number_array[1].split(',').collect();
		let mut count = 0;
		for select in select_array {
			let select_int = i64::from_str(select).unwrap();
			if select_int >= 0_i64 && select_int <= 7_i64 {
				count += 1;
			} else {
				return Result::Err(ErrCode::NumberIsWrong as i32);
			}
		}
		Result::Ok((count))
	}
}

pub struct ValidateJcd0401;

impl Validate for ValidateJcd0401 {
	
	fn validate(&self, ticket: &Json, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult {
		let number = json_str!(ticket; "number");
		try!(Helper::check_format(number));
		let number_array:Vec<&str> = number.split('|').collect();
		let number_array:Vec<&str> = number_array[0].split(':').collect();
		
		let select_array:Vec<&str> = number_array[1].split(',').collect();
		let mut count = 0;
		for select in select_array {
			if HP.bf_items.contains(select) {
				count += 1;
			} else {
				return Result::Err(ErrCode::NumberIsWrong as i32);
			}
		}
		Result::Ok((count))
	}
}

pub struct ValidateJcd0501;

impl Validate for ValidateJcd0501 {
	
	fn validate(&self, ticket: &Json, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult {
		let number = json_str!(ticket; "number");
		try!(Helper::check_format(number));
		let number_array:Vec<&str> = number.split('|').collect();
		let number_array:Vec<&str> = number_array[0].split(':').collect();
		
		let select_array:Vec<&str> = number_array[1].split(',').collect();
		let mut count = 0;
		for select in select_array {
			if HP.ha_items.contains(select) {
				count += 1;
			} else {
				return Result::Err(ErrCode::NumberIsWrong as i32);
			}
		}
		Result::Ok((count))
	}
}

lazy_static! {
	static ref HP:Helper = Helper::new();
}
