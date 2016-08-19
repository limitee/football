extern crate hyper;
use self::hyper::client::{Client, RedirectPolicy};
use hyper::header::Headers;

use std::thread;
use std::io::Read;
use std::str::FromStr;

extern crate regex;
use regex::Regex;

extern crate chrono;
use chrono::*;

use std::collections::BTreeMap;

/*
fn main() {
    let now = Local::now();
    let now_str = now.format("%Y%m%d").to_string();
    let (_, date_str) = now_str.split_at(2);

    let dur = Duration::days(1);
    let last_day = now - dur;
    let last_day_str = last_day.format("%Y%m%d").to_string();
    let (_, last_date_str) = last_day_str.split_at(2);

    println!("{}", date_str);
    let url = format!("http://www.9188.com/zqjc/jsbf/pc/jc/{}.js", date_str);
    get_jcc_draw_info(&url);

    println!("{}", last_date_str);
    let url = format!("http://www.9188.com/zqjc/jsbf/pc/jc/{}.js", last_date_str);
    let rst = get_jcc_draw_info(&url);
    if let Err(msg) = rst {
        println!("Err:{}.", msg); 
        return;
    }
    let map = rst.unwrap();
    for (key, value) in map {
        println!("{}, {}", key, value);
    }
}
*/

pub fn get_jcc_draw_info(date:&DateTime<Local>) -> Result<BTreeMap<i64, String>, String> {
    let date_str = date.format("%Y%m%d").to_string();
    let (_, short_date_str) = date_str.split_at(2);
    let url = format!("http://www.9188.com/zqjc/jsbf/pc/jc/{}.js", 
        short_date_str);

    info!("{}", url);
    let stamp = Local::now().timestamp();
    let url = format!("{}?_={}", url, stamp);
    let mut client = Client::new();
    client.set_redirect_policy(RedirectPolicy::FollowNone);
    let mut rb = client.get(url.as_str());
    let res_rst = rb.send().or(Err("网络错误".to_string()));
    let mut res = try!(res_rst);

    let mut content = String::new();
    res.read_to_string(&mut content);
    let content = content.trim_left().trim_right();
    //println!("{}", content);

    let mut map = BTreeMap::<i64, String>::new();

    let re = Regex::new(r"^var matchs=\[\[([\s\S]+)\]\];$").unwrap();
    let cap_op = re.captures(&content);
    if cap_op.is_none() {
        info!("{}", content);
        return Result::Err("内容格式错误".to_string());
    }
    let cap = cap_op.unwrap();
    let match_str = cap.at(1).unwrap();
    let term_list:Vec<&str> = match_str.split("],[").collect();
    let trim_chars:&[_] = &['\''];
    for term_str in term_list {
        //println!("{}", term_str);
        let mut vec = Vec::<String>::new();
        let term_info_array:Vec<&str> = term_str.split(",").collect();
        for info in term_info_array {
            let cur = info.trim_matches(trim_chars); 
            vec.push(cur.to_string());
        }
        let status = i64::from_str(&vec[1]).unwrap();
        if status != 4 {
            continue;
        }
        let code_str = format!("20{}", vec[0]);
        let code = i64::from_str(code_str.as_str()).unwrap();
        let give = i64::from_str(&vec[7]).unwrap();
        let m_all = i64::from_str(&vec[16]).unwrap();
        let g_all = i64::from_str(&vec[17]).unwrap();
        let half_array:Vec<&str> =  vec[18].split("-").collect();
        let m_half = i64::from_str(&half_array[0]).unwrap();
        let g_half = i64::from_str(&half_array[1]).unwrap();
        let draw_code = format!("{},{},{},{},{}", m_all*10, g_all*10, give*10, m_half*10, g_half*10);
        map.insert(code, draw_code);
    }

    return Result::Ok(map);
}
