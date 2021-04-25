fn main() {
  // let s1 = String::from("hello");
  // let s2 = s1.clone();

  // println!("s1 = {}, s2 = {}", s1, s2);

  let s = String::from("hello");
  takes_ownership(s);
  // println!("{}", s);

  let x = 5;
  makes_copy(x);
  // println!("{}", x);

  let s1 = gives_ownership();
  println!("{}", s1);
  let s2 = String::from("hello");
  println!("{}", s2);
  let s3 = takes_and_gives_back(s2);
  println!("{}", s3);
}

fn takes_ownership(some_string: String) {
  println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
  println!("{}", some_integer);
}

fn gives_ownership() -> String {
  let some_string = String::from("hello");
  some_string
}

fn takes_and_gives_back(a_string: String) -> String {
  a_string
}
