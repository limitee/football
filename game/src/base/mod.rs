extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::str::FromStr;

use std::collections::BTreeMap;

extern crate cons;
use cons::ErrCode;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Ticket {
	id: i64,
	game_id: i32,
	play_type: i32,
	bet_type: i32,
	icount: i32,		//注数
	amount: i32,		//金额
	multiple: i32,	//倍数
	number: String,
}

impl Ticket {
	
	pub fn new(id:i64, game_id:i32, play_type:i32, bet_type:i32, icount:i32, multiple:i32, amount:i32, number:&str) -> Ticket {
		Ticket {
			id: id,
			game_id: game_id,
			play_type: play_type,
			bet_type: bet_type,
			multiple: multiple,
			icount: icount,
			amount: amount,
			number: number.to_string(),
		}
	}
	
	pub fn to_json(&self) -> Json {
		let mut back = json!("{}");
		json_set!(&mut back; "id"; self.id);
		json_set!(&mut back; "game_id"; self.game_id);
		json_set!(&mut back; "play_type"; self.play_type);
		json_set!(&mut back; "bet_type"; self.bet_type);
		json_set!(&mut back; "multiple"; self.multiple);
		json_set!(&mut back; "icount"; self.icount);
		json_set!(&mut back; "amount"; self.amount);
		json_set!(&mut back; "number"; self.number);
		back
	}
	
	pub fn get_game_id(&self) -> i32 {
		self.game_id
	}
	
	pub fn get_amount(&self) -> i32 {
		self.amount
	}
	
	pub fn get_multiple(&self) -> i32 {
		self.multiple
	}
	
	pub fn get_number(&self) -> &str {
		&self.number
	}
	
	pub fn get_play_type(&self) -> i32 {
		self.play_type
	}
	
	pub fn get_bet_type(&self) -> i32 {
		self.bet_type
	}
}

///系统中的一种游戏
///
///
#[derive(RustcDecodable, RustcEncodable)]
pub struct Game {
	id:i32,
	code: String,
	name: String,
	map: BTreeMap<i32, PlayType>,
}

impl Game {
	
	///id为唯一标志,code为游戏简称，name为中文名称
	pub fn new(id:i32, code:&str, name:&str, map:BTreeMap<i32, PlayType>) -> Game {
		Game {
			id: id,
			code: code.to_string(),
			name: name.to_string(),
			map: map,
		}
	}
	
	///获得游戏id
	pub fn get_id(&self) -> i32 {
		self.id
	}
	
	///获得游戏名称
	pub fn get_name(&self) -> &str {
		&self.name
	}
	
	///获得游戏代码
	pub fn get_code(&self) -> &str {
		&self.code
	}
	
	///根据玩法id获得玩法
	pub fn get_play_type(&self, play_type_id:i32) -> Result<&PlayType, i32> {
		let op:Option<&PlayType> = self.map.get(&play_type_id);
		op.ok_or(ErrCode::PlayTypeNotExists as i32)
	}
	
}

///游戏玩法，单式，复试，胆托
#[derive(RustcDecodable, RustcEncodable)]
pub struct PlayType {
	pub id:i32,
	pub price:i32,
	pub name: String,
	pub map: BTreeMap<i32, BetType>,
}

impl PlayType {
	pub fn new(id:i32, price:i32, name:&str, map: BTreeMap<i32, BetType>) -> PlayType {
		PlayType {
			id: id,
			price: price,
			name: name.to_string(),
			map: map,
		}
	}
	
	pub fn get_price(&self) -> i32 {
		self.price
	}
	
	///根据id获得投注方式
	pub fn get_bet_type(&self, bet_type_id:i32) -> Result<&BetType, i32> {
		let op:Option<&BetType> = self.map.get(&bet_type_id);
		op.ok_or(ErrCode::BetTypeNotExists as i32)
	}
}

///投注方式，如标准
#[derive(RustcDecodable, RustcEncodable)]
pub struct BetType {
	pub id:i32,
	name: String,
}

impl BetType {
	
	pub fn new(id:i32, name:&str) -> BetType {
		BetType {
			id: id,
			name: name.to_string(),
		}
	}
}


///工厂类，存储了所有游戏的详情
pub struct GameFactory {
	game_list:BTreeMap<i32, Game>,
}

fn get_ssq_game() -> Game {
	let mut play_map = BTreeMap::new();
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(10, BetType::new(10, "单式"));
	bet_map.insert(20, BetType::new(20, "复式"));
	bet_map.insert(30, BetType::new(30, "胆托"));
	play_map.insert(10, PlayType::new(10, 200, "标准", bet_map));
	
	Game::new(100, "SSQ", "双色球", play_map)
}

fn get_dlt_game() -> Game {
	let mut play_map = BTreeMap::new();
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(10, BetType::new(10, "单式"));
	bet_map.insert(20, BetType::new(20, "复式"));
	bet_map.insert(30, BetType::new(30, "胆托"));
	play_map.insert(10, PlayType::new(10, 200, "标准", bet_map));
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(10, BetType::new(10, "单式"));
	bet_map.insert(20, BetType::new(20, "复式"));
	bet_map.insert(30, BetType::new(30, "胆托"));
	play_map.insert(60, PlayType::new(60, 300, "追加", bet_map));
	
	Game::new(200, "DLT", "大乐透", play_map)
}

