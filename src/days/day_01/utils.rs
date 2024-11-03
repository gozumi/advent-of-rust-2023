pub fn get_calibration_value(line: &str) -> u32 {
  let mut first_digit: char = '_';
  let mut last_digit: char = '_';

  for c in line.chars() {
    if first_digit == '_' && c.is_digit(10) {
      first_digit = c;
    }

    if c.is_digit(10) {
      last_digit = c;
    }
  }

  format!("{}{}", first_digit, last_digit).parse::<u32>().unwrap()
}

#[cfg(test)]
mod test {

  use crate::days::day_01::utils::get_calibration_value;

  #[test]
  fn get_calibration_value_should_work_when_2_digits_are_in_the_string() {
    let input_line = "abc2defg1xyz";
    let calibration_value = get_calibration_value(input_line);

    assert_eq!(calibration_value, 21);
  }

  #[test]
  fn get_calibration_value_should_work_when_3_digits_are_in_the_string() {
    let input_line = "abc2de4fg1xyz";
    let calibration_value = get_calibration_value(input_line);

    assert_eq!(calibration_value, 21);
  }

  #[test]
  fn get_calibration_value_should_work_when_1_digit_is_in_the_string() {
    let input_line = "abcssde4fgsxyz";
    let calibration_value = get_calibration_value(input_line);

    assert_eq!(calibration_value, 44);
  }
}
