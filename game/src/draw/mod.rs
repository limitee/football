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
use super::base::DrawResult;
use super::base::Draw;
use super::base::Print;

mod dlt;
use self::dlt::*;

mod jcd;
use self::jcd::*;

mod jcc;
use self::jcc::*;

mod jcl;
use self::jcl::*;

pub struct DrawFactory {
	map:BTreeMap<String, Box<Draw>>,
}

macro_rules! add_inter {
    ($o:expr, $k:expr, $v:expr) => {{
        $o.insert($k.to_string(), Box::new($v) as Box<Draw>);
    }}
}

impl DrawFactory {
	
	pub fn new() -> DrawFactory {
		let mut map = BTreeMap::new();
        add_inter!(map, "2001010", DrawDlt1010);
        add_inter!(map, "2001020", DrawDlt1020);
        add_inter!(map, "2001030", DrawDlt1030);
        
        add_inter!(map, "2006010", DrawDlt1010);
        add_inter!(map, "2006020", DrawDlt1020);
        add_inter!(map, "2006030", DrawDlt1030);
        
        add_inter!(map, "2020101", DrawJcd0101);
        add_inter!(map, "2020201", DrawJcd0201);
        add_inter!(map, "2020301", DrawJcd0301);
        add_inter!(map, "2020401", DrawJcd0401);
        add_inter!(map, "2020501", DrawJcd0501);

        add_inter!(map, "2010102", DrawJcc0102);
        add_inter!(map, "2010202", DrawJcc0202);
        add_inter!(map, "2010302", DrawJcc0302);
        add_inter!(map, "2010402", DrawJcc0402);
        add_inter!(map, "2010502", DrawJcc0502);
        add_inter!(map, "2011002", DrawJcc1002);

        //jcl
        add_inter!(map, "3010101", DrawJcl0102);
        add_inter!(map, "3010102", DrawJcl0102);

        add_inter!(map, "3010201", DrawJcl0202);
        add_inter!(map, "3010202", DrawJcl0202);

        add_inter!(map, "3010301", DrawJcl0302);
        add_inter!(map, "3010302", DrawJcl0302);

        add_inter!(map, "3010401", DrawJcl0402);
        add_inter!(map, "3010402", DrawJcl0402);
        
        add_inter!(map, "3011002", DrawJcl1002);
        
        DrawFactory {
            map:map,
        }
	}
	
	pub fn draw(&self, ticket:&Json, draw_number:&Vec<i32>, gl:&BTreeMap<i64, Json>) -> DrawResult {
		//info!("{}", ticket);
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
		let draw = self.map.get(&key).unwrap();
		let rst = draw.draw(ticket, draw_number, gl, game, play_type, bet_type);
		rst
	}
}

pub struct PrintFactory {
	map:BTreeMap<String, Box<Print>>,
}

macro_rules! add_inter_print {
    ($o:expr, $k:expr, $v:expr) => {{
        $o.insert($k.to_string(), Box::new($v) as Box<Print>);
    }}
}

impl PrintFactory {
	
	pub fn new() -> PrintFactory {
		let mut map = BTreeMap::new();
        
        add_inter_print!(map, "2020101", DrawJcd0101);
        add_inter_print!(map, "2020201", DrawJcd0101);
        add_inter_print!(map, "2020301", DrawJcd0301);
        add_inter_print!(map, "2020401", DrawJcd0401);
        add_inter_print!(map, "2020501", DrawJcd0501);

        add_inter_print!(map, "2010102", DrawJcc0102);
        add_inter_print!(map, "2010202", DrawJcc0202);
        add_inter_print!(map, "2010302", DrawJcc0302);
        add_inter_print!(map, "2010402", DrawJcc0402);
        add_inter_print!(map, "2010502", DrawJcc0502);
        add_inter_print!(map, "2011002", DrawJcc1002);

        //jcl
        add_inter_print!(map, "3010101", DrawJcl0102);
        add_inter_print!(map, "3010102", DrawJcl0102);
        
        add_inter_print!(map, "3010201", DrawJcl0202);
        add_inter_print!(map, "3010202", DrawJcl0202);

        add_inter_print!(map, "3010301", DrawJcl0302);
        add_inter_print!(map, "3010302", DrawJcl0302);

        add_inter_print!(map, "3010401", DrawJcl0402);
        add_inter_print!(map, "3010402", DrawJcl0402);

        add_inter_print!(map, "3011002", DrawJcl1002);

        PrintFactory {
            map:map,
        }
	}
    
	pub fn get_print_number(&self, game_id:i64, play_type_id:i64, bet_type_id:i64, number:&str, stub:&str) -> Result<String, i32> {
		//info!("{}", ticket);
		
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
		let draw = self.map.get(&key).unwrap();
		let rst = draw.get_print_number(number, stub);
		Result::Ok(rst)
	}

}


