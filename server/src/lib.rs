use std::thread;
use std::net::{TcpListener, TcpStream};
use std::sync::MutexGuard;

use std::io::Read;
use std::io::Cursor;

#[macro_use]
extern crate log;
extern crate elog;

extern crate chrono;
use chrono::*;

extern crate protocol;
use protocol::Protocol;
use protocol::ProtocolHelper;

extern crate cons;
use cons::CONS;

extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;
use std::str::FromStr;

extern crate service;
use service::ApiFactory;

extern crate dc;
use dc::MyDbPool;
use dc::DataBase;

use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;

use std::sync::{Arc, Mutex};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

extern crate game;
use game::base::Ticket;

extern crate sev_helper;
use sev_helper::CacheService;
use sev_helper::PrintService;
use sev_helper::TicketService;
use sev_helper::QueryService;
use sev_helper::TerminalService;

extern crate rand;
use rand::distributions::{IndependentSample, Range};

pub type Msg = Json;
pub type ManMsg = Json;
pub type IndexMap = Arc<Mutex<BTreeMap<i32, Sender<(String, String)>>>>;
pub type UserMap = Arc<Mutex<BTreeMap<String, Client>>>;
pub type ClientRx = Arc<Mutex<Receiver<(String, String)>>>;
pub type ManRx = Arc<Mutex<Receiver<ManMsg>>>;

pub struct Client {
	sender: Sender<(String, String)>,
    //客户端的状态，-1,no balance,0，idle，1，出票中，2，兑奖中，3，收集心跳信息中，4，查询中
	status: i32,	
	cur_msg: String, 	//缓存当前出票，或者兑奖的信息，客户端断开连接时，重新进入队列
	games: HashSet<i32>,	//客户端能出的游戏列表，登录时，从数据库读取
    user_id: String,  //终端机名称
    customer_id: i64, //终端机id
    cur_type: i64,    //终端类型，0，普通，1，管理
    print_gap: i64,   //终端出票间隔
    cur_gap: i64,     //当前终端出票间隔
    msg_tick: i64,    //发送消息之后的记时器，超时断开连接
    mode: i64,        //模式，100,正常，200,只出票，300,只兑奖，900,停用
    help_bonus_id: i64, //帮助目标终端兑奖, -1，不帮助任何终端
    guarantee_amount: i64, //保证金额度
}

/**
 * 客户端连接的详细信息，0是空闲状态，1是忙的状态
 */
impl Client {
	
	pub fn new(user_id:&str, customer_id:i64, sender: Sender<(String, String)>) -> Client {
		Client {
			sender: sender,
			status: 0,
			cur_msg: String::new(),
			games: HashSet::new(),
            user_id: user_id.to_string(),
            customer_id: customer_id,
            cur_type: 0,
            print_gap: 6,
            cur_gap: 0,
            msg_tick: 0,
            mode: 900,
            help_bonus_id: -1,
            guarantee_amount: 100000,
		}
	}
	
	pub fn send(&self, cmd: String, body:String) -> Result<i32, i32> {
		let rst = self.sender.send((cmd, body));
        let rst = rst.or_else(|err|{
            error!("manager send msg to client, error.........");
            error!("{}", err);
            Result::Err(-1)
        });
        rst.and_then(|_|{
            Result::Ok(1) 
        })
	}
	
	pub fn get_status(&self) -> i32 {
		self.status
	}
	
	pub fn set_status(&mut self, status:i32) {
		self.status = status;
	}
	
	///客户端是否处于空闲状态
	pub fn is_idle(&self) -> bool {
		if self.status == 0 {
			true
		} else {
			false
		}
	}

    //是否在等待客户端回复
    pub fn is_waiting_response(&self) -> bool {
        if self.status == 1 || self.status == 2 || self.status == 4 {
            true
        } else {
            false
        }
    }

    //检查终端机的余额是否足够
    pub fn has_enough_balance(&mut self, amount:i64, db:&DataBase<MyDbPool>) -> bool {
        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; self.customer_id);

        let doc = json!("{}");
        let op = json!("{}");

        let table = db.get_table("account").unwrap();
        let acc_rst = table.find_one(&cond, &doc, &op);
        if acc_rst.is_err() {
            return false;
        }
        let account = acc_rst.unwrap();
        let cur_amount = json_i64!(&account; "client_balance");
        if cur_amount < amount + self.guarantee_amount {
            self.cur_gap = 0;    //重新计算权重
            if self.guarantee_amount == 0 {  //不需要保证金，则消耗所有的金额
                if cur_amount < 200 {
                    self.status = -1;
                }
            } else {
                self.status = -1;
            }
            return false;
        }
        return true;
    }
}

