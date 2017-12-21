/*
  Normal procedural fizzbuzz
*/

fn main() {
  for i in 1..100 {
    let mut fizz = "";
    let mut buzz = "";
    if i % 3 == 0 {
      fizz = "fizz";
    }
    if i % 5 == 0 {
      buzz = "buzz";
    }
    if fizz == "" && buzz == "" {
      println!("{}", i);
    } else {
      println!("{}{}", fizz, buzz);
    }
  }
}
