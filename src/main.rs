use std::io::BufRead;
use std::{env, fs::File, io};

use common::utils::get_file_lines;
use days::{day_01::day_01, day_02::day_02};

mod common;
mod days;

fn main() {
  let args: Vec<String> = env::args().collect();
  let day = &args[1];
  let file_path = &args[2];

  let lines_buffer = get_file_lines(file_path);

  match (*day).as_str() {
    "1" => day_01(lines_buffer),
    "2" => day_02(lines_buffer),
    _ => println!("Cannot find a module for day {}", day),
  }
}
