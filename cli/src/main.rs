use std::env;

use common::utils::get_file_lines;
use days::{
  day_01::{execute_day_01, execute_day_01_part_02},
  day_02::execute_day_02,
};

mod common;
mod days;

fn main() {
  let args: Vec<String> = env::args().collect();

  let params: Vec<&str> = args[1].split(".").collect();

  let day = *params.get(0).unwrap();

  let day = match day.parse::<u32>() {
    Ok(d) => d,
    Err(_) => panic!("The `day` argument must be a number!"),
  };

  let day = match day < 10 {
    true => format!("0{}", day),
    false => format!("{}", day),
  };

  let file_path = format!("cli/src/days/day_{0}/data.txt", day);

  let lines_buffer = get_file_lines(file_path);

  let day_and_part = if params.len() > 1 {
    let part = *params.get(1).unwrap();

    let part = match part.parse::<u32>() {
      Ok(d) => d,
      Err(_) => panic!("The `part` argument must be a number!"),
    };

    format!("{}_part_{}", day, part)
  } else {
    format!("{}", day)
  };

  match &day_and_part[..] {
    "01" => execute_day_01(lines_buffer),
    "01_part_2" => execute_day_01_part_02(lines_buffer),
    "02" => execute_day_02(lines_buffer),
    _ => println!("Cannot find a module for day {}", day_and_part),
  }
}
