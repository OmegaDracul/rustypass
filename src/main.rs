extern crate rustypasslib;

use std::io;

fn main() {
	println!("Welcome to rustypass");

	// Looping to make sure it doesn't break if it the user doesn't
	// perform the right action
	loop {
		println!(
			"1. Check for existing folder
2. Create a new user
3. Login to account

q to quit
"
		);

		let mut user_choice = String::new();
		io::stdin().read_line(&mut user_choice).unwrap();
		let input = user_choice.trim();

		match input {
			"1" => {
				println!();
				rustypasslib::create_directory()
			}
			"2" => {
				println!();
				rustypasslib::new_user()
			}
			"3" => {
				println!();
				rustypasslib::login_user()
			}
			"q" => {
				rustypasslib::clear_screen();
				break;
			}
			_ => {
				eprintln!("Invalid choice");
				let mut input = String::new();
				io::stdin().read_line(&mut input).unwrap();
			}
		}
	}
}
