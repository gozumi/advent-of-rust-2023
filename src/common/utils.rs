use std::{
  fs::File,
  io::{self, BufRead, BufReader, Lines},
};

pub fn get_file_lines(file_path: String) -> Lines<BufReader<File>> {
  return match File::open(file_path) {
    Err(why) => panic!("{:?}", why),
    Ok(file) => io::BufReader::new(file).lines(),
  };
}
