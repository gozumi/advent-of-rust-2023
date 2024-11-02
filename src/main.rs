use std::env;

use days::{day_01::day_01, day_02::day_02};

mod days;

fn main() {
  let args: Vec<String> = env::args().collect();
  let day = &args[1];

  match (*day).as_str() {
    "1" => println!("{}", day_01()),
    "2" => println!("{}", day_02()),
    _ => println!("Cannot find a module for day {}", day),
  }
}