pub struct ClientHandler {
	proto: Protocol,
	man_sx: Sender<ManMsg>,
	client_rx: ClientRx,
	index: i32,
	api: Arc<ApiFactory>,
	db: Arc<DataBase<MyDbPool>>,
	user_id: String,
	customer_id: i64,
	key: String,
}

impl ClientHandler {
	
	pub fn new(proto:Protocol, man_sx:Sender<ManMsg>, client_rx:ClientRx, index:i32, api: Arc<ApiFactory>, db: Arc<DataBase<MyDbPool>>,) -> ClientHandler {
		ClientHandler {
			proto: proto,
			man_sx: man_sx,
			client_rx: client_rx,
			index: index,
			api: api,
			db: db,
			user_id: String::from(""),
			customer_id: -1_i64,
			key: String::from(""),
		}
	}
	
	/**
	 * 客户端登录信息，1成功，0失败
	 */
	pub fn login_info(&self, status:i32) -> Result<i32, i32> {
		let mut man_info = json!("{}");
		json_set!(&mut man_info; "cmd"; 10);
		json_set!(&mut man_info; "userId"; self.user_id);
		json_set!(&mut man_info; "customer_id"; self.customer_id);
		json_set!(&mut man_info; "index"; self.index);
		json_set!(&mut man_info; "status"; status);
		let rst = self.man_sx.send(man_info);
		let rst = rst.and(Result::Ok(1)).or_else(|err|{
			error!("{}", err);
			Result::Err(-1)
		});
		rst
	}
	
	/**
	 * 客户端断开连接信息
	 */
	pub fn disconnect_info(&self) -> Result<i32, i32> {
		let mut man_info = json!("{}");
		json_set!(&mut man_info; "cmd"; 20);
		json_set!(&mut man_info; "index"; self.index);
		json_set!(&mut man_info; "userId"; self.user_id);
		let rst = self.man_sx.send(man_info);
		let rst = rst.and(Result::Ok(1)).or_else(|err|{
			error!("{}", err);
			Result::Err(-1)
		});
		rst
	}
	
	/**
	 * 客户端状态变化信息
	 */
	pub fn status_info(&self, status:i32) -> Result<i32, i32> {
		let mut man_info = json!("{}");
		json_set!(&mut man_info; "cmd"; 30);
		json_set!(&mut man_info; "index"; self.index);
		json_set!(&mut man_info; "userId"; self.user_id);
		json_set!(&mut man_info; "status"; status);
		let rst = self.man_sx.send(man_info);
		let rst = rst.and(Result::Ok(1)).or_else(|err|{
			error!("{}", err);
			Result::Err(-1)
		});
		rst
	}
	
	/**
	 * 开始消息循环
	 */
	pub fn start_event_loop(&mut self) -> Result<i32, i32> {
		loop {
			let client_rx = self.client_rx.lock().unwrap();
			let rst = client_rx.try_recv();

            if let Err(err) = rst {
                match err {
                    TryRecvError::Empty => {

                    },
                    TryRecvError::Disconnected => {
                        error!("请求超时:{}", self.user_id);
				        self.disconnect_info();
                        return Result::Ok(1);
                    },
                }
            }

            if let Ok((cmd, body)) = rst {
				let head_string = ProtocolHelper::get_msg_head(&self.user_id, &cmd, &self.key, &body);
	    		let rst = self.proto.send(head_string, body);
                if rst.is_err() {
                    error!("发送消息失败:{}", self.user_id);
				    self.disconnect_info();
                    return Result::Ok(1);
                }
            }

			let rst = self.proto.rec_msg();
            //-1才需要断开连接
            if let Err(flag) = rst {
                if flag == -1 {
				    self.disconnect_info();
				    break;
                } else if flag == -2 {
        	        thread::sleep(std::time::Duration::from_millis(200));
                }
            }

			let rst = rst.and_then(|(head, body)|{
				let head_json = json!(&head);
                let back_cmd = json_str!(&head_json; "cmd");
                if back_cmd != "T10" {
				    info!("head:{}", head);
				    info!("body:{}", body);
				    let _ = self.status_info(0);
                }
				let body_json = json!(&body);
				self.handle_back(&head_json, &body_json)
			});

        }
		Result::Ok(1)
	}
	
