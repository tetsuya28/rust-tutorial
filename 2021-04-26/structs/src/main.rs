struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

fn main() {
	let user = build_user(String::from("hoge@example.com"), String::from("hoge"));
	println!("{}", user.email);
	println!("{}", user.username);
	println!("{}", user.active);
	println!("{}", user.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
	User {
		email,
		username,
		active: true,
		sign_in_count: 1,
	}
}
