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

struct DltHelper;

impl DltHelper {
	
	pub fn check_format(num:&str) -> Result<(i32, i32), i32> {
		let num_array:Vec<&str> = num.split('|').collect();
		let red_num = num_array.get(0).unwrap();
		let blue_num = num_array.get(1).unwrap();
		
		let red_vec = NumberUtil::to_int_array(red_num);
		try!(NumberUtil::array_sort_from_min_to_max(&red_vec).or(
				Result::Err(ErrCode::NumberIsWrong as i32)
			)
		);
		try!(NumberUtil::check_margin(&red_vec, 1, 35).or(
				Result::Err(ErrCode::NumberIsWrong as i32)
			)
		);
		
		let blue_vec = NumberUtil::to_int_array(blue_num);
		try!(NumberUtil::array_sort_from_min_to_max(&blue_vec).or(
				Result::Err(ErrCode::NumberIsWrong as i32)
			)
		);
		let rst = NumberUtil::check_margin(&blue_vec, 1, 12).or(
			Result::Err(ErrCode::NumberIsWrong as i32)
		);
		let rst = rst.and_then(|_| {
			Result::Ok((red_vec.len() as i32, blue_vec.len() as i32))
		});
		rst
	}
	
}

pub struct ValidateDlt1010;

impl Validate for ValidateDlt1010 {
	
	fn validate(&self, ticket: &Json, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult {
		let number = json_str!(ticket; "number");
		let v: Vec<&str> = number.split(';').collect();
		let count = v.len() as i32;
		if count > 5 {
			return Result::Err(ErrCode::CountBtFive as i32);
		}
		let re = Regex::new(r"^([0-9]{2},){4}[0-9]{2}\|[0-9]{2},[0-9]{2}$").unwrap();
		for num in &v {
            if !re.is_match(num) {
                return Result::Err(ErrCode::NumberIsWrong as i32);
            }
            try!(DltHelper::check_format(num));
		}
		Result::Ok((count))
	}
}

pub struct ValidateDlt1020;

impl Validate for ValidateDlt1020 {
	
	fn validate(&self, ticket: &Json, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult {
		let number = json_str!(ticket; "number");
		let re = Regex::new(r"^([0-9]{2},){4,34}[0-9]{2}\|[0-9]{2}(,[0-9]{2}){1,11}$").unwrap();
        if !re.is_match(number) {
            return Result::Err(ErrCode::NumberIsWrong as i32);
        }
        let (red_len, blue_len) = try!(DltHelper::check_format(number));
        let count = MathUtil::get_c(red_len as i64, 5)*MathUtil::get_c(blue_len as i64, 2);
        Result::Ok((count as i32))
	}
}

pub struct ValidateDlt1030;

impl Validate for ValidateDlt1030 {
	