	///处理回执
	pub fn handle_back(&self, head:&Json, body:&Json) -> Result<i32, i32> {
		let cmd = json_str!(head; "cmd");
		match cmd {
			"T02" => {
				PrintService.back(self.customer_id, body, &self.db);
			},
            "T03" => {
				TicketService.cash(self.customer_id, body, &self.db);
			},
            "T04" => {
				QueryService.term_info(self.customer_id, body, &self.db);
			},
            "T05" => {  //缴款报表
				QueryService.charge_info(self.customer_id, body, &self.db);
			},
			"T10" => {
				//info!("{} is online.", self.user_id);
			},
			_ => {
				info!("not surpported cmd {}.", cmd);
			}
		}
		Result::Ok(1)
	}
	
	/**
	 * 处理和client交互
	 */
	pub fn start(&mut self) {
		// 第一步，等待客户端登录
        let rst = self.proto.rec_msg_block(100);
        let rst = rst.and_then(|(head_str, body_str)|{
			info!("head:{}", head_str);
			info!("body:{}", body_str);
			let mut req_map = BTreeMap::new();
	        req_map.insert(String::from("head"), head_str);
	        req_map.insert(String::from("body"), body_str);
	        self.api.check(&self.db, &req_map)
		});
		let rst = rst.and_then(|msg|{
			let user_id = json_str!(&msg; "head", "userId").to_string();
			let key = json_str!(&msg; "key").to_string();	
			let rst = self.api.run(&(self.db), &msg);
			let rst = rst.and_then(|rst_json|{
				//设置customer_id
				let customer_id = json_i64!(&rst_json; "userId");
				self.customer_id = 	customer_id;
				
		        let body_str = rst_json.to_string();
		        let back_head = self.api.get_back_head(&msg, &body_str);
		        Result::Ok((back_head.to_string(), body_str, user_id, key))
			});
			rst
		});
		//返回登录信息
        let rst = match rst {
			Ok((head, body, user_id, key)) => {
				self.user_id = user_id.to_string();	//设置用户id
				self.key = key.to_string();	//设置用户密钥
				info!("{}", head);
				info!("{}", body);
				
				//send the login mananger info
				let _ = self.login_info(1);
				//send to the client
				self.proto.send(head, body);
				Result::Ok(1)
			},
			Err(e) => {
				error!("登录失败，错误代码:{}", e);
				let _ = self.login_info(0);
				Result::Err(-1)
			},
		};
        //开启消息循环
		let rst = rst.and_then(|_|{
			self.start_event_loop()
		});
	}
}

pub struct ServerManager {
	index_map: IndexMap,	//map the index and the sender of connection
	user_map: UserMap,
	man_rx: ManRx,
	db: Arc<DataBase<MyDbPool>>,
    err_map: HashMap<i64, Json>,
}

impl ServerManager {
	
	pub fn new(man_rx:ManRx, index_map:IndexMap, user_map:UserMap, db: Arc<DataBase<MyDbPool>>) -> ServerManager {
        let err_map = HashMap::new();
		ServerManager {
			man_rx: man_rx,
			index_map: index_map,
			user_map: user_map,
			db: db,
            err_map: err_map,
		}
	}
	
	///启动管理消息循环，1、处理客户连接成功、失败请求，2、分发票据
	pub fn start_loop(&mut self) {
		let mut count = 0_i64;
        //let mut gap = -1_i64;
        loop {
        	count += 1;
            /*
            if gap < 0 {
                gap = CacheService.get_print_gap(&self.db).unwrap() as i64;
                let between = Range::new(0, 8);
                let mut rng = rand::thread_rng();
                let rand_int = between.ind_sample(&mut rng);
                gap += rand_int;
            }
            */

            //处理子线程消息
            loop {
        	    let rst = self.handle_client_info();
                if rst.is_err() {
                    break;
                }
            }
            //错误票据处理倒计时
            self.timer_err_ticket();
            //处理错误票据
            self.handle_err_ticket();
            //增加cur_gap
            self.check_cur_gap();

            if count%8 == 0 {
                //发送查询请求
                self.send_query_msg();
                //属于特定终端机的查询请求
                self.terminal_query_msg();
            }

            //出票请求
            loop {
        	    let rst = self.send_ticket();
                if rst.is_err() {
                    break;
                }
            }
            //兑奖请求
            self.bonus_msg(); 

        	//4s一次的心跳信息
        	if count%5 == 0 {
        		let _ = self.send_alive_msg();
        	}

            //20分钟查一次缴款报表
            if count%1200 == 0 {
                self.generate_report_query();
            }
            
            //检查余额不足的机器
            if count%300 == 0 {
                self.check_no_balance_terminal();

                self.refresh_print_gap();
            }

            //20‘s输出终端机状态
            if count%20 == 0 {
                self.check_busy();
            }

            //递减
            //gap -= 1;
        	thread::sleep(std::time::Duration::from_millis(800));
        }
	}

