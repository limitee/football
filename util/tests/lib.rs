extern crate util;
use util::DigestUtil;
use util::MathUtil;
use util::NumberUtil;

#[test]
fn md5_test() {
    assert!("900150983cd24fb0d6963f7d28e17f72" == DigestUtil::md5("abc"));
	assert!("a7bac2239fcdcb3a067903d8077c4a07" == DigestUtil::md5("中文"));
    assert!(4_i64 == MathUtil::get_a(4, 1));
	assert!(12_i64 == MathUtil::get_a(4, 2));
	assert!(6_i64 == MathUtil::get_c(4, 2));
	
	let vec = NumberUtil::to_int_array("01,02,13");
	assert_eq!(vec![1, 2, 13], vec);
	let rst = NumberUtil::array_sort_from_min_to_max(&vec);
	assert!(Result::Ok(1) == rst);
	
	let rst = NumberUtil::check_margin(&vec, 1, 12);
	assert!(Result::Err(-1) == rst);
	
	let vec = NumberUtil::to_int_array("10,12,01");
	assert_eq!(vec![10, 12, 1], vec);
	let rst = NumberUtil::array_sort_from_min_to_max(&vec);
	assert!(Result::Err(-1) == rst);
	
	let rst = NumberUtil::check_margin(&vec, 1, 12);
	assert!(Result::Ok(1) == rst);
}

#[test]
fn detailc_test() {
    let dc = MathUtil::get_detailc(8, 2);
    for i in 0..dc.data.len() {
        let set = dc.data.get(i).unwrap(); 
        println!("{:?}", set);
    }
}
