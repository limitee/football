#[macro_use]
extern crate lazy_static;

extern crate game;
use game::validate::ValidateFactory;
use game::base::Ticket;
use game::VF;
use game::DF;
use game::PF;

extern crate cons;
use cons::ErrCode;

extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::str::FromStr;

#[macro_use]
extern crate log;
extern crate elog;

#[macro_use]
extern crate easy_util;

extern crate util;
use util::NumberUtil;

use std::collections::BTreeMap;

fn get_gl_value(draw_op:&str) -> Json {
    let mut rst = json!("{}");
    let array = NumberUtil::to_int_array(draw_op);
    //胜平负
    let all;
    if array[0] > array[1] {
        all = "3";
    } else if array[0] == array[1] {
        all = "1";
    } else {
        all = "0";
    }
    json_set!(&mut rst; "01"; all);
    //让球胜平负
    let value;
    if array[0] + array[2] > array[1] {
        value = "3";
    } else if array[0] + array[2] == array[1] {
        value = "1";
    } else {
        value = "0";
    }
    json_set!(&mut rst; "02"; value);
    //总进球数
    let mut value = (array[0] + array[1])/10;
    if value > 7 {
        value = 7;
    }
    json_set!(&mut rst; "03"; value.to_string());
    //比分
    let mut master;
    let mut guest;
    if array[0] > array[1] {
        master = array[0]/10;
        guest = array[1]/10;
        if master > 5 || guest > 2 {
            master = 9;
            guest = 0;
        }
    } else if array[0] == array[1] {
        master = array[0]/10;
        if master > 3 {
            master = 9;
        }
        guest = master;
    } else {
        master = array[0]/10;
        guest = array[1]/10;
        if master > 2 || guest > 5 {
            master = 0;
            guest = 9;
        }
    }
    let bf = format!("{}{}", master, guest);
    json_set!(&mut rst; "04"; bf);
    //半场
    let half;
    if array[3] > array[4] {
        half = "3";
    } else if array[3] == array[4] {
        half = "1";
    } else {
        half = "0";
    }
    let ha = format!("{}{}", half, all);
    json_set!(&mut rst; "05"; ha);
    rst
}

fn get_gl_map(draw_array:&Vec<&str>) -> BTreeMap<i64, Json> {
    let mut gl_map = BTreeMap::<i64, Json>::new();
    for match_info in draw_array {
        let match_info_array:Vec<&str> = match_info.split(":").collect();
        let term_code = i64::from_str(match_info_array[0]).unwrap();
        let mut rst = get_gl_value(match_info_array[1]);
        gl_map.insert(term_code, rst); 
    }
    gl_map
}

struct Center {
    dlt_gl_map:BTreeMap<i64, Json>,
    jcc_gl_map:BTreeMap<i64, Json>,
}

impl Center {
    
    fn new() -> Center {
        let dlt_gl_map = Center::get_dlt_gl_map(); 
        let jcc_gl_map = Center::get_jcc_gl_map(); 
        Center {
            dlt_gl_map: dlt_gl_map,
            jcc_gl_map: jcc_gl_map,
        }
    }

    fn get_jcc_gl_map() -> BTreeMap<i64, Json> {
        let draw_number = "20160417001:20,20,0,10,10;20160417002:20,20,0,10,10;20160417003:20,30,0,20,10";
        let draw_array:Vec<&str> = draw_number.split(";").collect();
        return get_gl_map(&draw_array);
    }

    fn get_dlt_gl_map() -> BTreeMap<i64, Json> {
        let mut map = BTreeMap::new();
        for i in 1..14 {
            let lev = i as i64;
            let bonus = lev*100;
            map.insert(lev, Center::get_dlt_lev(lev, bonus, bonus));
        }
        map
    }

