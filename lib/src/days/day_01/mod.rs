use utils::{get_calibration_value, get_enhanced_calibration_value};

mod utils;

pub fn execute_day_01(lines: Vec<String>) -> u32 {
  let mut accumulator: u32 = 0;
  for line in lines {
    accumulator += get_calibration_value(&line[..]);
  }

  accumulator
}

pub fn execute_day_01_part_02(lines: Vec<String>) -> u32 {
  let mut accumulator: u32 = 0;
  for line in lines {
    accumulator += get_enhanced_calibration_value(&line[..]);
  }

  accumulator
}