	fn validate(&self, ticket: &Json, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult {
		let number = json_str!(ticket; "number");
		let number_array:Vec<&str> = number.split('$').collect();
		let len = number_array.len();
		if len < 2 || len > 3 {
			return Result::Err(ErrCode::NumberIsWrong as i32);
		}
		let mut count = 0;
		if len == 2 {	//前胆托，或者后胆托
			let f_number = number_array.get(0).unwrap();
			let s_number = number_array.get(1).unwrap();
			if f_number.contains('|') {	//后胆托
				let re = Regex::new(r"^[0-9]{2}(,[0-9]{2}){4,34}\|[0-9]{2}$").unwrap();
				if !re.is_match(f_number) {
		            return Result::Err(ErrCode::NumberIsWrong as i32);
		        }
				let re = Regex::new(r"^[0-9]{2}(,[0-9]{2}){1,10}$").unwrap();
				if !re.is_match(s_number) {
		            return Result::Err(ErrCode::NumberIsWrong as i32);
		        }
				let f_number_array:Vec<&str> = f_number.split('|').collect();
				let red_number = f_number_array.get(0).unwrap();
				let blue_dan_number = f_number_array.get(1).unwrap();
				
				let red_array = NumberUtil::to_int_array(red_number);
				let blud_dan_array = NumberUtil::to_int_array(blue_dan_number);
				let blud_tuo_array = NumberUtil::to_int_array(s_number);
				
				try!(NumberUtil::array_sort_from_min_to_max(&red_array).or(
						Result::Err(ErrCode::NumberIsWrong as i32)
					)
				);
				try!(NumberUtil::array_sort_from_min_to_max(&blud_tuo_array).or(
						Result::Err(ErrCode::NumberIsWrong as i32)
					)
				);
				try!(NumberUtil::check_margin(&red_array, 1, 35).or(
						Result::Err(ErrCode::NumberIsWrong as i32)
					)
				);
				try!(NumberUtil::check_margin(&blud_dan_array, 1, 12).or(
						Result::Err(ErrCode::NumberIsWrong as i32)
					)
				);
				try!(NumberUtil::check_margin(&blud_tuo_array, 1, 12).or(
						Result::Err(ErrCode::NumberIsWrong as i32)
					)
				);
				if MathUtil::get_hit_count(&blud_dan_array, &blud_tuo_array) > 0 {
					return Result::Err(ErrCode::NumberIsWrong as i32);
				}
				count = MathUtil::get_c(red_array.len() as i64, 5)*MathUtil::get_c(blud_tuo_array.len() as i64, 1);
			} else {	//前胆托
				let re = Regex::new(r"^[0-9]{2}(,[0-9]{2}){0,3}$").unwrap();
				if !re.is_match(f_number) {
		            return Result::Err(ErrCode::NumberIsWrong as i32);
		        }
				let re = Regex::new(r"^[0-9]{2}(,[0-9]{2}){0,34}\|[0-9]{2}(,[0-9]{2}){1,11}$").unwrap();
				if !re.is_match(s_number) {
		            return Result::Err(ErrCode::NumberIsWrong as i32);
		        }
				let s_number_array:Vec<&str> = s_number.split('|').collect();
				let red_tuo_number = s_number_array.get(0).unwrap();
				let blue_number = s_number_array.get(1).unwrap();
				
				let red_dan_array = NumberUtil::to_int_array(f_number);
				let red_tuo_array = NumberUtil::to_int_array(red_tuo_number);
				let blue_array = NumberUtil::to_int_array(blue_number);
				
				try!(NumberUtil::array_sort_from_min_to_max(&red_dan_array).or(
						Result::Err(ErrCode::NumberIsWrong as i32)
					)
				);
				try!(NumberUtil::array_sort_from_min_to_max(&red_tuo_array).or(
						Result::Err(ErrCode::NumberIsWrong as i32)
					)
				);
				try!(NumberUtil::array_sort_from_min_to_max(&blue_array).or(
						Result::Err(ErrCode::NumberIsWrong as i32)
					)
				);
				try!(NumberUtil::check_margin(&red_dan_array, 1, 35).or(
						Result::Err(ErrCode::NumberIsWrong as i32)
					)
				);
				try!(NumberUtil::check_margin(&red_tuo_array, 1, 35).or(
						Result::Err(ErrCode::NumberIsWrong as i32)
					)
				);
				try!(NumberUtil::check_margin(&blue_array, 1, 12).or(
						Result::Err(ErrCode::NumberIsWrong as i32)
					)
				);
				if MathUtil::get_hit_count(&red_dan_array, &red_tuo_array) > 0 {
					return Result::Err(ErrCode::NumberIsWrong as i32);
				}
				count = MathUtil::get_c(red_tuo_array.len() as i64, 5_i64 - red_dan_array.len() as i64)*MathUtil::get_c(blue_array.len() as i64, 2);
			}
		} else {
			let re = Regex::new(r"^[0-9]{2}(,[0-9]{2}){0,3}\$[0-9]{2}(,[0-9]{2}){0,34}\|[0-9]{2}\$[0-9]{2}(,[0-9]{2}){1,11}$").unwrap();
			if !re.is_match(number) {
	            return Result::Err(ErrCode::NumberIsWrong as i32);
	        }
			
			let f_number = number_array.get(0).unwrap();
			let s_number = number_array.get(1).unwrap();
			
			let s_number_array:Vec<&str> = s_number.split('|').collect();
			let red_dan_number = f_number;
			let red_tuo_number = s_number_array.get(0).unwrap();
			let blue_dan_number = s_number_array.get(1).unwrap();
			let blue_tuo_number = number_array.get(2).unwrap();
			
			let red_dan_array = NumberUtil::to_int_array(red_dan_number);
			let red_tuo_array = NumberUtil::to_int_array(red_tuo_number);
			let blue_dan_array = NumberUtil::to_int_array(blue_dan_number);
			let blue_tuo_array = NumberUtil::to_int_array(blue_tuo_number);
			
			try!(NumberUtil::array_sort_from_min_to_max(&red_dan_array).or(
					Result::Err(ErrCode::NumberIsWrong as i32)
				)
			);
			try!(NumberUtil::check_margin(&red_dan_array, 1, 35).or(
					Result::Err(ErrCode::NumberIsWrong as i32)
				)
			);
			
			try!(NumberUtil::array_sort_from_min_to_max(&red_tuo_array).or(
					Result::Err(ErrCode::NumberIsWrong as i32)
				)
			);
			try!(NumberUtil::check_margin(&red_tuo_array, 1, 35).or(
					Result::Err(ErrCode::NumberIsWrong as i32)
				)
			);
			
			try!(NumberUtil::array_sort_from_min_to_max(&blue_dan_array).or(
					Result::Err(ErrCode::NumberIsWrong as i32)
				)
			);
			try!(NumberUtil::check_margin(&blue_dan_array, 1, 12).or(
					Result::Err(ErrCode::NumberIsWrong as i32)
				)
			);
			
			try!(NumberUtil::array_sort_from_min_to_max(&blue_tuo_array).or(
					Result::Err(ErrCode::NumberIsWrong as i32)
				)
			);
			try!(NumberUtil::check_margin(&blue_tuo_array, 1, 12).or(
					Result::Err(ErrCode::NumberIsWrong as i32)
				)
			);
			
			if MathUtil::get_hit_count(&red_dan_array, &red_tuo_array) > 0 {
				return Result::Err(ErrCode::NumberIsWrong as i32);
			}
			
			if MathUtil::get_hit_count(&blue_dan_array, &blue_tuo_array) > 0 {
				return Result::Err(ErrCode::NumberIsWrong as i32);
			}
			
			count = MathUtil::get_c(red_tuo_array.len() as i64, 5_i64 - red_dan_array.len() as i64)*MathUtil::get_c(blue_tuo_array.len() as i64, 2_i64 - blue_dan_array.len() as i64);
		}
		if count < 2_i64 {
			return Result::Err(ErrCode::NumberIsWrong as i32);
		}
        Result::Ok((count as i32))
	}
}