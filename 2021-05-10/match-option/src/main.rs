fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i+i),
	}
}

fn main() {
	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);

	println!("{:?}", five);
	println!("{:?}", six);
	println!("{:?}", none);

	let some_u8_value = 0u8;
	match some_u8_value {
		1 => println!("one"),
		_ => (),
	}
}

