#![feature(toowned_clone_into)]

use std::{
	env,
	fs,
	io as stdio,
	path::Path,
	process::Command,
};

use easy_password::bcrypt::{
	hash_password,
	verify_password,
};

#[cfg(test)]
mod tests;

pub struct User {
	username: String,
	password: String,
}

impl User {
	// Creating a new user storage
	pub fn new(username: &str, password: &str) -> User {
		User {
			username: username.into(),
			password: password.into(),
		}
	}
}

// A constant for the path to be used in experimentation
const PATH: &str = "rustypass";
const ACCOUNT_PATH: &str = "accounts";

pub fn create_directory() {
	let path = Path::new(PATH);
	println!("Checking for the rustypass folder\n");

	// Checks if the path exists and if it doesn't, it creates the folder
	// in the path. If it does exist, then it tells the user
	if path.exists() == false {
		fs::create_dir(path).unwrap();
		println!("Created the rustypass folder");
		wait_for_input();
		clear_screen();
	}
	else {
		println!("rustypass folder already exists");
		wait_for_input();
		clear_screen();
	};
}

pub fn new_user() {
	let path = Path::new(PATH);

	// Checks to see if the rustypass folder exists before doing anything
	// else
	if path.exists() == false {
		clear_screen();
		println!("Please create the rustypass folder first");
		wait_for_input();
	}
	else {
		println!("Creating a new account:");

		// The loop makes sure that the user has created a proper username and
		// password before it breaks
		loop {
			clear_screen();

			println!("New Username:");
			let mut username = String::new();
			read_user_input(&mut username)
				.trim()
				.clone_into(&mut username);

			println!();

			println!("New Password:");
			let mut password = String::new();
			read_user_input(&mut password)
				.trim()
				.clone_into(&mut password);

			println!("Confirm password:");
			let mut confirm_password = String::new();
			read_user_input(&mut confirm_password)
				.trim()
				.clone_into(&mut confirm_password);

			// The secure_key unlocks the hashed password
			println!("Enter security key:");
			let mut secure_key = String::new();
			read_user_input(&mut secure_key)
				.trim()
				.clone_into(&mut secure_key);

			println!("Confirm security key:");
			let mut confirm_security = String::new();
			read_user_input(&mut confirm_security)
				.trim()
				.clone_into(&mut confirm_security);

			if secure_key == confirm_security {
				let hashed_password = hash_password(&password, secure_key.as_ref(), 12).unwrap();
				let new_password = hashed_password.trim();
				let _hashed_password_confirm =
					hash_password(&confirm_password, secure_key.as_ref(), 12)
						.unwrap()
						.trim();

				// Sets the user's folder
				let username_folder = format!("rustypass/{}", username);
				let username_path = Path::new(&username_folder);

				// Checks if the user's profile folder already exists and creates one
				// when it doesn't exist

				if username_path.exists() {
					eprintln!("This user profile already exists");
					wait_for_input();
				}
				else {
					// Checks both passwords to make sure they match and if they don't
					// prints an error
					if password != confirm_password {
						// Waiting for user input to make sure the user reads the error
						// message
						clear_screen();
						println!("The two passwords do not match");
						wait_for_input();
					}
					else {
						// Creates the rustypass file for the user to use whenever they want
						// to create a new account and password within the username
						fs::create_dir(&username_folder).unwrap();
						fs::File::create(format!("rustypass/{}/user", username)).unwrap();
						let user = create_user_space(username.as_str(), new_password);
						fs::write(
							format!("rustypass/{}/user", user.username),
							format!("{}", user.password.to_string()),
						)
						.unwrap();

						clear_screen();
						println!("User created");
						wait_for_input();
						break;
					}
				}
			}
			else {
				println!("Secure keys do not match")
			}
		}
	}
}

pub fn login_user() {
	let path = Path::new(PATH);

	// Checks if the path exists and if it does requests the user's info
	if path.exists() == false {
		println!("Please create the rustypass folder first");

		wait_for_input();
	}
	else {
		println!("Username:");
		let mut username = String::new();
		read_user_input(&mut username)
			.trim()
			.clone_into(&mut username);

		println!();

		let username_folder = format!("rustypass/{}", username);
		let username_path = Path::new(&username_folder);

		if username_path.exists() == false {
			println!();
			eprintln!("The user profile you're trying to access doesn't exist");

			// Waiting for user input to make sure the user reads the error
			// message
			wait_for_input();
		}
		else {
			loop {
				println!("Password:");
				let mut password = String::new();
				read_user_input(&mut password)
					.trim()
					.clone_into(&mut password);

				// Reads user password from the user file in the rustypass folder
				if read_password(password.as_str(), username.as_str()) {
					clear_screen();
					println!("Welcome, {}", username);

					// Sets the users directory to the depth of the users
					env::set_current_dir(username_path).unwrap();
					wait_for_input();
					break;
				};
			}
			clear_screen();

			// Makes sure the user is logged in then moves the experience
			logged_in_experience();
			println!();
		};
	}
}

