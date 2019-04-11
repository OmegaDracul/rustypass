use easy_password::bcrypt::{
	hash_password,
	verify_password,
};

use std::fs;

#[test]
fn test_pass() {
	let bcrypt_rounds = 12;
	let hash = dbg!(hash_password("my_password", b"secure_key", bcrypt_rounds).unwrap());

	let success = dbg!(verify_password("my_password", hash.as_str(), b"secure_key").unwrap());
	assert!(success);
}

#[test]
fn test_read_pass() {
	let file = fs::read("user").expect("Something went wrong reading the file");
	let password = String::from_utf8(file).unwrap();
	//let useable_pass = password.clone();
	let confirm = dbg!(verify_password("myfather", password.as_str(), b"myfather",).unwrap());

	// println!("{:?}", password);
	println!("{}", confirm);
}