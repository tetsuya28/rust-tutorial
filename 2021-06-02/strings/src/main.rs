fn main() {
	let mut s = String::new();
	s.push_str("bar");
	let data = "initial contents";
	let d = data.to_string();
	let dd = "initial contents".to_string();
	let ss = String::from("initial contetns");

	let mut s1 = String::from("foo");
	let s2 = "bar";
	s1.push_str(s2);
	s1.push('l');
	println!("s2 is {}", s2);

	let a1 = String::from("Hello, ");
	let a2 = String::from("world!");
	let a3 = a1 + &a2;
	println!("{}", a3);

	let q1 = String::from("tic");
	let q2 = String::from("tac");
	let q3 = String::from("toe");
	let q = format!("{}-{}-{}", q1, q2, q3);
	println!("{}", q)
}