    //刷新出票间隔
    pub fn refresh_print_gap(&mut self) {
        let mut user_map = self.user_map.lock().unwrap();
		for (_, mut client) in user_map.iter_mut() {
            let gap_rst = TerminalService.get_print_gap(client.customer_id, &self.db);            
            if let Ok(gap) = gap_rst {
                client.print_gap = gap; 
            }
        }
    }

    //递增cur_gap，并校验请求是否超时
    pub fn check_cur_gap(&mut self) {
        let mut time_out_list = Vec::new();
        {
            let mut user_map = self.user_map.lock().unwrap();
		    for (key, mut client) in user_map.iter_mut() {
                if client.is_idle() {
                    client.cur_gap += 1;
                }

                if client.is_waiting_response() {
                    client.msg_tick += 1;
                }
            
                if client.msg_tick > 248 {
                    time_out_list.push(key.to_string()); 
                }
            }
        }

        for key in time_out_list {
            let _ = self.user_logout(&key);
        }
    }

    //检查终端的状态
    pub fn check_busy(&self) {
        let user_map = self.user_map.lock().unwrap();
		for (_, client) in user_map.iter() {
            if !client.is_idle() {
                info!("client {}'s status is {}, gap: {}/{}, msg_tick is {}, the msg is {}.", 
                    client.user_id, client.status, client.cur_gap, 
                    client.print_gap, client.msg_tick, client.cur_msg);
            } else {
                info!("client {} is idle, gap: {}/{}.", 
                    client.user_id, client.cur_gap, client.print_gap);
            }
        }
    }

    //校验因为余额不足停用的机器，如果余额已经充足，则启用
    pub fn check_no_balance_terminal(&self) {
        let mut user_map = self.user_map.lock().unwrap();
		for (_, mut client) in user_map.iter_mut() {
            if client.status == -1 {
                let account_rst = TerminalService.get_account_by_id(client.customer_id, &self.db);
                if account_rst.is_err() {
                    continue;
                }
                let account = account_rst.unwrap();
                let balance = json_i64!(&account; "client_balance");

                if balance >= 100000 {
                    client.status = 0;
                }
            }
        }
    }

    //生成缴款报表查询请求
    pub fn generate_report_query(&self) {
        let user_map = self.user_map.lock().unwrap();
		for (key, client) in user_map.iter() {
            let terminal_id = client.customer_id;

            let mut msg = json!("{}");
            json_set!(&mut msg; "cmd"; "T05");
            let mut body = json!("{}");
            json_set!(&mut body; "type"; 0);
            json_set!(&mut msg; "body"; body);
            let _ = CacheService.send_terminal_query_msg(terminal_id, &msg, &self.db);
        }
    }

    /**
     * 处理异常票据
     */
    pub fn handle_err_ticket(&mut self) {
        let mut user_map = self.user_map.lock().unwrap();
		//遍历在线终端
		for (key, mut client) in user_map.iter_mut() {
			if !client.is_idle() {
                continue;
			}
            let err_ticket_op = self.err_map.remove(&client.customer_id);
            if err_ticket_op.is_none() {
                continue;
            }
            let ticket = err_ticket_op.unwrap();

            let id = json_i64!(&ticket; "id");
            let ticket_rst = TicketService.find_by_id(id, &self.db);
            if ticket_rst.is_err() {
                continue;
            }
            let db_ticket = ticket_rst.unwrap();
            let status = json_i64!(&db_ticket; "status");
            if status != CONS.code_to_id("ticket_status", "printing").unwrap() as i64
            {
                continue;
            } 

            //发送出票请求
            self.send_one_ticket(&ticket, client);
		}
    }

