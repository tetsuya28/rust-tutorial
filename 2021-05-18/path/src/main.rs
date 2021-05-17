mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {}
	}
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
	hosting::add_to_waitlist();
}

use std::collections::HashMap;

// use std::fmt;
// use std::io as IoResult;
// 
// fn function1() -> fmt::Result {
// }
// 
// fn function2() -> IoResult<()> {
// }

fn main() {
	let mut map = HashMap::new();
	map.insert(1, 2);
}
