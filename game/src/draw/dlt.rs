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
use super::super::base::Game;
use super::super::base::PlayType;
use super::super::base::BetType;

use std::collections::BTreeMap;

type LvDetail = (i32, i32, i64, i64);

struct Helper {
	relation: Vec<Vec<i32>>,	//配备数和奖级关系表
}

impl Helper {
	
	pub fn new() -> Helper {
		let mut relation = Vec::new();
		relation.push(vec![5, 2, 1]);
		relation.push(vec![5, 1, 2]);
		relation.push(vec![5, 0, 3]);
		relation.push(vec![4, 2, 3]);
		relation.push(vec![4, 1, 4]);
		relation.push(vec![4, 0, 5]);
		relation.push(vec![3, 2, 4]);
		relation.push(vec![3, 1, 5]);
		relation.push(vec![3, 0, 6]);
		relation.push(vec![2, 2, 5]);
		relation.push(vec![2, 1, 6]);
		relation.push(vec![1, 2, 6]);
		relation.push(vec![0, 2, 6]);
		Helper {
			relation: relation,
		}
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
	pub fn append_rst(&self, detail:&mut BTreeMap<i32, LvDetail>, gl:&BTreeMap<i64, Json>, lev:i32, count:i32) -> (i64, i64) {
		if count <= 0 {
			return (0_i64, 0_i64);
		}
		let cur_gl = gl.get(&(lev as i64)).unwrap();
		let bonus = json_i64!(cur_gl; "bonus")*count as i64;
		let bonus_after_tax = json_i64!(cur_gl; "bonus_after_tax")*count as i64;
		
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

pub struct DrawDlt1010;

impl Draw for DrawDlt1010 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, gl:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {
		
		let mut json = json!("{}");
		let (mut bonus, mut bonus_after_tax) = (0_i64, 0_i64);
		let mut detail = BTreeMap::<i32, LvDetail>::new();
		
		let number = json_str!(ticket; "number");
		let v: Vec<&str> = number.split(';').collect();
		for num in &v {
            let num_int_array = NumberUtil::to_int_array(num);
            let red_hit_count = MathUtil::get_hit_count(&num_int_array[0..5], &draw_number[0..5]);
            let blue_hit_count = MathUtil::get_hit_count(&num_int_array[5..7], &draw_number[5..7]);
            
            for set in &HP.relation {
           		if set[0] == red_hit_count && set[1] == blue_hit_count {
           			let (b, bat) = HP.append_rst(&mut detail, gl, set[2], 1);
           			bonus += b;
           			bonus_after_tax += bat;
           			
           			if play_type.id == 60  {	//处理追加
           				let (b, bat) = HP.append_rst(&mut detail, gl, set[2] + 6, 1);
	           			bonus += b;
	           			bonus_after_tax += bat;
           			}
           		}
            }
		}
		Result::Ok(HP.get_rst(detail, bonus, bonus_after_tax))
	}
}

pub struct DrawDlt1020;

impl Draw for DrawDlt1020 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, gl:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {
		
		let mut json = json!("{}");
		let (mut bonus, mut bonus_after_tax) = (0_i64, 0_i64);
		let mut detail = BTreeMap::<i32, LvDetail>::new();
		
		let number = json_str!(ticket; "number");
		let v: Vec<&str> = number.split('|').collect();
		let red_num = v[0];
		let blue_num = v[1];
		
        let red_int_array = NumberUtil::to_int_array(red_num);
        let blue_int_array = NumberUtil::to_int_array(blue_num);
        let red_hit_count = MathUtil::get_hit_count(&red_int_array, &draw_number[0..5]) as i64;
        let blue_hit_count = MathUtil::get_hit_count(&blue_int_array, &draw_number[5..7]) as i64;
        for set in &HP.relation {
       		let red_count = MathUtil::get_c(red_hit_count, set[0] as i64)*MathUtil::get_c(red_int_array.len() as i64 - red_hit_count, 5_i64 - set[0] as i64);
       		let blue_count = MathUtil::get_c(blue_hit_count, set[1] as i64)*MathUtil::get_c(blue_int_array.len() as i64 - blue_hit_count, 2_i64 - set[1] as i64);
       		let count = red_count*blue_count;
       		let (b, bat) = HP.append_rst(&mut detail, gl, set[2], count as i32);
    		bonus += b;
    		bonus_after_tax += bat;
    			
    		if play_type.id == 60 {	//处理追加
    			let (b, bat) = HP.append_rst(&mut detail, gl, set[2] + 6, count as i32);
       			bonus += b;
       			bonus_after_tax += bat;
    		}
        }
		Result::Ok(HP.get_rst(detail, bonus, bonus_after_tax))
	}
}

pub struct DrawDlt1030;

impl Draw for DrawDlt1030 {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, gl:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult {
		
