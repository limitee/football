#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate easy_util;

use std::collections::BTreeMap;
use std::sync::{Arc};

extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;

pub enum ErrCode {
    Success = 0,

    DigestFailure,
    ApiNotExits,
    TokenExpired,
    ValueNotExist,	//值不存在
    NotAllowed,

    UsernameIsNull,
    UsernameWrongPattern,
    UsernameIsWrong,

    PasswordIsNull,
    PasswordWrongPattern,

    UserInfoIsWrong,
	
	KeyIsMissing, 	//请求的字段不全
	
	GameNotExists,	//游戏不存在
	TermStatusNotAllowed, //期次状态错误
	AmountIsWrong,	//金额错误
	PlayTypeNotExists,	//玩法不存在
	BetTypeNotExists,	//投注方式不存在
	NumberIsWrong,	//号码格式错误
	CountBtFive,	//单式不能超过5注
	
	BalanceNotAllowed, 	//账户金额不允许
    DataExpired, //数据已经失效
    OutIdRepeat, //外部id重复

	DrawNumberIsWrong,	//开奖号码格式错误

    NoDbConn, //无连接可用
}

fn get_err_map() -> BTreeMap<i32, String> {
    let mut err_map = BTreeMap::new();
    err_map.insert(ErrCode::Success as i32, "操作成功".to_string()); 
    err_map.insert(ErrCode::DigestFailure as i32, "加密检验失败".to_string()); 
    err_map.insert(ErrCode::ApiNotExits as i32, "接口不存在".to_string()); 
    err_map.insert(ErrCode::TokenExpired as i32, "密钥已过期".to_string()); 
    err_map.insert(ErrCode::ValueNotExist as i32, "值不存在".to_string()); 
    err_map.insert(ErrCode::NotAllowed as i32, "操作不允许".to_string()); 
    err_map.insert(ErrCode::UsernameIsNull as i32, "用户名为空".to_string()); 
    err_map.insert(ErrCode::UsernameWrongPattern as i32, "用户名格式错误".to_string()); 
    err_map.insert(ErrCode::PasswordIsNull as i32, "密码为空".to_string()); 
    err_map.insert(ErrCode::PasswordWrongPattern as i32, "密码格式错误".to_string()); 
    err_map.insert(ErrCode::UserInfoIsWrong as i32, "用户信息错误".to_string()); 
    err_map.insert(ErrCode::KeyIsMissing as i32, "缺少必要字段".to_string()); 
    err_map.insert(ErrCode::GameNotExists as i32, "游戏不存在".to_string()); 
    err_map.insert(ErrCode::TermStatusNotAllowed as i32, "期次状态错误".to_string()); 
    err_map.insert(ErrCode::AmountIsWrong as i32, "金额错误".to_string()); 
    err_map.insert(ErrCode::PlayTypeNotExists as i32, "玩法不存在".to_string()); 
    err_map.insert(ErrCode::BetTypeNotExists as i32, "投注方式不存在".to_string()); 
    err_map.insert(ErrCode::NumberIsWrong as i32, "号码格式错误".to_string()); 
    err_map.insert(ErrCode::CountBtFive as i32, "单式不能超过5注".to_string()); 
    err_map.insert(ErrCode::BalanceNotAllowed as i32, "账户金额错误".to_string()); 
    err_map.insert(ErrCode::DataExpired as i32, "数据已经失效".to_string()); 
    err_map.insert(ErrCode::OutIdRepeat as i32, "外部id重复".to_string()); 
    err_map.insert(ErrCode::DrawNumberIsWrong as i32, "开奖号码格式错误".to_string()); 
    err_map.insert(ErrCode::NoDbConn as i32, "无数据库连接可用".to_string()); 
    err_map
}

pub struct ConsNode {
    id:i32,
    code:String,
    desc:String,
}

impl ConsNode {
	
	pub fn new(id:i32, code:&str, desc:&str) -> ConsNode {
		ConsNode {
			id:id,
			code:code.to_string(),
			desc:desc.to_string(),
		}
	}
	
}

