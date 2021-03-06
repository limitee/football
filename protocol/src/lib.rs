#[macro_use]
extern crate log;
extern crate elog;

extern crate byteorder;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

use std::net::TcpStream;
use std::io::prelude::*;
use std::io::Error;
use std::io::Cursor;

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;
use std::str::FromStr;

extern crate util;
use self::util::DigestUtil;

extern crate chrono;
use chrono::*;

pub struct ProtocolHelper;

impl ProtocolHelper {
	
	pub fn get_msg_head(user_id:&str, cmd: &str, key: &str, body: &str) -> String {
		let mut head = json!("{}");
		json_set!(&mut head; "cmd"; cmd);
		json_set!(&mut head; "userId"; user_id);
		json_set!(&mut head; "userType"; "terminal");
		let time_stamp = Local::now().to_string();
		json_set!(&mut head; "timeStamp"; time_stamp);
	    let digest_content = format!("{}{}{}", key, body, time_stamp);
	    let digest = DigestUtil::md5(&digest_content);
	    json_set!(&mut head; "digestType"; "md5");
	    json_set!(&mut head; "digest"; digest);
	    
	    head.to_string()
	}
	
}

pub struct Protocol {
	stream:TcpStream,
	rec_buffer: Vec<u8>, 
	key: String,
}

impl Protocol {
	
	pub fn new(stream:TcpStream, key:String) -> Protocol {
		Protocol {
			stream:stream,
			rec_buffer: Vec::<u8>::new(),
			key: key,
		}
	}
	
	/**
	 * get msg from the outside, -2，暂时无数据，－1,出现错误
	 */		
	pub fn rec_msg(&mut self) -> Result<(String, String), i32> {
		let mut buf = vec![0u8; 1024];	//收取数据的缓存大小
		loop {
			let rst = self.stream.read(&mut buf);
			match rst {
				Ok(size) => {
   					debug!("size:{}", size);
   					if size == 0 {
                        return Result::Err(-1);
   					}
   					self.rec_buffer.extend_from_slice(&buf[0..size]);
					let mut cur_buffer_length = self.rec_buffer.len() as i32;
					let mut target_buffer_length = 0;	//当前数据块的长度
					if cur_buffer_length >= 4 {
						let mut rdr = Cursor::new(&self.rec_buffer[0..4]);
						target_buffer_length = rdr.read_i32::<BigEndian>().unwrap();
					}
					debug!("tbl:{}", target_buffer_length);
    				debug!("cbl:{}", cur_buffer_length);
    				
    					//有消息需要处理
					if cur_buffer_length >= target_buffer_length + 4 && target_buffer_length > 0 {
						let vec2 = self.rec_buffer.split_off((target_buffer_length + 4) as usize);
						let (head_str, body_str) = {
							let mut rdr = Cursor::new(&self.rec_buffer[4..8]);
							//消息头的长度
							let head_length = rdr.read_i32::<BigEndian>().unwrap();
							let head_end = (head_length + 8) as usize;
							let head_str = String::from_utf8_lossy(&self.rec_buffer[8..head_end]).into_owned();
							
							//消息体的长度
							let body_length = (target_buffer_length - head_length - 4) as usize;
							let body_end = head_end + body_length; 
							let body_str = String::from_utf8_lossy(&self.rec_buffer[head_end..body_end]).into_owned();
							(head_str, body_str)
						};
						self.rec_buffer = vec2;
						return Result::Ok((head_str, body_str));
					}
	    		},
	    		Err(err) => {
                    if err.kind() == std::io::ErrorKind::WouldBlock {
                        return Result::Err(-2);
                    } else {
	    			    error!("err:{:?}", err.kind());
		                return Result::Err(-1);
                    }
	    		},
	    	}
		}
		Result::Err(-1)
	}

	pub fn rec_msg_block(&mut self, out_count:u64) -> Result<(String, String), i32> {
        let mut rst = Err(-1);
        let mut count = 1_u64;
        loop {
            let non_block_rst = self.rec_msg();
            if let Err(flag) = non_block_rst {
                if flag == -2 {
                    if count >= out_count {
                        rst = Err(-1); 
                        break;
                    } else {
                        count += 1;
        	            std::thread::sleep(std::time::Duration::from_millis(200));
                        continue;
                    }
                } else {
                    rst = Err(flag); 
                    break;
                }
            } else {
                rst = non_block_rst;
                break;
            }
        }
        rst
    }
	
	pub fn send(&mut self, head:String, body:String) -> Result<(), Error> {
		let mut body_bytes = body.into_bytes();
		let body_len = body_bytes.len();
		let mut head_bytes = head.into_bytes();
		let head_len = head_bytes.len();
		let len = head_len + body_len + 4;
		
		let mut wtr = vec![];
		wtr.write_i32::<BigEndian>(len as i32).unwrap();
		wtr.write_i32::<BigEndian>(head_len as i32).unwrap();
		
		wtr.append(&mut head_bytes);
		wtr.append(&mut body_bytes);
		
		let rst = self.stream.write_all(&wtr);
		rst
	}
	
	pub fn send_body(&mut self, user_id:&str, cmd:&str, body:&Json) -> Result<(), Error> {
		let body_str = body.to_string();
	    let head_str = ProtocolHelper::get_msg_head(user_id, cmd, &self.key, &body_str);
		self.send(head_str, body_str)
	}
}
