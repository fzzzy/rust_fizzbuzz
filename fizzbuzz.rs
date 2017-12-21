/*
  example from Programming Rust
*/

use std::iter::{once, repeat};

fn main() {
  let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
  let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
  let fizz_buzz_zip = fizzes.zip(buzzes);

  let fizzbuzz = (1..100).zip(fizz_buzz_zip).map(|tuple|
    match tuple {
      (i, ("", "")) => i.to_string(),
      (_, (fizz, buzz)) => format!("{}{}", fizz, buzz)
    });

  for line in fizzbuzz {
    println!("{}", line);
  }
}