    /**
     * 处理异常票据
     */
    pub fn timer_err_ticket(&mut self) {
        let mut remove_list = Vec::new();
		for (terminal_id, mut ticket) in self.err_map.iter_mut() {
            info!("terminal {}'s ticket {} is in err status.", terminal_id, ticket);
            let timer_count = {
                let node = ticket.find("timer_count").unwrap();
                node.as_i64().unwrap()
            };
            if timer_count <= 0 {
                PrintService.print_err(*terminal_id, ticket, &self.db);
                remove_list.push(*terminal_id);
            } else {
                json_set!(ticket; "timer_count"; timer_count - 1); 
            }
        }
        for terminal_id in remove_list {
            let _ = self.err_map.remove(&terminal_id);
        }
    }
	
	/**
	 * 处理客户端信息，连接，断开，处于空闲状态等
	 */
	pub fn handle_client_info(&mut self) -> Result<i32, i32> {
        let rst = {
		    let man_rx = self.man_rx.lock().unwrap();
    	    let rst = man_rx.try_recv();
            rst
        };
    	match rst {
    		Ok(msg) => {
    			info!("{}", msg);
    			let cmd = json_i64!(&msg; "cmd");
    			if cmd == 10 {
    				let index = json_i64!(&msg; "index");
    				let index = index as i32;
    				let user_id = json_str!(&msg; "userId");
    				let customer_id = json_i64!(&msg; "customer_id");
    				let status = json_i64!(&msg; "status");
    				if status == 1 {	//登录成功
    					self.user_login(index, user_id, customer_id);
    				} else {
                        //登录失败,移除index对应的连接 
                        let mut index_map = self.index_map.lock().unwrap();
	                    let _  = index_map.remove(&index);
                    }
    			} else if cmd == 20 {	//client 断开连接
    				let user_id = json_str!(&msg; "userId");
    				self.user_logout(user_id);
    			} else if cmd == 30 {
    				let mut user_map = self.user_map.lock().unwrap();
    				let userId = json_str!(&msg; "userId");
                    let client_op = user_map.get_mut(userId);
                    if let Some(client) = client_op {
    				    client.set_status(0);
                        client.msg_tick = 0;
                    }
    			}
                Result::Ok(1)
    		},
    		Err(err) => {
    			//info!("{}", err);
                Result::Err(-1)
    		},
    	}
	}
	
	///用户登陆操作
	pub fn user_login(&self, index:i32, user_id:&str, customer_id:i64) -> Result<i32, i32> {
		let mut user_map = self.user_map.lock().unwrap();
		let mut index_map = self.index_map.lock().unwrap();
		let sender = index_map.remove(&index).unwrap();
		let mut client = Client::new(user_id, customer_id, sender);
		
		let rst = PrintService.get_terminal_game(customer_id, &self.db);
		//获得终端机可以出的游戏列表
		let rst = rst.and_then(|json|{
			let list_json = json_path!(&json; "data");
			let list = list_json.as_array().unwrap();
			for game in list {
				let game_id = json_i64!(game; "game_id");
				client.games.insert(game_id as i32);
			}
			Result::Ok(1)
		});
        //更新终端机状态和类型
        let rst = rst.and_then(|_| {
            let table = self.db.get_table("terminal").unwrap();
            let mut cond = json!("{}");
            json_set!(&mut cond; "id"; customer_id);

            let mut doc = json!("{}");
            let mut set_data = json!("{}");
            json_set!(&mut set_data; "status"; CONS.code_to_id("terminal_status", "online").unwrap());
            json_set!(&mut doc; "$set"; set_data);

            let mut op = json!("{}");
            let mut ret = json!("{}");
            json_set!(&mut ret; "type"; 1);
            json_set!(&mut ret; "print_gap"; 1);
            json_set!(&mut ret; "mode"; 1);
            json_set!(&mut ret; "help_bonus_id"; 1);
            json_set!(&mut ret; "guarantee_amount"; 1);
            json_set!(&mut op; "ret"; ret);

            let rst = table.update(&cond, &doc, &op);
            if let Ok(json) = rst {
                let cur_type = json_i64!(&json; "data", "0", "type");
                let print_gap = json_i64!(&json; "data", "0", "print_gap");
                let mode = json_i64!(&json; "data", "0", "mode");
                let help_bonus_id = json_i64!(&json; "data", "0", "help_bonus_id");
                let guarantee_amount = json_i64!(&json; "data", "0", "guarantee_amount");
                client.cur_type = cur_type;
                client.print_gap = print_gap;
                client.mode = mode;
                client.help_bonus_id = help_bonus_id;
                client.guarantee_amount = guarantee_amount;
            }
			user_map.insert(user_id.to_string(), client);
            Result::Ok(1)
        });
		rst
	}
	
