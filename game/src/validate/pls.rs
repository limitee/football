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

pub struct ValidatePls0101;

impl Validate for ValidatePls0101 {
	
	fn validate(&self, ticket: &Json, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult {
		let number = json_str!(ticket; "number");
		let v: Vec<&str> = number.split(';').collect();
		let count = v.len() as i32;
		if count > 5 {
			return Result::Err(ErrCode::CountBtFive as i32);
		}
		let re = Regex::new(r"^([0-9]{1}\|){2}[0-9]{1}$").unwrap();
		for num in &v {
            if !re.is_match(num) {
                return Result::Err(ErrCode::NumberIsWrong as i32);
            }
		}
		Result::Ok((count))
	}
}

pub struct ValidatePls0102;

impl Validate for ValidatePls0102 {
	
	fn validate(&self, ticket: &Json, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult {
		let number = json_str!(ticket; "number");
        let re = Regex::new(r"^[0-9](,[0-9]){0,9}(\|[0-9](,[0-9]){0,9}){2}$").unwrap();
        if !re.is_match(number) {
            return Result::Err(ErrCode::NumberIsWrong as i32);
        }
        let mut count = 1;
		let number_array:Vec<&str> = number.split('|').collect();
        for num in number_array {
            let int_num_array = NumberUtil::to_int_array(num);
            try!(NumberUtil::array_sort_from_min_to_max(&int_num_array).or(
		   	        Result::Err(ErrCode::NumberIsWrong as i32)
				)
			);
            count = count*int_num_array.len() as i32;
        }
        if count <= 1 {
            return Result::Err(ErrCode::NumberIsWrong as i32);
        }
		Result::Ok((count))
	}
}

