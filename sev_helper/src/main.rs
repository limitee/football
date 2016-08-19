extern crate sev_helper;
use sev_helper::GameService;

#[macro_use]
extern crate log;
extern crate elog;

extern crate regex;
use regex::Regex;

fn main() {
	let _ = elog::init();
	
	let stub = "中国体育彩票演示票\n演示票竞彩足球胜平负   单场固定\n200243-945712-189922-67 FD362CCE 00546858\n─────────────────────\n周日001\n主队:皇家马德里 Vs 客队:巴塞罗那\n胜@12.0元+平@12.0元+负@12.0元\n(选项固定奖金额为每1元投注对应的奖金额)\n本票最高可能固定奖金:268.00元\n *  *  *\n *  *  *\n─────────────────────\n倍数:1  合计:    样票  2016-04-16 17:22:40\n演示票，请勿用于销售！！！\r\n中国竞彩网 http://www.sporttery.cn\n\r\n2001021D32303032343339343537313231383939323236372030303534363835";
	let re = Regex::new(r"周(\w)(\d{3})\n(\w{2}):(\w{1,})\sVs\s(\w{2}):(\w{1,})\n(.+)\n").unwrap();
	let cap_op = re.captures(stub);
	match cap_op {
		Some(x) => {
			//let sep = x.at(0).unwrap_or("");
			info!("length:{}", x.len());
			let day = x.at(1).unwrap_or("");
			let seq = x.at(2).unwrap_or("");
			let master = x.at(3).unwrap_or("");
			let master_team = x.at(4).unwrap_or("");
			let guest = x.at(5).unwrap_or("");
			let guest_team = x.at(6).unwrap_or("");
			let rate = x.at(7).unwrap_or("");
			info!("{}-{}-{}-{}-{}-{}-{}", day, seq, master, master_team, guest, guest_team, rate);
		},
		None => {
			info!("not found");
		},
	}
	/*
	let game_list = GameService.get_game_list();
	for game in game_list {
		info!("{}", game);	
	}*/
}