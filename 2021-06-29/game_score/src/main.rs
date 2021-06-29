use std::collections::HashMap;

fn main() {
	let mut scores = HashMap::new();
	
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);
	let team_name = String::from("Blue");
	scores.entry(String::from("Yellow")).or_insert(50);
	let score = scores.get(&team_name);
	for (key, value) in &scores {
		println!("{}: {}", key, value)
	}

	let teams = vec![String::from("Blue"), String::from("Yellow")];
	let initial_scores = vec![10, 50];
	let mut scores: HashMap<_, _> = 
		teams.into_iter().zip(initial_scores.into_iter()).collect();

	let field_name = String::from("Favorite color");
	let field_value = String::from("Blue");
	let mut map = HashMap::new();
	map.insert(field_name, field_value);

	let text = "hellow world wonderful world";
	let mut map = HashMap::new();
	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);
		*count += 1;
	}
	println!("{:?}", map);
}