	///用户登出操作（断开连接）
	pub fn user_logout(&mut self, userId:&str) -> Result<i32, i32> {
		let mut user_map = self.user_map.lock().unwrap();
		info!("{} is logout.", userId);
    	let rst = user_map.remove(userId).ok_or(-1);
        let client = try!(rst);
		if client.status == 1 {	//如果在出票中，票据重回队列
			let mut ticket = json!(&client.cur_msg);
            let resend_ticket = CacheService.resend_ticket_when_err(&self.db);
            if resend_ticket {
                let try_count = json_i64!(&ticket; "try_count");
                let new_count = try_count + 1;
                if new_count >= 4 {
                    json_set!(&mut ticket; "try_count"; 1);
                    CacheService.print(&ticket, -1, &self.db);
                } else {
                    json_set!(&mut ticket; "try_count"; new_count);
                    json_set!(&mut ticket; "timer_count"; 40);
                    self.err_map.insert(client.customer_id, ticket);
                }
            } else {
                PrintService.print_err(client.customer_id, &ticket, &self.db);
            }
        } else if client.status == 2 {	//如果在兑奖中，置成兑奖错误
            let mut ticket = json!(&client.cur_msg);
			PrintService.bonus_err(client.customer_id, &ticket, &self.db);
            /*
            let try_count = {
                let node = ticket.find("try_count");
                if let Some(count) = node {
                    count.as_i64().unwrap()
                } else {
                    1
                }
            };
            if try_count <= 1 {
                let new_count = try_count + 1;
                json_set!(&mut ticket; "try_count"; new_count);

                let id = json_i64!(&ticket; "id");
                let ticket_rst = TicketService.find_by_id(id, &self.db);
                let db_ticket = try!(ticket_rst);
                let status = json_i64!(&db_ticket; "status");
                if status == CONS.code_to_id("ticket_status", "funded").unwrap() as i64
                    || status == CONS.code_to_id("ticket_status", "bonus_err").unwrap() as i64
                {
			        CacheService.bonus(client.customer_id, &ticket, &self.db);
                }
            } else {
			    PrintService.bonus_err(client.customer_id, &ticket, &self.db);
            }
            */
        }

        //更新终端机状态
        let table = self.db.get_table("terminal").unwrap();
        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; client.customer_id);

        let mut doc = json!("{}");
        let mut set_data = json!("{}");
        json_set!(&mut set_data; "status"; CONS.code_to_id("terminal_status", "offline").unwrap());
        json_set!(&mut doc; "$set"; set_data);
        let _ = table.update(&cond, &doc, &json!("{}"));

