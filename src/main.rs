use std::env;

use common::utils::get_file_lines;
use days::{day_01::execute_day_01, day_02::execute_day_02};

mod common;
mod days;

fn main() {
  let args: Vec<String> = env::args().collect();
  let day = match &args[1].parse::<u32>() {
    Ok(d) => *d,
    Err(_) => panic!("The day argument must be a number!"),
  };
  let day = match day < 10 {
    true => format!("0{}", day),
    false => format!("{}", day),
  };

  let file_path = format!("src/days/day_{0}/data.txt", day);

  let lines_buffer = get_file_lines(file_path);

  match &day[..] {
    "01" => execute_day_01(lines_buffer),
    "02" => execute_day_02(lines_buffer),
    _ => println!("Cannot find a module for day {}", day),
  }
}
