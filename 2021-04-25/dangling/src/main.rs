fn main() {
	let reference_to_noting = dangle();
	println!("{}", reference_to_noting);
}

fn dangle() -> String {
	let s = String::from("hello");
	s
}