pub fn logged_in_experience() {
	loop {
		println!(
			"1. Create a new account
2. Access an account
3. Change account password
4. Delete account

q to quit
"
		);

		let mut user_choice = String::new();
		read_user_input(&mut user_choice)
			.trim()
			.clone_into(&mut user_choice);

		match user_choice.as_str() {
			"1" => {
				println!();
				new_account();
			}
			"2" => {
				println!();
				access_accounts();
			}
			"3" => {
				println!();
				change_password();
			}
			"4" => {
				println!();
				delete_account();
			}
			"q" => {
				clear_screen();

				// Returns to the rustypass directory
				env::set_current_dir(Path::new("../..")).unwrap();
				break;
			}
			_ => {
				eprintln!("Invalid choice");
				wait_for_input()
			}
		}

		println!();
	}
}

pub fn new_account() {
	let account_path = Path::new(ACCOUNT_PATH);

	// Checks if the account folder exists, and if it doesn't, creates one

	if account_path.exists() == false {
		fs::create_dir(account_path).unwrap();
		println!("Created the account folder");
		wait_for_input();
		create_account();
	}
	else {
		create_account();
	}

	wait_for_input();
	clear_screen();
}

fn create_account() {
	// User enters the new account they want to make
	println!("Enter account:");
	let mut account = String::new();

	read_user_input(&mut account)
		.trim()
		.clone_into(&mut account);

	let path = format!("{}/{}", ACCOUNT_PATH, account);
	let account_path = Path::new(&path);

	/*
	Checks if the account already exists and if it doesn't:
	- Requests the user's information
	- Encrypts using the information received
	- Proceeds to create the file
	*/

	if account_path.exists() == false {
		println!("Enter username:");
		let mut username = String::new();
		read_user_input(&mut username)
			.trim()
			.clone_into(&mut username);

		println!();

		println!("Enter account password:");
		let mut password = String::new();
		read_user_input(&mut password)
			.trim()
			.clone_into(&mut password);

		println!("Confirm account password:");
		let mut confirm_password = String::new();
		read_user_input(&mut confirm_password)
			.trim()
			.clone_into(&mut confirm_password);

		println!();

		if password == confirm_password {
			println!("Secure key:");
			let mut secure_key = String::new();
			read_user_input(&mut secure_key)
				.trim()
				.clone_into(&mut secure_key);

			println!("Confirm secure key:");
			let mut confirm_secure_key = String::new();
			read_user_input(&mut confirm_secure_key)
				.trim()
				.clone_into(&mut confirm_secure_key);

			println!();

			if secure_key == confirm_secure_key {
				let password = hash_password(&password, secure_key.as_ref(), 12).unwrap();
				fs::write(account_path, format!("{}\n{}", username, password)).unwrap();
				println!("Account created");
				wait_for_input();
			}
			else {
				println!("Secure keys do not match")
			}
		}
		else {
			println!("Passwords do not match")
		}
	}
	else {
		println!("The account you're trying to create already exists")
	}

	clear_screen()
}

pub fn access_accounts() {
	// TODO: Check for the accounts in the user's folder
	println!("Accessing an account");
	wait_for_input();
	clear_screen();
}

pub fn delete_account() {
	// TODO: Make it possible for the user to delete their account

	// First would be make sure they're deleting the right account
	println!("Which account would you like to delete?");
	let mut account = String::new();
	read_user_input(&mut account)
		.trim()
		.clone_into(&mut account);

	let account_path = format!("accounts/{}", account);
	let account_exists = Path::new(&account_path).exists();

	// Next would be removing the file
	if account_exists {
		fs::remove_file(&account_path).unwrap();
		println!("Account deleted");
		wait_for_input();
		clear_screen();
	}
	else {
		println!("The account doesn't exist");
		wait_for_input();
	}
}

