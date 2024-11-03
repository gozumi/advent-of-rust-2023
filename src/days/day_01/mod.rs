use std::{
  fs::File,
  io::{BufReader, Lines},
};

pub fn execute_day_01(lines_buffer: Lines<BufReader<File>>) {
  for line in lines_buffer {
    match line {
      Ok(bar) => println!("{}", bar),
      Err(why) => panic!("{:#?}", why),
    };
  }
}
