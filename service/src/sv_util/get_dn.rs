extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::md5::Md5;

extern crate hyper;
use self::hyper::client::{Client, RedirectPolicy};
use hyper::header::Headers;

use std::thread;
use std::io::Read;
use std::str::FromStr;

extern crate time;

extern crate xml;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::{OwnedAttribute};

extern crate regex;
use regex::Regex;

extern crate rustc_serialize;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;

pub fn get_time() -> i64 {
    let now = time::get_time();
    now.sec*1000 + (now.nsec/1000000) as i64
}

pub fn md5(input:&str) -> String {
    let mut sh = Md5::new();
    sh.input_str(input);
    let out_str = sh.result_str();
    out_str
}

/**
 * get a random key by time
 */
pub fn random_key() -> String {
    let now = time::get_time();
    let str = format!("{:?}", &now);
    md5(&str)
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

/*
fn main() {
    let url = "http://www.lottery.gov.cn/news/11048739.shtml";
    let (term_code, draw_number, gl_list) = get_dlt_draw_info(url);
    println!("term code is {}.", term_code);
    println!("draw_number is {}.", draw_number);
    for gl in gl_list {
        println!("{}", gl);
    }
}
*/

pub fn get_dlt_draw_info(url:&str) -> (i64, String, Vec<Json>) {
    let mut client = Client::new();
    client.set_redirect_policy(RedirectPolicy::FollowNone);
    let key = random_key();
    let mut rb = client.get(url);
    let mut headers = Headers::new();
    let ua = format!("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/46.0.2490.80 Safari/537.36 {}", key);
    headers.set_raw("User-Agent", vec![ua.as_bytes().to_vec()]);
    let nrb = rb.headers(headers);
    let mut res = nrb.send().unwrap();

    let mut content = String::new();
    res.read_to_string(&mut content);

    let re = Regex::new(r"&[a-z|A-Z]+;").unwrap();
    let content = re.replace_all(&content, "");

    let re = Regex::new(r"(<script[\s\S]*?</script>)").unwrap();
    let content = re.replace_all(&content, "");

    let mut term_code_op = None;
    let mut dn_op = None;
    //开奖结果
    let mut result_vec = Vec::<Vec<String>>::new();
    let mut vec = Vec::<String>::new();

    let parser = EventReader::from_str(content.as_str());
    let mut depth = 0;
    let mut record_tr = false;
    let mut record_depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.local_name == "table" {
                    //println!("{}+{},dep:{}", indent(depth), name.local_name, depth);
                    let id_op = get_attr(&attributes, "id");
                    if let Some(id) = id_op {
                        //println!("the table's id is {}.", id);
                        if id == "result" {
                            record_tr = true;
                            record_depth = depth;
                        }
                    }
                }
                if name.local_name == "tr" && record_tr {
                    vec = Vec::<String>::new();
                    //println!("{}+{},dep:{}", indent(depth), name.local_name, depth);
                }
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                if name.local_name == "table" {
                    //println!("{}-{},dep:{}", indent(depth), name.local_name, depth);
                    if depth == record_depth {
                        record_tr = false;
                    }
                }
                if name.local_name == "tr" && record_tr {
                    //println!("{}+{},dep:{}", indent(depth), name.local_name, depth);
                    result_vec.push(vec.clone());
                }
            }
            Ok(XmlEvent::Characters (data)) => {
                if record_tr {
                    //println!("data is {}.", data); 
                    let data = data.trim_left().trim_right();
                    vec.push(data.to_string());
                } else {
                    if data.contains("中国体育彩票超级大乐透") {
                        let re = Regex::new(r"\d{5}").unwrap();
                        let cap = re.captures(&data).unwrap();
                        let term_code_str = cap.at(0).unwrap();
                        let term_code_string = format!("20{}", term_code_str);
                        term_code_op = Some(i64::from_str(term_code_string.as_str()).unwrap());
                    } else if data.contains("本期开奖结果") {
                        let re = Regex::new(r"(\d{2})").unwrap();
                        let mut vec = Vec::new();
                        for cap in re.captures_iter(&data) {
                            vec.push(cap.at(0).unwrap());
                        }
                        let draw_number = format!("{},{},{},{},{}|{},{}", vec[0], vec[1], vec[2], vec[3], vec[4], vec[5], vec[6]);
                        dn_op = Some(draw_number);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    let term_code = term_code_op.unwrap();
    let mut gl_list = Vec::<Json>::new();

    let first_index = get_index(&result_vec, "一等奖").unwrap();
    let second_index = get_index(&result_vec, "二等奖").unwrap();
    let gap = second_index - first_index;
    if gap == 2 {
        let ref line = result_vec[first_index];
        let gl = get_normal_gl(line, term_code, 1, "一等奖");
        gl_list.push(gl);

        let ref line = result_vec[first_index + 1];
        let gl = get_append_gl(line, term_code, 7, "一等奖追加");
        gl_list.push(gl);
    } else if gap == 4 {
        let ref line_1 = result_vec[first_index];
        let ref line_2 = result_vec[first_index + 1];
        let gl = get_normal_and_append_gl(line_1, 3, line_2, 2, term_code, 1, "一等奖");
        gl_list.push(gl);
        
        let ref line_1 = result_vec[first_index + 2];
        let ref line_2 = result_vec[first_index + 3];
        let gl = get_normal_and_append_gl(line_1, 2, line_2, 2, term_code, 7, "一等奖追加");
        gl_list.push(gl);
    }
    let third_index = get_index(&result_vec, "三等奖").unwrap();
    let gap = third_index - second_index;
    if gap == 2 {
        let ref line = result_vec[second_index];
        let gl = get_normal_gl(line, term_code, 2, "二等奖");
        gl_list.push(gl);

        let ref line = result_vec[second_index + 1];
        let gl = get_append_gl(line, term_code, 8, "二等奖追加");
        gl_list.push(gl);
    }
    let forth_index = get_index(&result_vec, "四等奖").unwrap();
    let gap = forth_index - third_index;
    if gap == 2 {
        let ref line = result_vec[third_index];
        let gl = get_normal_gl(line, term_code, 3, "三等奖");
        gl_list.push(gl);

        let ref line = result_vec[third_index + 1];
        let gl = get_append_gl(line, term_code, 9, "三等奖追加");
        gl_list.push(gl);
    }

    let fifth_index = get_index(&result_vec, "五等奖").unwrap();
    let gap = fifth_index - forth_index;
    if gap == 2 {
        let ref line = result_vec[forth_index];
        let gl = get_normal_gl(line, term_code, 4, "四等奖");
        gl_list.push(gl);

        let ref line = result_vec[forth_index + 1];
        let gl = get_append_gl(line, term_code, 10, "四等奖追加");
        gl_list.push(gl);
    } else if gap == 3 {
        let ref line = result_vec[forth_index];
        let gl = get_normal_gl(line, term_code, 4, "四等奖");
        gl_list.push(gl);
        
        let ref line_1 = result_vec[forth_index + 1];
        let ref line_2 = result_vec[forth_index + 2];
        let gl = get_normal_and_append_gl(line_1, 2, line_2, 2, term_code, 10, "四等奖追加");
        gl_list.push(gl);
    }

    let sixth_index = get_index(&result_vec, "六等奖").unwrap();
    let gap = sixth_index - fifth_index;
    if gap == 2 {
        let ref line = result_vec[fifth_index];
        let gl = get_normal_gl(line, term_code, 5, "五等奖");
        gl_list.push(gl);

        let ref line = result_vec[fifth_index + 1];
        let gl = get_append_gl(line, term_code, 11, "五等奖追加");
        gl_list.push(gl);
    } else if gap == 3 {
        let ref line = result_vec[fifth_index];
        let gl = get_normal_gl(line, term_code, 5, "五等奖");
        gl_list.push(gl);
        
        let ref line_1 = result_vec[fifth_index + 1];
        let ref line_2 = result_vec[fifth_index + 2];
        let gl = get_normal_and_append_gl(line_1, 2, line_2, 2, term_code, 11, "五等奖追加");
        gl_list.push(gl);
    }

    let end_index = get_index(&result_vec, "合计").unwrap();
    let gap = end_index - sixth_index;
    if gap == 1 {
        let ref line = result_vec[sixth_index];
        let gl = get_append_gl(line, term_code, 6, "六等奖");
        gl_list.push(gl);

        let gl = get_gl(200, term_code, 12, "六等奖派奖", 0, 0);
        gl_list.push(gl);
    } else if gap == 2 {
        let ref line = result_vec[sixth_index];
        let gl = get_normal_gl(line, term_code, 6, "六等奖");
        gl_list.push(gl);
        
        let ref line = result_vec[sixth_index + 1];
        let gl = get_append_gl(line, term_code, 12, "六等奖派奖");
        gl_list.push(gl);
    }

    return (term_code_op.unwrap(), dn_op.unwrap(), gl_list);
}

//get the normal game level
fn get_normal_and_append_gl(line_1:&Vec<String>, index_1:usize, line_2:&Vec<String>, index_2:usize, term_code:i64, 
    lev:i64, descrip:&str) -> Json {
    let mut bonus = 0;
    let mut bonus_after_tax = 0;

    let ref bonus_str = line_1[index_1]; 
    let (cur_bonus, cur_bat) = get_bonus(bonus_str);
    bonus += cur_bonus;
    bonus_after_tax += cur_bat;
        
    let ref bonus_str = line_2[index_2]; 
    let (cur_bonus, cur_bat) = get_bonus(bonus_str);
    bonus += cur_bonus;
    bonus_after_tax += cur_bat;

    let gl = get_gl(200, term_code, lev, descrip, bonus, bonus_after_tax);
    gl
}

//get the normal game level
fn get_normal_gl(line:&Vec<String>, term_code:i64, 
    lev:i64, descrip:&str) -> Json {
    let trim_chars:&[_] = &['元', ' ', ','];
    let ref bonus = line[3]; 
    let (bonus_i64, bonus_after_tax) = get_bonus(bonus);
    let gl = get_gl(200, term_code, lev, descrip, bonus_i64, bonus_after_tax);
    gl
}

//get the normal game level
fn get_append_gl(line:&Vec<String>, term_code:i64, 
    lev:i64, descrip:&str) -> Json {
    let ref bonus = line[2]; 
    let (bonus_i64, bonus_after_tax) = get_bonus(bonus);
    let gl = get_gl(200, term_code, lev, descrip, bonus_i64, bonus_after_tax);
    gl
}

fn get_bonus(bonus:&str) -> (i64, i64) {
    let trim_chars:&[_] = &['元', ' ', ','];
    let bonus_i64;
    if bonus == "---" {
        bonus_i64 = 0;
    } else {
        let bonus = bonus.replace(trim_chars, "");
        bonus_i64 = i64::from_str(bonus.as_str()).unwrap();
    }
    let bonus_i64 = bonus_i64*100;
    let bonus_after_tax;
    if bonus_i64 >= 1000000 {
        bonus_after_tax = ((bonus_i64 as f64)*0.8) as i64;
    } else {
        bonus_after_tax = bonus_i64;
    }
    (bonus_i64, bonus_after_tax)
}

//get the index of the start
fn get_index(result:&Vec<Vec<String>>, start:&str) -> Option<usize> {
    let mut op = None;
    let len = result.len();
    for i in 0..len {
        let ref line = result[i];
        let ref line_start = line[0];
        if line_start.starts_with(start) {
            op = Some(i);
            break;
        }
    }
    return op;
}

//get the index of the start, after the start index
fn get_index_after(result:&Vec<Vec<String>>, start_index:usize, start:&str) -> Option<usize> {
    let mut op = None;
    let len = result.len();
    for i in start_index..len {
        let ref line = result[i];
        let ref line_start = line[0];
        if line_start.starts_with(start) {
            op = Some(i);
            break;
        }
    }
    return op;
}

fn get_gl(game_id:i64, term_code:i64, lev:i64, descrip:&str, bonus:i64, bonus_after_tax:i64) -> Json {
    let mut gl = json::Object::new();
    gl.insert("game_id".to_string(), game_id.to_json());
    gl.insert("term_code".to_string(), term_code.to_json());
    gl.insert("lev".to_string(), lev.to_json());
    gl.insert("descrip".to_string(), descrip.to_json());
    gl.insert("bonus".to_string(), bonus.to_json());
    gl.insert("bonus_after_tax".to_string(), bonus_after_tax.to_json());
    let now = time::get_time();
    gl.insert("create_time".to_string(), now.sec.to_json());
    return gl.to_json();
}