pub fn change_password() {
	// TODO: Have the user be able to change their password
	println!("Are you sure you want to change your password? Y/y or N/n");
	let mut answer = String::new();
	read_user_input(&mut answer).trim().clone_into(&mut answer);

	match answer.as_str() {
		"y" | "Y" | "Yes" => password_changing_process(),
		"n" | "N" | "No" => println!("Password remains unchanged"),
		_ => println!("Wrong input received... Going back"),
	}

	wait_for_input();
	clear_screen();
}

pub fn delete_password() {
	// TODO: Have the user be able to delete a password
	println!("Deleting a password");
	wait_for_input();
	clear_screen();
}

fn password_changing_process() {
	// TODO: Change user's password

	// Request the old password from the user
	println!("Enter your old password:");
	let mut old_password = String::new();
	stdio::stdin().read_line(&mut old_password).unwrap();
	let old_password = old_password.trim();

	println!("Enter your secure key:");
	let mut secure_key = String::new();
	stdio::stdin().read_line(&mut secure_key).unwrap();
	let secure_key = secure_key.trim();

	if match_password(old_password, secure_key) {
		// Request a new password from the user

		println!("Enter your new password:");
		let mut new_password = String::new();
		stdio::stdin().read_line(&mut new_password).unwrap();
		let new_password = new_password.trim();

		println!("Confirm new password:");
		let mut confirm_new_password = String::new();
		stdio::stdin().read_line(&mut confirm_new_password).unwrap();
		let confirm_new_password = confirm_new_password.trim();

		println!();

		if new_password == confirm_new_password {
			println!("Would you like to change your secure key? Y/y or N/n");
			let mut answer = String::new();
			stdio::stdin().read_line(&mut answer).unwrap();
			let answer = answer.trim();

			match answer {
				"y" | "Y" | "Yes" => {
					println!("Enter new secure key:");
					let mut new_secure_key = String::new();
					stdio::stdin().read_line(&mut new_secure_key).unwrap();
					let new_secure_key = new_secure_key.trim().as_ref();
					let password = hash_password(new_password, new_secure_key, 12).unwrap();
					fs::write("user", password.to_string()).unwrap();
					println!("Changed password and secure key")
				}
				"n" | "N" | "No" => {
					let password = hash_password(new_password, secure_key.as_bytes(), 12).unwrap();
					fs::write("user", password.to_string()).unwrap();
					println!("Changed password")
				}
				_ => println!("Invalid input. Will default to no"),
			}
		}
		else {
			println!("The two passwords do not match")
		}
	}
	// Overwrite the old password
}

/* Functions to make things easier */
fn match_password(pass: &str, secure_key: &str) -> bool {
	let file = fs::read("user").expect("Something went wrong reading the file");
	let password = String::from_utf8(file).unwrap();

	let confirm = verify_password(&pass, password.as_str(), secure_key.as_bytes()).unwrap();

	if confirm {
		confirm
	}
	else {
		println!("Incorrect password or secure key");
		confirm
	}
}

fn read_password(pass: &str, user: &str) -> bool {
	// Reads the user file
	let user = user;
	let file = fs::read(format!("rustypass/{}/user", user))
		.expect("Something went wrong reading the file");
	let password = String::from_utf8(file).unwrap();
	let pass = pass;

	// Checks the secure key
	println!("Enter your secure key:");
	let mut secure_key = String::new();
	stdio::stdin().read_line(&mut secure_key).unwrap();
	let secure_key = secure_key.trim();

	let confirm = verify_password(&pass, password.as_str(), secure_key.as_bytes()).unwrap();

	if confirm {
		confirm
	}
	else {
		println!("Incorrect password or secure key");
		confirm
	}
}

/* These are functions created to make manipulating to make things
 * easier to code and to also improve the user experience */

fn create_user_space(username: &str, password: &str) -> User {
	let user = User::new(username, password);

	user
}

fn wait_for_input() {
	let mut input = String::new();
	stdio::stdin().read_line(&mut input).unwrap();
}

fn read_user_input(value: &mut String) -> String {
	// let mut confirm_new_password = String::new();
	// stdio::stdin().read_line(&mut confirm_new_password).unwrap();
	// let confirm_new_password = confirm_new_password.trim();
	let mut value = String::from(value.clone());
	stdio::stdin().read_line(&mut value).unwrap();

	value
}

pub fn clear_screen() {
	let output = Command::new("clear")
		.output()
		.unwrap_or_else(|e| panic!("failed to execute process: {}", e));

	println!("{}", String::from_utf8_lossy(&output.stdout));
}
