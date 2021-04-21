use std::io;

fn main() {
	let x = 5;
	let x = x + 1;
	let x = x * 2;
	println!("The value of x is: {}", x);

  let sum = 5 + 10;
  let difference = 95.5 - 4.3;
  let product = 4 * 30;
  let quotient = 56.7 / 32.2;
  let remainder = 43 % 5;
	println!("{}", sum);
	println!("{}", difference);
	println!("{}", product);
	println!("{}", quotient);
	println!("{}", remainder);

  let tup: (i32, f64, u8) = (500, 6.4, 1);
  println!("{}", tup.0);
  let (x, y, z) = tup;
  println!("{}, {}, {}", x, y, z);

  let a: [i32; 5] = [1, 2, 3, 4, 5];
  println!("{}", a[0]);
  let b = [3; 5];
  println!("{}", b[3]);

  println!("Please enter an array index.");
  
  let mut index = String::new();
  io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

  let index: usize = index
    .trim()
    .parse()
    .expect("Index entered was not a number");

  let element = a[index];
  println!("The value of the element at index {} is: {}",
    index, element
  );
}