        Result::Ok(1)
	}

    //如果有查询请求，进行查询
	pub fn terminal_query_msg(&self) -> Result<i32, i32> {
        let mut user_map = self.user_map.lock().unwrap();
		for (key, mut client) in user_map.iter_mut() {
		    if !client.is_idle() && client.status != -1 {
                continue;
		    }
            let msg_rst = CacheService.get_terminal_query_msg(client.customer_id, &self.db);  
            if msg_rst.is_err() {
                continue;
            }
            let msg_string = msg_rst.unwrap();
            let msg = json!(&msg_string);
            
            //发送查询请求
            let cmd = json_str!(&msg; "cmd");
            let body = json_path!(&msg; "body");
		    let body_string = body.to_string();
		    client.send(cmd.to_string(), body_string);
		    client.set_status(4);
		}
		Result::Ok(1)
	}

	///如果是兑奖模式，发送兑奖消息
	pub fn bonus_msg(&self) -> Result<i32, i32> {
        let mode = CacheService.get_terminal_mode(&self.db).unwrap();
        if mode == CONS.code_to_id("sys_mode", "bonus").unwrap() {
            let mut user_map = self.user_map.lock().unwrap();
		    for (key, mut client) in user_map.iter_mut() {
			    if (client.is_idle() || client.status == -1)
                    && (client.mode == 100 || client.mode == 300)
                {
                    //查看是否有票据需要兑奖
		            let mut rst = CacheService.get_ticket_to_bonus(client.customer_id, &self.db);
                    //如果没有票据需要兑奖，查看帮助终端是否有兑奖票
                    if rst.is_err() && client.help_bonus_id > 0 {
		                rst = CacheService.get_ticket_to_bonus(client.help_bonus_id, &self.db);
                    }
                    let rst = rst.and_then(|ticket| {
                        //发送兑奖请求
		                let mut body = json!("{}");
		                json_set!(&mut body; "ticket"; ticket);
		                let body_string = body.to_string();
		                client.send(String::from("T03"), body_string);
		                client.set_status(2);
		
		                //记录当前client的出票信息
		                let cur_msg = ticket.to_string();
		                client.cur_msg = cur_msg;
                        Result::Ok(1)
                    });
			    }
		    }
        }
		Result::Ok(1)
	}

	///发送心跳信息
	pub fn send_alive_msg(&mut self) -> Result<i32, i32> {
        let mut err_list = Vec::new();
        {
		    let mut user_map = self.user_map.lock().unwrap();
		    for (key, client) in user_map.iter() {
			    let mut body = json!("{}");
			    let body_string = body.to_string();
			    let rst = client.send(String::from("T10"), body_string);
                if rst.is_err() {
                    err_list.push(key.to_string());
                }
	   	    }
        }
        for key in err_list {
            self.user_logout(&key);
        }
		Result::Ok(1)
	}

    //发送查询请求
	pub fn send_query_msg(&self) -> Result<i32, i32> {
		//查看是否有票据需要打印
		let rst = CacheService.get_query_msg(&self.db);
		let rst = rst.or_else(|_|{
			info!("no msg to query.");
			Result::Err(-1)
		});
		let msg_string = try!(rst);
        let msg = json!(&msg_string);
        let cmd = json_str!(&msg; "cmd");
		
		let mut user_map = self.user_map.lock().unwrap();
		//获得适合的终端
		let mut rst = Result::Err(-1);
		for (key, mut client) in user_map.iter_mut() {
			if client.is_idle() && client.cur_type == 1 {
				rst = Result::Ok(client);
				break;
			}
		}
		let rst = rst.or_else(|_|{
			info!("no terminal to query {}", msg);
			CacheService.send_query_msg(&msg, &self.db);	//返回查询队列
			Result::Err(-1)
		});
		let mut client = try!(rst);
		
		//发送查询请求
        let body = json_path!(&msg; "body");
		let body_string = body.to_string();
		client.send(cmd.to_string(), body_string);
		client.set_status(4);
		
		Result::Ok(1)
	}

	pub fn send_ticket(&self) -> Result<i32, i32> {
        //基本出票间隔
        let sys_gap = CacheService.get_print_gap(&self.db);
		//查看是否有票据需要打印
		let rst = CacheService.get_ticket_to_print(&self.db);
		let rst = rst.or_else(|_|{
			info!("no ticket to print.");
			Result::Err(-1)
		});
		let ticket = try!(rst);
        //校验截止时间
        let end_time_node = ticket.find("end_time").unwrap();
        let end_time = end_time_node.as_i64().unwrap();
        let now = Local::now();
        //如果截止时间已过，置成出票错误
        if end_time < now.timestamp() {
            PrintService.print_err(-1, &ticket, &self.db);
            return Result::Ok(1);
        }
        //再次校验票据的状态
        let id_node = ticket.find("id").unwrap();
        let id = id_node.as_i64().unwrap();
        let status_rst = TicketService.get_status_by_id(id, &self.db);
        match status_rst {
            Ok(status) => {
                if status != 10 {
                    return Result::Ok(1);
                }
            },
            Err(flag) => {
                return Result::Ok(1);
            },
        }
        
        let amount = json_i64!(&ticket; "amount");
		let game_id = json_i64!(&ticket; "game_id") as i32;
		
		let mut user_map = self.user_map.lock().unwrap();
		//获得空闲的出票终端
        let mut idle_list = Vec::new();
        //备用队列，在系统票据较多时，起作用
        let mut second_list = Vec::new();
        let mut max_key_op =  None;
        let mut weight = -1_f64;
		for (key, mut client) in user_map.iter_mut() {
            if client.is_idle()
                && client.games.contains(&game_id) 
                && (client.mode == 100 || client.mode == 200)
            {
                //0为最高权重
                if client.print_gap <= 0 {
                    idle_list.push(key.to_string());
                } else if client.cur_gap > client.print_gap {
                    let cur_weight = (client.cur_gap as f64)/(client.print_gap as f64);
                    if cur_weight > weight {
                        max_key_op = Some(key.to_string()); 
                        weight = cur_weight;
                    }
                } else if client.cur_gap > sys_gap {   //加入备用队列
                    second_list.push(key.to_string());
                }
			}
		}
        if let Some(max_key) = max_key_op {
            idle_list.push(max_key);
        }
        //优先队列和备用队列都无终端可用
        if idle_list.len() == 0 && second_list.len() == 0{
			info!("no terminal to print {}", ticket);
			CacheService.print(&ticket, -1, &self.db);	//返回出票队列
            return Result::Err(-1);
        }
        
        let mut client;
        if idle_list.len() > 0 {
            let between = Range::new(0, idle_list.len());
            let mut rng = rand::thread_rng();
            let rand_int = between.ind_sample(&mut rng);

            let key = idle_list.get(rand_int).unwrap();
            client = user_map.get_mut(key).unwrap();
        } else {
            let between = Range::new(0, second_list.len());
            let mut rng = rand::thread_rng();
            let rand_int = between.ind_sample(&mut rng);

            let key = second_list.get(rand_int).unwrap();
            client = user_map.get_mut(key).unwrap();
        }

        //检查终端机余额，如果终端机余额不足，
        //status置为-1，表示终端机不可用
        if !client.has_enough_balance(amount, &self.db) {
			CacheService.print(&ticket, -1, &self.db);	//返回出票队列
            return Result::Ok(1);
        }

        client.cur_gap = 0;

		//发送出票请求
        self.send_one_ticket(&ticket, client);
		
		Result::Ok(1)
	}

    //send one ticket to client's channel
	pub fn send_one_ticket(&self, ticket:&Json, client:&mut Client) {
        //发送出票请求
		let mut body = json!("{}");
		json_set!(&mut body; "ticket"; ticket.clone());
		let body_string = body.to_string();
		let rst = client.send(String::from("T02"), body_string);
        if rst.is_err() {
            //返回出票队列
			CacheService.print(ticket, -1, &self.db);
            client.set_status(-2);
        } else {
		    client.set_status(1);
		    //记录当前client的出票信息
		    let cur_msg = ticket.to_string();
		    client.cur_msg = cur_msg;
        }
    }
}

