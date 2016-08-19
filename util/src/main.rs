extern crate util;
use util::DigestUtil;
use util::MathUtil;
use util::NumberUtil;

fn main() {
    println!("{}", NumberUtil::get_jc_rate(9.5000));
    println!("{}", NumberUtil::get_jc_rate(8.4000));
    println!("{}", NumberUtil::get_jc_rate(8.5000));
    println!("{}", NumberUtil::get_jc_rate(8.6000));
    println!("{}", NumberUtil::get_jc_rate(8.50000000001));
    println!("{}", NumberUtil::get_jc_rate(8.500000001));
}