    fn get_dlt_lev(lev:i64, bonus:i64, bonus_after_tax:i64) -> Json {
        let mut json = json!("{}");
        json_set!(&mut json; "lev"; lev);
        json_set!(&mut json; "bonus"; bonus);
        json_set!(&mut json; "bonus_after_tax"; bonus_after_tax);
        json
    }
}


fn get_ticket<'a>(game_id:i64, play_type:i64, bet_type:i64, number:&'a str, print_number:&'a str) -> Json {
    let mut ticket = json!("{}");
    json_set!(&mut ticket; "game_id"; game_id);
    json_set!(&mut ticket; "play_type"; play_type);
    json_set!(&mut ticket; "bet_type"; bet_type);
    json_set!(&mut ticket; "number"; number);
    json_set!(&mut ticket; "print_number"; print_number);
    ticket
}

#[test]
fn draw_jcc() {
    let ticket = get_ticket(201, 1, 2, "", "20160417001:3@2.00,1@1.99,0@1.20;20160417002:3@2.00,1@1.99,0@1.20|2*1");
    let rst = DF.draw(&ticket, &Vec::<i32>::new(), &CT.jcc_gl_map);
    assert!(rst.is_ok());
    let rst_json = rst.unwrap();
    let bonus = rst_json.find("bonus").unwrap().as_f64().unwrap();
    assert_eq!(bonus, 200_f64*1.99*1.99);

    let ticket = get_ticket(201, 5, 2, "", "20160417001:33@2.00;20160417003:11@1.99,00@1.20|2*1");
    let rst = DF.draw(&ticket, &Vec::<i32>::new(), &CT.jcc_gl_map);
    assert!(rst.is_ok());
    let rst_json = rst.unwrap();
    let bonus = rst_json.find("bonus").unwrap().as_f64().unwrap();
    assert_eq!(bonus, 0_f64);

    //半全场
    let ticket = get_ticket(201, 5, 2, "", "20160417001:11@2.90;20160417003:30@1.99,03@1.20|2*1");
    let rst = DF.draw(&ticket, &Vec::<i32>::new(), &CT.jcc_gl_map);
    assert!(rst.is_ok());
    let rst_json = rst.unwrap();
    let bonus = rst_json.find("bonus").unwrap().as_f64().unwrap();
    assert_eq!(bonus, 200_f64*2.90*1.99);

    /*
    let ticket = get_ticket(201, 10, 2, "", "20160417001:01:1@2.00;20160417002:05:11@1.99,00@1.20|2*1");
    let rst = DF.draw(&ticket, &Vec::<i32>::new(), &CT.jcc_gl_map);
    assert!(rst.is_ok());
    let rst_json = rst.unwrap();
    let bonus = json_i64!(&rst_json; "bonus");
    assert_eq!(bonus, (200_f64*2.00*1.99) as i64);
    */
}

#[test]
fn draw_dlt() {
    let draw_number_str = "01,02,03,04,05|01,02";
	let draw_number = NumberUtil::to_int_array(draw_number_str);
	let ticket = Ticket::new(1_i64, 200, 10, 10, 1, 1, 200, "03,04,05,09,10|02,03");
	let rst = DF.draw(&ticket.to_json(), &draw_number, &CT.dlt_gl_map);
	assert!(rst.is_ok());
	println!("{}", rst.unwrap());
}

