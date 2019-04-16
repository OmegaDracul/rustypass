#![feature(toowned_clone_into)]

extern crate rustypasslib;

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
		rustypasslib::read_user_input(&mut user_choice).trim().clone_into(&mut user_choice);

		match user_choice.as_str() {
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
				rustypasslib::wait_for_input();
			}
		}
	}
}
