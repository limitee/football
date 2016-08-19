extern crate dc;
use dc::MyDbPool;
use dc::DataBase;

extern crate hyper;
use self::hyper::client::{Client, RedirectPolicy};
use hyper::header::Headers;
use hyper::client::Body;

use std::thread;
use std::io::Read;
use std::str::FromStr;

extern crate regex;
use regex::Regex;

extern crate chrono;
use chrono::*;

use std::collections::BTreeMap;

extern crate encoding;
use encoding::{Encoding, ByteWriter, EncoderTrap, DecoderTrap};
use encoding::all::GBK;

extern crate xml;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::{OwnedAttribute};

extern crate sev_helper;
use sev_helper::QueryService;

pub fn handle_jcc_by_date(date:&DateTime<Local>, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
    let mut page = 0;
    loop {
        let rst = get_jcl_draw_info(date, page);
        let (page_count, map) = try!(rst.or(Err(-1)));
        info!("the page count is {}.", page_count);
        let _ = try!(handle_map(&map, db));
        if page_count < 34 || page >= 3 {
            break;
        } else {
            page += 1;
        }
    }
    Result::Ok(1)
}

/**
 * 更新数据库，只有已停售的才更新，且要增加version版本
 */
fn handle_map(map:&BTreeMap<i64, String>, db:&DataBase<MyDbPool>) -> Result<i32, i32> {
    for (key, value) in map {
        info!("{}-{}", key, value);
        let sql = format!("update term set draw_number='{}',version=version+1 where game_id=201 and code={} and status=50", value, key);
        let rst = try!(db.execute(&sql));
    }
    return Result::Ok(1);
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn get_attr(attrs:&Vec<OwnedAttribute>, name:&str) -> Option<String> {
    let mut op = None;
    for attr in attrs {
        if attr.name.local_name == name {
            op = Some(attr.value.to_string());
        }
    }
    op
}

fn get_jcl_draw_info(date:&DateTime<Local>, page:i64) -> Result<(i64, BTreeMap<i64, String>), String> {
    let url = "http://info.sporttery.cn/football/match_result.php"; 
    let mut headers = Headers::new();
    {
        headers.set_raw("Content-Type", vec![b"application/x-www-form-urlencoded".to_vec()]);
        headers.set_raw("Connection", vec![b"keep-alive".to_vec()]);
        headers.set_raw("User-Agent", vec![b"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2661.102 Safari/537.36".to_vec()]);
    }
    for item in headers.iter() {
        info!("{}", item);
    }

    info!("{}", url);
    let body_string = format!("start_date={}&end_date={}&page={}", date.format("%Y-%m-%d").to_string(), date.format("%Y-%m-%d").to_string(), page); 

    let mut client = Client::new();
    let mut rb = client.post(url);
    let rb = rb.headers(headers).body(body_string.as_bytes());
    let res_rst = rb.send().or(Err("网络错误".to_string()));
    let mut res = try!(res_rst);
    info!("status:{:?}", res.status);
    info!("{:?}", res.headers);
    info!("{}", res.url);
    info!("{:?}", res.version);

    let mut buf = Vec::<u8>::new();
    let rst = res.read_to_end(&mut buf);
    info!("{:?}", rst);
    let _ = try!(rst.or(Err("读取数据错误")));
    let rst = GBK.decode(&buf, DecoderTrap::Strict);
    let content = rst.unwrap();

    let re = Regex::new(r"&[a-z|A-Z]+;").unwrap();
    let content = re.replace_all(&content, "");

    let re = Regex::new(r"(<script[\s\S]*?</script>)").unwrap();
    let content = re.replace_all(&content, "");

    let re = Regex::new(r"(<!--[\s\S]*?-->)").unwrap();
    let content = re.replace_all(&content, "");

    let re = Regex::new(r"(<img[\s\S]*?>)").unwrap();
    let content = re.replace_all(&content, "");

    let re = Regex::new(r"(<br>)").unwrap();
    let content = re.replace_all(&content, "");

    let re = Regex::new("(<a href=\"match_result.php[\\s\\S]*?</a>)").unwrap();
    let content = re.replace_all(&content, "");

    //info!("{}", content);

    //开奖结果
    let mut result_vec = Vec::<Vec<String>>::new();
    let mut vec = Vec::<String>::new();
    let parser = EventReader::from_str(content.as_str());
    let mut depth = 0;
    let mut record_tr = false;
    let mut record_depth = 0;
    let mut page_count = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.local_name == "div" {
                    let class_op = get_attr(&attributes, "class");
                    if let Some(class) = class_op {
                        if class == "match_list" {
                            record_tr = true;
                            record_depth = depth;
                        }
                    }
                }
                if name.local_name == "tr" && record_tr {
                    vec = Vec::<String>::new();
                }
                depth += 1;
            },
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                if name.local_name == "div" {
                    if depth == record_depth {
                        record_tr = false;
                    }
                }
                if name.local_name == "tr" && record_tr {
                    page_count += 1;
                    if vec.len() == 10 {
                        result_vec.push(vec.clone());
                    }
                }
            },
            Ok(XmlEvent::Characters (data)) => {
                if record_tr {
                    let data = data.trim_left().trim_right();
                    vec.push(data.to_string());
                }
            },
            Err(e) => {
                println!("Error: {}", e);
                break;
            },
            _ => {}
        }
    }

    let mut map = BTreeMap::<i64, String>::new();
    for set in result_vec {
        info!("{:?}", set);
        let (week_str, index_str) = set[1].split_at(6);
        let time = format!("{} 12:00:00", set[0]);
        let end_time = Local.datetime_from_str(time.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();
        let day = QueryService.get_day(week_str, &end_time);
        let term_code_string = format!("{}{}", day, index_str);
        let term_code = i64::from_str(&term_code_string).unwrap();
        if set[8] == "\u{5df2}\u{5b8c}\u{6210}" && set[9] == "\u{8be6}\u{7ec6}" {
            let half_vec:Vec<&str> = set[6].split(':').collect();
            let master_half = i64::from_str(half_vec[0]).unwrap();
            let guest_half = i64::from_str(half_vec[1]).unwrap();
            
            let all_vec:Vec<&str> = set[7].split(':').collect();
            let master_all = i64::from_str(all_vec[0]).unwrap();
            let guest_all = i64::from_str(all_vec[1]).unwrap();

            let x: &[_] = &['(', ')'];
            let master = set[3].trim_matches(x);
            let master_len = master.len();
            let (_, give_str) = master.split_at(master_len - 2); 
            let give:i64 = give_str.parse().unwrap();

            let draw_number = format!("{},{},{},{},{}", master_all*10, guest_all*10, give*10, master_half*10, guest_half*10); 
            map.insert(term_code, draw_number);
        }
    }

    return Result::Ok((page_count, map));
}