fn get_jcd_game() -> Game {
	let mut play_map = BTreeMap::new();
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(1, BetType::new(1, "单关"));
	play_map.insert(1, PlayType::new(1, 200, "胜平负", bet_map));
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(1, BetType::new(1, "单关"));
	play_map.insert(2, PlayType::new(2, 200, "让球胜平负", bet_map));
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(1, BetType::new(1, "单关"));
	play_map.insert(3, PlayType::new(3, 200, "总进球数", bet_map));
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(1, BetType::new(1, "单关"));
	play_map.insert(4, PlayType::new(4, 200, "比分", bet_map));
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(1, BetType::new(1, "单关"));
	play_map.insert(5, PlayType::new(5, 200, "半全场", bet_map));
	
	Game::new(202, "JCD", "竞彩单关", play_map)
}

fn get_jcc_game() -> Game {
	let mut play_map = BTreeMap::new();
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(2, BetType::new(2, "串关"));
	play_map.insert(1, PlayType::new(1, 200, "胜平负", bet_map));
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(2, BetType::new(2, "串关"));
	play_map.insert(2, PlayType::new(2, 200, "让球胜平负", bet_map));
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(2, BetType::new(2, "串关"));
	play_map.insert(3, PlayType::new(3, 200, "总进球数", bet_map));
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(2, BetType::new(2, "串关"));
	play_map.insert(4, PlayType::new(4, 200, "比分", bet_map));
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(2, BetType::new(2, "串关"));
	play_map.insert(5, PlayType::new(5, 200, "半全场", bet_map));

	let mut bet_map = BTreeMap::new();
	bet_map.insert(2, BetType::new(2, "串关"));
	play_map.insert(10, PlayType::new(10, 200, "混投", bet_map));

	Game::new(201, "JCC", "竞彩串关", play_map)
}

fn get_jcl_game() -> Game {
	let mut play_map = BTreeMap::new();
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(1, BetType::new(1, "单关"));
	bet_map.insert(2, BetType::new(2, "串关"));
	play_map.insert(1, PlayType::new(1, 200, "胜负", bet_map));
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(1, BetType::new(1, "单关"));
	bet_map.insert(2, BetType::new(2, "串关"));
	play_map.insert(2, PlayType::new(2, 200, "让分胜负", bet_map));
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(1, BetType::new(1, "单关"));
	bet_map.insert(2, BetType::new(2, "串关"));
	play_map.insert(3, PlayType::new(3, 200, "胜分差", bet_map));
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(1, BetType::new(1, "单关"));
	bet_map.insert(2, BetType::new(2, "串关"));
	play_map.insert(4, PlayType::new(4, 200, "大小分", bet_map));
	

	let mut bet_map = BTreeMap::new();
	bet_map.insert(2, BetType::new(2, "串关"));
	play_map.insert(10, PlayType::new(10, 200, "混投", bet_map));

	Game::new(301, "JCL", "竞彩篮球", play_map)
}


fn get_pls_game() -> Game {
    let mut play_map = BTreeMap::new();
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(1, BetType::new(1, "单式"));
	bet_map.insert(2, BetType::new(2, "复式"));
	bet_map.insert(3, BetType::new(4, "和值"));
	play_map.insert(1, PlayType::new(1, 200, "直选", bet_map));
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(1, BetType::new(1, "单式"));
	bet_map.insert(2, BetType::new(2, "复式"));
	play_map.insert(2, PlayType::new(2, 200, "组三", bet_map));

	let mut bet_map = BTreeMap::new();
	bet_map.insert(1, BetType::new(1, "单式"));
	bet_map.insert(2, BetType::new(2, "复式"));
	play_map.insert(3, PlayType::new(3, 200, "组六", bet_map));

	Game::new(203, "PLS", "排列三", play_map)
}

impl GameFactory {
	
	pub fn new() -> GameFactory {
		let mut map = BTreeMap::new();
		
		let game = get_ssq_game();
		map.insert(game.get_id(), game);
		
		let game = get_dlt_game();
		map.insert(game.get_id(), game);
		
		let game = get_jcd_game();
		map.insert(game.get_id(), game);

		let game = get_jcc_game();
		map.insert(game.get_id(), game);

        let game = get_pls_game();
		map.insert(game.get_id(), game);

        let game = get_jcl_game();
		map.insert(game.get_id(), game);

		GameFactory {
			game_list:map,
		}
    }
	
	///根据id获得&Game
	pub fn get_game_by_id(&self, id:i32) -> Result<&Game, i32> {
		let op:Option<&Game> = self.game_list.get(&id);
		op.ok_or(ErrCode::GameNotExists as i32)
	}
	
	pub fn get_game_list(&self) -> &BTreeMap<i32, Game> {
		&self.game_list
	}
}

pub type ValidateResult = Result<(i32), i32>;

pub trait Validate: Send + Sync {
	
	fn validate(&self, ticket: &Json, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult;
}

pub type DrawResult = Result<Json, i32>;

pub trait Draw: Send + Sync {
	
	fn draw(&self, ticket: &Json, draw_number:&Vec<i32>, gl:&BTreeMap<i64, Json>, game:&Game, play_type:&PlayType, bet_type:&BetType) -> DrawResult;
}

pub trait Print: Send + Sync {
    ///获得出票返回的赔率信息
    fn get_print_number(&self, number:&str, stub:&str) -> String;
}

lazy_static! {
    pub static ref GF:GameFactory = GameFactory::new();
}