// Specify encoding method manually
impl ToJson for ConsNode {
	
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        // All standard types implement `to_json()`, so use it
        d.insert("id".to_string(), self.id.to_json());
        d.insert("code".to_string(), self.code.to_json());
        d.insert("desc".to_string(), self.desc.to_json());
        Json::Object(d)
    }
    
}

pub struct Cons {
    code_data:BTreeMap<String, Arc<ConsNode>>,
    id_data:BTreeMap<i32, Arc<ConsNode>>,
}

// Specify encoding method manually
impl ToJson for Cons {
	
    fn to_json(&self) -> Json {
   		let mut d = BTreeMap::new();
   		for (key, value) in &self.id_data {	
   			d.insert(key.to_string(), value.to_json());
   		}
   		Json::Object(d)
    }
}


impl Cons {

    pub fn from_vec(vec:Vec<Arc<ConsNode>>) -> Cons {
        let mut code_data:BTreeMap<String, Arc<ConsNode>> = BTreeMap::new();
        let mut id_data:BTreeMap<i32, Arc<ConsNode>> = BTreeMap::new();
        for node in vec {
            code_data.insert(node.code.clone(), node.clone());
            id_data.insert(node.id, node);
        }
        Cons {
            code_data: code_data,
            id_data: id_data,
        }
    }

}

pub struct ConsFactory {
    cons: BTreeMap<String, Cons>,
    err_map: BTreeMap<i32, String>,
}

macro_rules! get_node {
    ($o:expr, $k:expr, $v:expr) => {{
        Arc::new(ConsNode::new($o, $k, $v))
    }}
}


impl ConsFactory {

