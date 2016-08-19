use std::collections::BTreeMap;

extern crate cons;
use cons::ErrCode;

extern crate util;
use util::NumberUtil;

extern crate regex;
use self::regex::Regex;

extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::str::FromStr;

use super::base::Ticket;
use super::base::Game;
use super::base::PlayType;
use super::base::BetType;
use super::base::GF;
use super::base::ValidateResult;
use super::base::Validate;

mod dlt;
use self::dlt::*;

mod jcd;
use self::jcd::*;

mod jcc;
use self::jcc::*;

mod jcl;
use self::jcl::*;

mod pls;
use self::pls::*;

pub struct ValidateSsq1010;

impl Validate for ValidateSsq1010 {
	
	fn validate(&self, ticket: &Json, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult {
		let amount = json_i64!(ticket; "amount");
		let price = play_type.get_price();
		let multiple = json_i64!(ticket; "multiple");
		let number = json_str!(ticket; "number");
		
		let v: Vec<&str> = number.split(';').collect();
		let count = v.len() as i32;
		if count > 5 {
			return Result::Err(ErrCode::CountBtFive as i32);
		}
		let re = Regex::new(r"^([0-9]{2},){5},[0-9]{2}|[0-9]{2}$").unwrap();
		for num in &v {
            if !re.is_match(num) {
                return Result::Err(ErrCode::NumberIsWrong as i32);
            }
		}
		let true_amount = (count as i64)*price as i64*multiple;
		let rst = {
			if amount == true_amount {
				Result::Ok((count))
			} else {
				Result::Err(ErrCode::AmountIsWrong as i32)
			}
		};
		rst
	}
}



pub struct ValidateFactory {
	map:BTreeMap<String, Box<Validate>>,
}

macro_rules! add_inter {
    ($o:expr, $k:expr, $v:expr) => {{
        $o.insert($k.to_string(), Box::new($v) as Box<Validate>);
    }}
}

impl ValidateFactory {
	
	pub fn new() -> ValidateFactory {
		let mut map = BTreeMap::new();
        add_inter!(map, "1001010", ValidateSsq1010);
        
        add_inter!(map, "2001010", ValidateDlt1010);
        add_inter!(map, "2001020", ValidateDlt1020);
        add_inter!(map, "2001030", ValidateDlt1030);
        
        add_inter!(map, "2006010", ValidateDlt1010);
        add_inter!(map, "2006020", ValidateDlt1020);
        add_inter!(map, "2006030", ValidateDlt1030);
        
        add_inter!(map, "2020101", ValidateJcd0101);
        add_inter!(map, "2020201", ValidateJcd0201);
        add_inter!(map, "2020301", ValidateJcd0301);
        add_inter!(map, "2020401", ValidateJcd0401);
        add_inter!(map, "2020501", ValidateJcd0501);
        
        add_inter!(map, "2010102", ValidateJcc0102);
        add_inter!(map, "2010202", ValidateJcc0202);
        add_inter!(map, "2010302", ValidateJcc0302);
        add_inter!(map, "2010402", ValidateJcc0402);
        add_inter!(map, "2010502", ValidateJcc0502);
        add_inter!(map, "2011002", ValidateJcc1002);

        //pls
        add_inter!(map, "2030101", ValidatePls0101);
        add_inter!(map, "2030102", ValidatePls0102);

        //jcl
        add_inter!(map, "3010101", ValidateJcl0101);
        add_inter!(map, "3010102", ValidateJcl0102);
        add_inter!(map, "3010201", ValidateJcl0201);
        add_inter!(map, "3010202", ValidateJcl0202);
        add_inter!(map, "3010301", ValidateJcl0301);
        add_inter!(map, "3010302", ValidateJcl0302);
        add_inter!(map, "3010401", ValidateJcl0401);
        add_inter!(map, "3010402", ValidateJcl0402);
        add_inter!(map, "3011002", ValidateJcl1002);

        ValidateFactory {
            map:map,
        }
	}
	
	pub fn validate(&self, ticket:&Json) -> ValidateResult {
		let game_id = json_i64!(ticket; "game_id");
		let play_type_id = json_i64!(ticket; "play_type");
		let bet_type_id = json_i64!(ticket; "bet_type");
		
		let game = try!(GF.get_game_by_id(game_id as i32));
		let play_type = try!(game.get_play_type(play_type_id as i32));
		let bet_type = try!(play_type.get_bet_type(bet_type_id as i32));
		
		let play_type_str;
		if play_type.id < 10 {
			play_type_str = format!("0{}", play_type.id);
		} else {
			play_type_str = format!("{}", play_type.id);
		}
		
		let bet_type_str;
		if bet_type.id < 10 {
			bet_type_str = format!("0{}", bet_type.id);
		} else {
			bet_type_str = format!("{}", bet_type.id);
		}
		
		let key = format!("{}{}{}", game_id, play_type_str, bet_type_str);
		let validate_op = self.map.get(&key);
        if validate_op.is_none() {
			return Result::Err(ErrCode::GameNotExists as i32);
        }
		let validate = validate_op.unwrap();
		
		let amount = json_i64!(ticket; "amount");
		let icount = json_i64!(ticket; "icount");
		let price = play_type.get_price();
		let multiple = json_i64!(ticket; "multiple");
		
		let rst = validate.validate(ticket, game, play_type, bet_type);
		let rst = rst.and_then(|count|{
			info!("the count is {}", count);
			let count = count as i64;
			let true_amount = count*price as i64*multiple;
			if amount == true_amount && icount == count {
				Result::Ok(count as i32)
			} else {
				Result::Err(ErrCode::AmountIsWrong as i32)
			}
		});
		rst
	}
}

