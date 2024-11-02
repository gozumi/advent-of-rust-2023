use std::env;

use common::utils::get_file_lines;
use days::{day_01::day_01, day_02::day_02};

mod common;
mod days;

fn main() {
  let args: Vec<String> = env::args().collect();
  let day = &args[1];
  // let file_path = &args[2];
  let file_path = format!("src/days/day_{}_data.txt", day);

  let lines_buffer = get_file_lines(file_path);

  match (*day).as_str() {
    "01" => day_01(lines_buffer),
    "02" => day_02(lines_buffer),
    _ => println!("Cannot find a module for day {}", day),
  }
}