#[test]
fn validate_dlt() {

   let ticket = Ticket::new(1_i64, 100, 10, 10, 1, 1, 200, "01,02,03,04,05,06|01");
	let rst = VF.validate(&ticket.to_json());
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (1));
	
	let ticket = Ticket::new(1_i64, 100, 10, 10, 2, 1, 400, "01,02,03,04,05,06|01;01,02,03,04,05,26|10");
	let rst = VF.validate(&ticket.to_json());
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (2));
	
	let ticket = Ticket::new(1_i64, 100, 10, 10, 6, 1, 1200, "01,02,03,04,05,06|01;01,02,03,04,05,26|10;01,02,03,04,05,26|10;01,02,03,04,05,26|10;01,02,03,04,05,26|10;01,02,03,04,05,26|10");
	let rst = VF.validate(&ticket.to_json());
	assert!(rst.is_err());
	assert!(rst.err().unwrap() == ErrCode::CountBtFive as i32);

    //dlt 标准 单式
	let ticket = Ticket::new(1_i64, 200, 10, 10, 1, 1, 200, "01,02,03,04,05|01,02");
	let rst = VF.validate(&ticket.to_json());
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (1));
	
	//dlt 标准 复式
	let ticket = Ticket::new(1_i64, 200, 10, 20, 3, 1, 600, "01,02,03,04,05|01,02,03");
	let rst = VF.validate(&ticket.to_json());
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (3));
	
	//dlt 标准 复式
	let ticket = Ticket::new(1_i64, 200, 10, 20, 18, 1, 3600, "01,02,03,04,05,09|01,02,03");
	let rst = VF.validate(&ticket.to_json());
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (18));
	
	//dlt 标准 胆托
	let ticket = Ticket::new(1_i64, 200, 10, 30, 2, 1, 400, "02,03,04,05,09|01$02,03");
	let rst = VF.validate(&ticket.to_json());
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (2));
	
	//dlt 标准 胆托
	let ticket = Ticket::new(1_i64, 200, 10, 30, 4, 1, 800, "03,04,05,09$01,10|01$02,03");
	let rst = VF.validate(&ticket.to_json());
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (4));
}

#[test]
fn validate_jcd() {
    //jcd 胜平负 标准
	let ticket = Ticket::new(1_i64, 202, 1, 1, 1, 1, 200, "20121029002:3|1*1");
	let rst = VF.validate(&ticket.to_json());
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (1));
	
	let ticket = Ticket::new(1_i64, 202, 1, 1, 3, 1, 600, "20121029002:3,1,0|1*1");
	let rst = VF.validate(&ticket.to_json());
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (3));
	
	let ticket = Ticket::new(1_i64, 202, 3, 1, 3, 1, 600, "20121029002:1,3,5|1*1");
	let rst = VF.validate(&ticket.to_json());
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (3));
}

#[test]
fn validate_jcc() {
    //串关，胜平负
	let ticket = Ticket::new(1_i64, 201, 1, 2, 4, 1, 800, "20121029002:3,1;20121029002:3,1|2*1");
	let rst = VF.validate(&ticket.to_json());
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (4));

    //串关，胜平负
	let ticket = Ticket::new(1_i64, 201, 1, 2, 9, 1, 1800, "20121029002:3,1,0;20121029002:3,1,0|2*1");
	let rst = VF.validate(&ticket.to_json());
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (9));

    //串关，让球胜平负
	let ticket = Ticket::new(1_i64, 201, 2, 2, 4, 1, 800, "20121029002:3,1;20121029002:3,1|2*1");
	let rst = VF.validate(&ticket.to_json());
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (4));
    
    //串关，总进球数
	let ticket = Ticket::new(1_i64, 201, 3, 2, 4, 1, 800, "20121029002:3,1;20121029002:3,1|2*1");
	let rst = VF.validate(&ticket.to_json());
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (4));

    //串关，比分
	let ticket = Ticket::new(1_i64, 201, 4, 2, 4, 1, 800, "20121029002:03,01;20121029002:30,10|2*1");
	let rst = VF.validate(&ticket.to_json());
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (4));

    //串关，半全场
	let ticket = Ticket::new(1_i64, 201, 5, 2, 4, 1, 800, "20121029002:03,11;20121029002:33,00|2*1");
	let rst = VF.validate(&ticket.to_json());
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (4));

    //串关，混投
	let ticket = Ticket::new(1_i64, 201, 10, 2, 4, 1, 800, "20121029002:01:3,1;20121029002:02:3,0|2*1");
	let rst = VF.validate(&ticket.to_json());
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (4));


}

lazy_static! {
    static ref CT:Center = Center::new();
}
