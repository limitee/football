use std::str::FromStr;

extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

extern crate rustc_serialize;
use rustc_serialize::json::{Json, ToJson};
use rustc_serialize::base64::{FromBase64, FromBase64Error};

use std::collections::BTreeMap;
extern crate time;

pub struct DigestUtil;

impl DigestUtil {

    pub fn md5(input:&str) -> String {
        let mut sh = Md5::new();
        sh.input_str(input);
        let out_str = sh.result_str();
        out_str
    }

    pub fn empty_key() -> String {
        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_string()
    }

    /**
     * get a random key by time
     */
    pub fn random_key() -> String {
        let now = time::get_time();
        let str = format!("{:?}", &now);
        DigestUtil::md5(&str)
    }

    /**
     * get bytes from base64 string
     */
    pub fn base64_to_bytes(str:&str) -> Result<Vec<u8>, FromBase64Error> {
        str.from_base64() 
    }
}


pub struct TimeUtil;

impl TimeUtil {

    pub fn get_cur_second() -> i64 {
        let now = time::get_time();
        now.sec
    }
}

pub struct DetailC {
    pub data:Vec<Vec<i64>>,
}

impl DetailC {
    
    pub fn new() -> DetailC {
        DetailC {
            data: Vec::new(),
        }
    }

    pub fn put(&mut self, set:Vec<i64>) {
        self.data.push(set); 
    }
}

pub struct MathUtil;

impl MathUtil {
	
    ///获得c(m, n)的详细记录集
    pub fn get_detailc(m:i64, n:i64) -> DetailC {
        let mut dc = DetailC::new();
        let mut data = Vec::<i64>::with_capacity(n as usize);
        for _ in 0..n as usize {
            data.push(0_i64);
        }
        MathUtil::get_detailc_record(&mut dc, &mut data, m, n, 0);
        dc
    }

    pub fn get_detailc_record(dc:&mut DetailC, data:&mut Vec<i64>, m:i64, n:i64, level:i64) {
        if n == 0_i64 {
            dc.put(data.clone());
            return;
        }
        let mut start_index = 0_i64;
        if level > 0 {
            start_index = data[level as usize - 1_usize] + 1;
        }
        for i in start_index..m {
            data[level as usize] = i; 
            MathUtil::get_detailc_record(dc, data, m, n - 1, level + 1);
        }
    }

	///获得C(m, n)的值
	pub fn get_c(m:i64, n:i64) -> i64 {
		if m < n || n < 0 {
	        return 0;
	    }
		return MathUtil::get_a(m, n) / MathUtil::get_a(n, n);
	}
	
	///获得A(m, n)的值
	pub fn get_a(m:i64, n:i64) -> i64 {
		let mut value = 1_i64;
		for key in 0..n {
			value = value*(m - key);
		}
		value
	}
	
	///获得两个整数数组中，共有元素的个数
	pub fn get_hit_count(vec_a:&[i32], vec_b:&[i32]) -> i32 {
		let mut count = 0;
		for a in vec_a {
			for b in vec_b {
				if a == b {
					count += 1;
				}
			}
		}
		count
	}
}

pub struct NumberUtil;

impl NumberUtil {
	
	///convert the number to Vec<i32>
	pub fn to_int_array(number:&str) -> Vec<i32> {
		let split_array:Vec<&str> = number.split(|c|{
			if c == ',' || c == '|' {
				true
			} else {
				false
			}
		}).collect();
		let mut vec = Vec::new();
		for num in split_array {
			vec.push(i32::from_str(num).unwrap());
		} 
		vec
	}
	
	///检查数组是否从小到大排序
	pub fn array_sort_from_min_to_max(vec:&Vec<i32>) -> Result<i32, i32> {
		let len = vec.len(); 
		if  len >= 2 {
			let mut cur = vec.get(0).unwrap();
			for num in vec[1..len].iter() {
				if cur >= num {
					return Result::Err(-1);
				} else {
					cur = num;
				}
			}
		}
		return Result::Ok(1);
	}
	
	///检查有序数组的边界，空数组返回错误
	pub fn check_margin(vec:&Vec<i32>, min:i32, max:i32) -> Result<i32, i32> {
		let len = vec.len();
		if len == 0 {
			return Result::Err(-1);
		}
		let first = vec.first().unwrap();
		let last = vec.last().unwrap();
		if *first < min || *last > max {
			return Result::Err(-1);
		}
		return Result::Ok(1);
	}

    /**
     * 使用四舍六入五成双方法，获得整数的赔率
     */
    pub fn get_jc_rate(rate:f64) -> i64 {
        let mut rate_int = rate as i64;
        let mut rate_m = rate*10_f64;
        let rate_int_m = rate_m as i64;
        let yue_shu = rate_int_m%10;
        if yue_shu < 5 {  //4舍
            return rate_int; 
        } 
        else if yue_shu > 5 //6入
        {
            return rate_int + 1;
        } 
        else 
        {
            if rate_int%2 == 1 {  //奇数进位
                return rate_int + 1; 
            } else {
                return rate_int;
            }
            /*
            let fract = rate.fract() - 0.5_f64;
            if fract > 1e-10 {
                return rate_int + 1;
            } else {
            }
            */
        }
    }
}

pub fn json() -> Json {
    let map = BTreeMap::<String, Json>::new();
    Json::Object(map)    
}

pub fn json_from_map(map:BTreeMap<String, Json>) -> Json {
    Json::Object(map)    
}

pub fn get_data(mut rst_data:Json) -> Json {
    let mut obj = rst_data.as_object_mut().unwrap();
    obj.remove("data").unwrap()
}

pub fn get_first_data(mut rst_data:Json) -> Json {
    let mut obj = rst_data.as_object_mut().unwrap();
    let mut json = obj.remove("data").unwrap();
    let mut array = json.as_array_mut().unwrap();
    array.remove(0)
}

pub fn get_count(mut rst_data:Json) -> i64 {
    let first_data = get_first_data(rst_data);
    let node = first_data.find("count").unwrap();
    node.as_i64().unwrap()
}

