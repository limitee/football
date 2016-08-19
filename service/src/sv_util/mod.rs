extern crate util;
use self::util::DigestUtil;

extern crate dc;
use self::dc::DataBase;
use self::dc::MyDbPool;

extern crate cons;
use self::cons::CONS;
use self::cons::ErrCode;

extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;
use std::str::FromStr;

pub struct KeyHelper;

extern crate time;

mod get_dn;
pub use self::get_dn::get_dlt_draw_info;

pub mod get_jcc_draw_info;
pub use self::get_jcc_draw_info::get_jcc_draw_info;

impl KeyHelper {

    /**
     * get key from cache.
     */
    pub fn from_cache(db:&DataBase<MyDbPool>, head:&Json) -> Result<String, i32> {
        let user_type = json_str!(head;"userType");
        if user_type == "center" {
            return KeyHelper::from_db_by_id(db, head);
        }
        let user_id = json_i64!(head; "userId");
        let st_table = db.get_table("st").expect("st table not exists.");
        let cond = format!(r#"
            {{
                "id":{}
            }}
        "#, user_id);
        let rst = st_table.find_one_by_str(&cond, "{}", "{}");
        let rst = rst.or(Result::Err(ErrCode::UserInfoIsWrong as i32));
        
        let back_json = try!(rst);
        match user_type {
    		"terminal" => {
    			let fix_st = json_str!(&back_json; "fix_st");
    			Result::Ok(fix_st.to_string())
    		},
    		"company" => {
    			let fix_st = json_str!(&back_json; "fix_st");
    			Result::Ok(fix_st.to_string())
    		},
    		_ => {
    			let last_active_time = json_i64!(&back_json; "last_active_time");
		        let sec = time::get_time().sec;
		        if sec - last_active_time > 120000 {
		            Result::Err(ErrCode::TokenExpired as i32)
		        }
		        else
		        {
		            let st = json_str!(&back_json; "st");
		            Result::Ok(st.to_string())
		        }
        	},
        }
    }

    pub fn active(db:&DataBase<MyDbPool>, head:&Json) {
        let user_id = json_i64!(head; "userId");
        let cond = format!(r#"
            {{
                "id":{}
            }}
        "#, user_id);
        let sec = time::get_time().sec;
        let doc = format!(r#"
        {{
            "$set":
                {{
                    "last_active_time":{}
                }}
            }}
        "#, sec);
        let st_table = db.get_table("st").expect("st table not exists.");
        let _ = st_table.update_by_str(&cond, &doc, "{}");
    }

    pub fn from_db(db:&DataBase<MyDbPool>, head:&Json) -> Result<String, i32> {
        let username = json_str!(head; "userId");
        let user_type = json_str!(head; "userType");
		
        let rst = CONS.code_to_id("user_type", &user_type);
        rst.and_then(|user_type_id| {
            let table = db.get_table("customer").expect("table not exists.");
            let cond = format!(r#"
               {{
                   "username":"{}",
                   "type":{}
               }}
            "#, username, user_type_id);
            let fd_back = try!(table.find_by_str(&cond, "{}", "{}"));
            info!("{}", fd_back);
            let rows = json_i64!(&fd_back; "rows");
            if rows > 0 {   //return the db object to client
                let password = json_str!(&fd_back; "data", "0", "password");
                let digest = DigestUtil::md5(password);
                return Result::Ok(digest)
            }
            else
            {
               return Result::Err(ErrCode::UserInfoIsWrong as i32);
            }
        })
    }
    
    pub fn from_db_by_id(db:&DataBase<MyDbPool>, head:&Json) -> Result<String, i32> {
        let user_id = json_i64!(head; "userId");
        let table = db.get_table("customer").expect("table not exists.");
        let cond = format!(r#"
           {{
               "id":"{}"
           }}
        "#, user_id);
        let fd_back = try!(table.find_by_str(&cond, "{}", "{}"));
        let rows = json_i64!(&fd_back; "rows");
        if rows > 0 {   //return the db object to client
            let password = json_str!(&fd_back; "data", "0", "password");
            let digest = DigestUtil::md5(password);
            return Result::Ok(digest)
        }
        else
        {
           return Result::Err(ErrCode::UserInfoIsWrong as i32);
        }
    }

}

pub struct PageHelper;

impl PageHelper {

    pub fn get_in_data(data:&Json, col:&str) -> Json {
        let mut in_data = json!("{}");
        {
            let mut id_array = Vec::new();
            let list = json_path!(data; "data");
            let array = list.as_array().unwrap();
            for terminal in array {
                let id = json_i64!(terminal; col);
                id_array.push(id.to_json());
            }
            json_set!(&mut in_data; "$in"; id_array);
        }
        in_data
    }

    pub fn get_cond(body:&Json) -> Json {
        let node_op = body.find("cond");
        match node_op {
            Some(node) => {
                let cond = json!(node.as_string().unwrap());
                cond
            },
            None => {
                json!("{}")
            }
        }
    }

    pub fn get_op(body:&Json) -> Json {
        let mut op = json!("{}");
        let sort_node_op = body.find("sort");
        if let Some(sort_node) = sort_node_op {
            let sort_str = sort_node.as_string().unwrap();
            let sort = json!(sort_str);
            json_set!(&mut op; "sort"; sort);
        }
        
        let limit_node_op = body.find("limit");
        if let Some(limit_node) = limit_node_op {
            json_set!(&mut op; "limit"; limit_node);
        }

        let offset_node_op = body.find("offset");
        if let Some(offset_node) = offset_node_op {
            json_set!(&mut op; "offset"; offset_node);
        }

        info!("{}", op);
        op
    }
}