/**
 * 服务器
 */
pub struct Server {
	url: String,
	listener: TcpListener,
	index: i32,	//计数器，为了一一对应当前登陆得用户
	index_map: IndexMap,	//map the index and the sender of connection
	user_map: UserMap,
	man_rx: ManRx,
	man_sx: Sender<ManMsg>, 
	api: Arc<ApiFactory>,
	db: Arc<DataBase<MyDbPool>>,
}

impl Server {
	
	pub fn new(url:&str, api: ApiFactory, db:DataBase<MyDbPool>) -> Server {
		let listener = TcpListener::bind(url).unwrap();
		let (man_sx, man_rx) = mpsc::channel::<ManMsg>();
		let api = Arc::new(api);
		let db = Arc::new(db);
		Server {
			url: url.to_string(),
			listener: listener,
			index: 0_i32,
			index_map: Arc::new(Mutex::new(BTreeMap::<i32, Sender<(String, String)>>::new())),
			user_map: Arc::new(Mutex::new(BTreeMap::<String, Client>::new())),
			man_sx: man_sx,
			man_rx: Arc::new(Mutex::new(man_rx)),
			api: api,
			db: db,
		}
	}
	
	pub fn start_man_thread(&mut self) {
		let man_rx = self.man_rx.clone();
		let user_map = self.user_map.clone();
		let index_map = self.index_map.clone();
		let db = self.db.clone();
		//处理出票分发的线程
		thread::spawn(move|| {
	        let mut sm = ServerManager::new(man_rx, index_map, user_map, db);
	        sm.start_loop();
	    });
	}
	
	/**
	 * this method will block
	 */
	pub fn start(&mut self) {
		info!("server start at: {}", self.url);
		self.start_man_thread();
		for stream_rst in self.listener.incoming() {
			let index_map = self.index_map.clone();
			let man_sx = self.man_sx.clone();
			let api = self.api.clone();
			let db = self.db.clone();
			self.index += 1;
			let index = self.index;
			let _ = stream_rst.and_then(|stream|{
				let (sx, rx) = mpsc::channel::<(String, String)>();
	        		let rx = Arc::new(Mutex::new(rx));
	    			//lock the index map
    				let mut index_map = index_map.lock().unwrap();
    				index_map.insert(index, sx);
    				let rx = rx.clone();
    				thread::spawn(move|| {
                        stream.set_read_timeout(Some(std::time::Duration::from_millis(100)));
    					let proto = Protocol::new(stream, String::from("abc"));
    					let mut ch = ClientHandler::new(proto, man_sx, rx, index, api, db);
    					ch.start();
	            });
    				Result::Ok(1)
			});
		}
	}
}