		let mut json = json!("{}");
		let (mut bonus, mut bonus_after_tax) = (0_i64, 0_i64);
		let mut detail = BTreeMap::<i32, LvDetail>::new();
		
		let number = json_str!(ticket; "number");
		let v: Vec<&str> = number.split('|').collect();
		let red_num = v[0];
		let blue_num = v[1];
		
		let red_num_array:Vec<&str> = red_num.split('$').collect();
		let (red_dan_int_array, red_tuo_int_array) = {
			if red_num_array.len() == 1_usize {
				let dan_int_array = NumberUtil::to_int_array(red_num_array[0]);
				let tuo_int_array = Vec::<i32>::new();
				(dan_int_array, tuo_int_array)
			} else {
				let dan_int_array = NumberUtil::to_int_array(red_num_array[0]);
				let tuo_int_array = NumberUtil::to_int_array(red_num_array[1]);
				(dan_int_array, tuo_int_array)
			}
		};
		
		let blue_num_array:Vec<&str> = blue_num.split('$').collect();
		let (blue_dan_int_array, blue_tuo_int_array) = {
			if blue_num_array.len() == 1_usize {
				let dan_int_array = NumberUtil::to_int_array(blue_num_array[0]);
				let tuo_int_array = Vec::<i32>::new();
				(dan_int_array, tuo_int_array)
			} else {
				let dan_int_array = NumberUtil::to_int_array(blue_num_array[0]);
				let tuo_int_array = NumberUtil::to_int_array(blue_num_array[1]);
				(dan_int_array, tuo_int_array)
			}
		};
		
		let red_dan_hit_count = MathUtil::get_hit_count(&red_dan_int_array, &draw_number[0..5]) as i64;
		let red_tuo_hit_count = MathUtil::get_hit_count(&red_tuo_int_array, &draw_number[0..5]) as i64;
		let blue_dan_hit_count = MathUtil::get_hit_count(&blue_dan_int_array, &draw_number[5..7]) as i64;
		let blue_tuo_hit_count = MathUtil::get_hit_count(&blue_tuo_int_array, &draw_number[5..7]) as i64;
		
		let red_dan_len = red_dan_int_array.len() as i64;
		let red_tuo_len = red_tuo_int_array.len() as i64;
		let blue_dan_len = blue_dan_int_array.len() as i64;
		let blue_tuo_len = blue_tuo_int_array.len() as i64;
		
		//info!("{}-{}-{}-{}", red_dan_len, red_tuo_len, blue_dan_len, blue_tuo_len);
		//info!("{}-{}-{}-{}", red_dan_hit_count, red_tuo_hit_count, blue_dan_hit_count, blue_tuo_hit_count);
		
        for set in &HP.relation {
       		let red_count = MathUtil::get_c(red_tuo_hit_count, set[0] as i64 - red_dan_hit_count)
       			*MathUtil::get_c(red_tuo_len - red_tuo_hit_count, 5_i64 - (set[0] as i64 - red_dan_hit_count) - red_dan_len);
       		let blue_count = MathUtil::get_c(blue_tuo_hit_count, set[1] as i64 - blue_dan_hit_count)
       			*MathUtil::get_c(blue_tuo_len - blue_tuo_hit_count, 2_i64 - (set[1] as i64 - blue_dan_hit_count) - blue_dan_len);
       		let count = red_count*blue_count;
       		//info!("{}-{}-{}", set[0], set[1], set[2]);
       		//info!("蓝球托码选中数:{}", MathUtil::get_c(blue_tuo_hit_count, set[1] as i64 - blue_dan_hit_count));
       		//info!("蓝球托码未选中数:{}", MathUtil::get_c(blue_tuo_len - blue_tuo_hit_count, 5_i64 - (set[1] as i64 - blue_dan_hit_count) - blue_dan_len));
       		//info!("{}-{}", red_count, blue_count);
       		let (b, bat) = HP.append_rst(&mut detail, gl, set[2], count as i32);
    		bonus += b;
    		bonus_after_tax += bat;
    			
    		if play_type.id == 60 {	//处理追加
    			let (b, bat) = HP.append_rst(&mut detail, gl, set[2] + 6, count as i32);
       			bonus += b;
       			bonus_after_tax += bat;
    		}
        }
		Result::Ok(HP.get_rst(detail, bonus, bonus_after_tax))
	}
}

lazy_static! {
	static ref HP:Helper = Helper::new();
}