    pub fn new() -> ConsFactory {
        let mut cons:BTreeMap<String, Cons> = BTreeMap::new();
        let user_type_vec = vec![
            get_node!(100, "guest", "游客"),
            get_node!(200, "normal", "普通用户"),
            get_node!(300, "company", "公司"),
            get_node!(400, "center", "出票中心"),
            get_node!(500, "terminal", "终端机"),
            get_node!(900, "admin", "管理员"),
        ];
        cons.insert("user_type".to_string(), Cons::from_vec(user_type_vec));
        let file_type_vec = vec![
            get_node!(-1, "unknown", "未知"),
            get_node!(100, "text/xml", "xml"),
            get_node!(200, "text/plain", "txt"),
            get_node!(300, "image/png", "png"),
        ];
        cons.insert("file_type".to_string(), Cons::from_vec(file_type_vec));

        let desk_status_vec = vec![
            get_node!(0, "idle", "空闲"),
            get_node!(100, "eat", "就餐中"),
        ];
        cons.insert("desk_status".to_string(), Cons::from_vec(desk_status_vec));
        
        let moneylog_type_vec = vec![
            get_node!(1000, "charge", "充值"),
            get_node!(1100, "bet", "投注"),
            get_node!(1200, "print", "出票"),
            get_node!(1300, "refund", "退款"),
            get_node!(1400, "fund", "返奖"),
            get_node!(1500, "cash", "兑奖"),
            get_node!(1600, "reset", "矫正"),
            get_node!(1700, "repay", "返佣"),
        ];
        cons.insert("moneylog_type".to_string(), Cons::from_vec(moneylog_type_vec));
        
        let term_status_vec = vec![
            get_node!(10, "init", "初始状态"),
            get_node!(20, "ready_to_sale", "销售准备中"),
            get_node!(30, "sale", "销售中"),
            get_node!(35, "pause", "暂停销售"),
            get_node!(40, "ready_to_end", "准备停售中"),
            get_node!(50, "end", "已停售"),
            get_node!(55, "delayed", "延迟"),
            get_node!(60, "open", "已开奖"),
            get_node!(70, "drawing", "算奖中"),
            get_node!(80, "drawed", "已算奖"),
            get_node!(90, "funding", "返奖中"),
            get_node!(100, "funded", "已返奖"),
        ];
        cons.insert("term_status".to_string(), Cons::from_vec(term_status_vec));
        
        let ticket_status_vec = vec![
            get_node!(10, "printing", "出票中"),
            get_node!(15, "print_err", "出票异常"),
            get_node!(20, "print_success", "出票成功"),
            get_node!(30, "hit", "中奖"),
            get_node!(40, "not_hit", "未中奖"),
            get_node!(50, "refund", "已退款"),
            get_node!(52, "bonus_super", "超级大奖"),
            get_node!(55, "bonus_big", "中大奖"),
            get_node!(58, "bonus_big_cashed", "大奖已兑"),
            get_node!(60, "funded", "已返奖"),
            get_node!(65, "bonus_err", "兑奖异常"),
            get_node!(70, "cashed", "已兑奖"),
        ];
        cons.insert("ticket_status".to_string(), Cons::from_vec(ticket_status_vec));

        let sys_mode_vec = vec![
            get_node!(0, "print", "出票"),
            get_node!(1, "bonus", "兑奖"),
        ];
        cons.insert("sys_mode".to_string(), Cons::from_vec(sys_mode_vec));

        let terminal_status_vec = vec![
            get_node!(0, "offline", "离线"),
            get_node!(1, "online", "在线"),
        ];
        cons.insert("terminal_status".to_string(), Cons::from_vec(terminal_status_vec));
        
        let terminal_type_vec = vec![
            get_node!(0, "normal", "普通机"),
            get_node!(1, "manager", "管理机"),
        ];
        cons.insert("terminal_type".to_string(), Cons::from_vec(terminal_type_vec));

        let terminal_hard_type_vec = vec![
            get_node!(1, "C8", "英特达"),
        ];
        cons.insert("terminal_hard_type".to_string(), Cons::from_vec(terminal_hard_type_vec));

        let province_type_vec = vec![
            get_node!(-1, "unset", "未设置"),
            get_node!(1, "hb", "湖北"),
            get_node!(2, "he", "河北"),
            get_node!(3, "sx", "陕西"),
            get_node!(4, "ah", "安徽"),
            get_node!(5, "hn", "湖南"),
        ];
        cons.insert("province_type".to_string(), Cons::from_vec(province_type_vec));


        let terminal_soft_type_vec = vec![
            get_node!(1, "TC001", "35-36"),
        ];
        cons.insert("terminal_soft_type".to_string(), Cons::from_vec(terminal_soft_type_vec));

        let terminal_mode_vec = vec![
            get_node!(100, "normal", "普通"),
            get_node!(200, "print", "只出票"),
            get_node!(300, "bonus", "只兑奖"),
            get_node!(900, "disabled", "停用"),
        ];
        cons.insert("terminal_mode".to_string(), Cons::from_vec(terminal_mode_vec));

        let charge_report_status_vec = vec![
            get_node!(10, "unhandle", "未处理"),
            get_node!(20, "handled", "已处理"),
        ];
        cons.insert("charge_report_status".to_string(), Cons::from_vec(charge_report_status_vec));

        let err_map = get_err_map();

        ConsFactory{
            cons:cons,
            err_map:err_map,
        }
    }
    
    /**
     * 获得对象模式的约束条件
     */
    pub fn get_json_obj(&self, name:&str) -> Json {
        let cons:&Cons = self.cons.get(name).unwrap();
        cons.to_json()
    }

    pub fn by_id(&self, name:&str, id:i32) -> Option<&Arc<ConsNode>> {
        let cons:&Cons = self.cons.get(name).unwrap();
        cons.id_data.get(&id)
    }

    pub fn code_to_id(&self, name:&str, code:&str) -> Result<i32, i32> {
        //println!("the name is {}.", name);
        //println!("the code is {}.", code);
        let op = self.by_code(name, code);
        match op {
            Some(x) => {
                Result::Ok((**x).id)
            },
            None => {
                Result::Err(ErrCode::ValueNotExist as i32)
            },
        }
    }

