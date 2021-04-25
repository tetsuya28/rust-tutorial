fn main() {
  let s1 = String::from("hello");
  let len = calculate_length(&s1);
  println!("The length of {} is {}.", s1, len);

  let mut s = String::from("hello");
  change(&mut s);
  println!("The string is {}.", s);

  let r1 = &s;
  let r2 = &s;
  println!("{} and {}", r1, r2);

  let r3 = &mut s;
  println!("{}", r3);
  // println!("{}, {} and {}", r1, r2, r3);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
