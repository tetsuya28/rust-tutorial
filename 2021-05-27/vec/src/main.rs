fn main() {
	// let v: Vec<i32> = Vec::new();
	let mut vv = vec![1, 2, 3];

	vv.push(4);
	vv.push(5);

	let third: &i32 = &vv[2];
	println!("The third element is {}", third);

	match vv.get(2) {
		Some(third) => println!("The third element is {}", third),
		None => println!("There is no third element."),
	}

	let v = vec![1, 2, 3, 4, 5];
	// let does_not_exist = &v[100]; // panic
	let does_not_exist = v.get(100); // ok
	println!("{:?}", does_not_exist);

	// let mut vvv = vec![1, 2, 3, 4, 5];
	// let first = &vvv[0];
	// vvv.push(6);
	// println!("The first element is: {}", first);

	let mut vvvv = vec![100, 32, 57];
	for i in &mut vvvv {
		*i += 50;
	}
	println!("{:?}", vvvv);

	#[derive(Debug)]
	enum SpreadsheetCell {
		Int(i32),
		Float(f64),
		Text(String),
	}

	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(4.5),
	];
	println!("{:?}", row);
}
