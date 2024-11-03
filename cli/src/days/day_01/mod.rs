use std::{
  fs::File,
  io::{BufReader, Lines},
};

use utils::{get_calibration_value, get_enhanced_calibration_value};

mod utils;

pub fn execute_day_01(lines_buffer: Lines<BufReader<File>>) {
  let mut accumulator: u32 = 0;
  for line_buffer in lines_buffer {
    match line_buffer {
      Ok(line) => {
        accumulator += get_calibration_value(&line[..]);
      }
      Err(why) => panic!("{:#?}", why),
    };
  }

  println!("{}", accumulator);
}

pub fn execute_day_01_part_02(lines_buffer: Lines<BufReader<File>>) {
  let mut accumulator: u32 = 0;
  for line_buffer in lines_buffer {
    match line_buffer {
      Ok(line) => {
        accumulator += get_enhanced_calibration_value(&line[..]);
      }
      Err(why) => panic!("{:#?}", why),
    };
  }

  println!("{}", accumulator);
}