    pub fn by_code(&self, name:&str, code:&str) -> Option<&Arc<ConsNode>> {
        let cons:&Cons = self.cons.get(name).unwrap();
        cons.code_data.get(code)
    }

    pub fn id_to_code(&self, name:&str, id:i32) -> Result<String, i32> {
        let op = self.by_id(name, id);
        match op {
            Some(x) => {
                Result::Ok((**x).code.clone())
            },
            None => {
                Result::Err(ErrCode::ValueNotExist as i32)
            },
        }
    }
    
    pub fn id_to_desc(&self, name:&str, id:i32) -> Result<&str, i32> {
        let op = self.by_id(name, id);
        match op {
            Some(x) => {
                Result::Ok((**x).desc.as_str())
            },
            None => {
                Result::Err(ErrCode::ValueNotExist as i32)
            },
        }
    }
	
	pub fn code_to_desc(&self, name:&str, code:&str) -> Result<&str, i32> {
        let op = self.by_code(name, code);
        match op {
            Some(x) => {
                Result::Ok((**x).desc.as_str())
            },
            None => {
                Result::Err(ErrCode::ValueNotExist as i32)
            },
        }
    }

    pub fn get_err_des(&self, code:i32) -> Option<&String> {
        self.err_map.get(&code)
    }
    
    pub fn get_err(&self, code:i32) -> Json {
        let mut json = json!("{}"); 
        json_set!(&mut json; "code"; code);
        let des = self.get_err_des(code);
        if let Some(des_str) = des {
            json_set!(&mut json; "des"; des_str);
        } else {
            json_set!(&mut json; "des"; "未知错误");
        }
        json
    }
}


pub struct GameLevelFactory {
	gv_map: BTreeMap<i32, Vec<Json>>,	//游戏奖级模版
}

fn get_level(lev:i32, bonus:i64, bonus_after_tax:i64, descrip:&str) -> Json {
	let mut gl = json!("{}");
	json_set!(&mut gl; "lev"; lev);
	json_set!(&mut gl; "bonus"; bonus);
	json_set!(&mut gl; "bonus_after_tax"; bonus_after_tax);
	json_set!(&mut gl; "descrip"; descrip);
	gl
}

impl GameLevelFactory {
	
	pub fn new() -> GameLevelFactory {
		let mut dlt = Vec::<Json>::new();
		dlt.push(get_level(1, 0_i64, 0_i64, "一等奖"));
		dlt.push(get_level(2, 0_i64, 0_i64, "二等奖"));
		dlt.push(get_level(3, 0_i64, 0_i64, "三等奖"));
		dlt.push(get_level(4, 20000_i64, 20000_i64, "四等奖"));
		dlt.push(get_level(5, 1000_i64, 1000_i64, "五等奖"));
		dlt.push(get_level(6, 500_i64, 500_i64, "六等奖"));
		
		dlt.push(get_level(7, 0_i64, 0_i64, "一等奖追加"));
		dlt.push(get_level(8, 0_i64, 0_i64, "二等奖追加"));
		dlt.push(get_level(9, 0_i64, 0_i64, "三等奖追加"));
		dlt.push(get_level(10, 10000_i64, 10000_i64, "四等奖追加"));
		dlt.push(get_level(11, 500_i64, 500_i64, "五等奖追加"));
		dlt.push(get_level(12, 0_i64, 0_i64, "六等奖派奖"));
		
		let mut gv_map = BTreeMap::new();
		gv_map.insert(200, dlt);
		GameLevelFactory {
			gv_map: gv_map,
		}
	}
	
	pub fn get_by_game_id(&self, game_id:i32) -> Option<&Vec<Json>> {
		self.gv_map.get(&game_id)
	}
}

lazy_static! {
    pub static ref CONS:ConsFactory = ConsFactory::new();
    pub static ref GLF:GameLevelFactory = GameLevelFactory::new();
}


