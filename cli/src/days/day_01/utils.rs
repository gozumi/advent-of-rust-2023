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

pub fn get_enhanced_calibration_value(line: &str) -> u32 {
  let mut first_digit: char = '_';
  let mut last_digit: char = '_';
  let mut potential_digit_string = String::from("");

  for c in line.chars() {
    potential_digit_string.push(c);

    if first_digit == '_' {
      if c.is_digit(10) {
        first_digit = c;
        potential_digit_string = String::from("");
      } else {
        first_digit = match get_digit_string(&(potential_digit_string.clone())[..]) {
          Some(c) => {
            potential_digit_string = String::from("");
            last_digit = c.1;
            c.1
          }
          None => '_',
        };
      }
    }

    if c.is_digit(10) {
      potential_digit_string = String::from("");
      last_digit = c;
    } else {
      last_digit = match get_digit_string(&(potential_digit_string.clone())[..]) {
        Some(c) => {
          potential_digit_string = String::from("");
          c.1
        }
        None => last_digit,
      };
    }
  }

  format!("{}{}", first_digit, last_digit).parse::<u32>().unwrap()
}

fn get_digit_string(potential: &str) -> Option<(&str, char)> {
  let digit_strings = vec![
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
  ];
  let mut matched_digit: Option<(&str, char)> = None;

  for digit_string in digit_strings {
    if potential.contains(digit_string.0) {
      matched_digit = Some(digit_string);
      break;
    }
  }

  return matched_digit;
}

#[cfg(test)]
mod test {

  use crate::days::day_01::utils::{
    get_calibration_value, get_digit_string, get_enhanced_calibration_value,
  };

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

  #[test]
  fn get_digit_string_returns_the_correct_strings() {
    let potential = "rstonestst";
    let digit_string = get_digit_string(potential);
    assert_eq!(digit_string, Some(("one", '1')));

    let potential = "rstontwoestst";
    let digit_string = get_digit_string(potential);
    assert_eq!(digit_string, Some(("two", '2')));

    let potential = "rstonthreeestst";
    let digit_string = get_digit_string(potential);
    assert_eq!(digit_string, Some(("three", '3')));

    let potential = "rstonfouroestst";
    let digit_string = get_digit_string(potential);
    assert_eq!(digit_string, Some(("four", '4')));

    let potential = "rstonfiveestst";
    let digit_string = get_digit_string(potential);
    assert_eq!(digit_string, Some(("five", '5')));

    let potential = "rstonthsixeestst";
    let digit_string = get_digit_string(potential);
    assert_eq!(digit_string, Some(("six", '6')));

    let potential = "rstonfosevenuroestst";
    let digit_string = get_digit_string(potential);
    assert_eq!(digit_string, Some(("seven", '7')));

    let potential = "rstontheighteestst";
    let digit_string = get_digit_string(potential);
    assert_eq!(digit_string, Some(("eight", '8')));

    let potential = "rstonfosninenuroestst";
    let digit_string = get_digit_string(potential);
    assert_eq!(digit_string, Some(("nine", '9')));
  }

  #[test]
  fn get_enhanced_calibration_value_should_return_the_correct_values() {
    let input_line = "atobcthroneefgfiexyz";
    let calibration_value = get_enhanced_calibration_value(input_line);
    assert_eq!(calibration_value, 11);

    let input_line = "atobcthrtwoefgfiexyz";
    let calibration_value = get_enhanced_calibration_value(input_line);
    assert_eq!(calibration_value, 22);

    let input_line = "atobcthreefgfiexyz";
    let calibration_value = get_enhanced_calibration_value(input_line);
    assert_eq!(calibration_value, 33);

    let input_line = "atobcthrfourfgfiexyz";
    let calibration_value = get_enhanced_calibration_value(input_line);
    assert_eq!(calibration_value, 44);

    let input_line = "atobctfivefgfiexyz";
    let calibration_value = get_enhanced_calibration_value(input_line);
    assert_eq!(calibration_value, 55);

    let input_line = "atobcthrsixefgfiexyz";
    let calibration_value = get_enhanced_calibration_value(input_line);
    assert_eq!(calibration_value, 66);

    let input_line = "atobcsevenfgfiexyz";
    let calibration_value = get_enhanced_calibration_value(input_line);
    assert_eq!(calibration_value, 77);

    let input_line = "atobcthreightfgfiexyz";
    let calibration_value = get_enhanced_calibration_value(input_line);
    assert_eq!(calibration_value, 88);

    let input_line = "atobctninefgfiexyz";
    let calibration_value = get_enhanced_calibration_value(input_line);
    assert_eq!(calibration_value, 99);

    let input_line = "atobctninefgfieightxyz";
    let calibration_value = get_enhanced_calibration_value(input_line);
    assert_eq!(calibration_value, 98);

    let input_line = "atobctninef4gfieightxyz";
    let calibration_value = get_enhanced_calibration_value(input_line);
    assert_eq!(calibration_value, 98);

    let input_line = "a2tobctninef4gfieightxyz";
    let calibration_value = get_enhanced_calibration_value(input_line);
    assert_eq!(calibration_value, 28);

    let input_line = "atobctninef4gfieightxy2z";
    let calibration_value = get_enhanced_calibration_value(input_line);
    assert_eq!(calibration_value, 92);

    let input_line = "atobctnnef4gfieghtxy2z";
    let calibration_value = get_enhanced_calibration_value(input_line);
    assert_eq!(calibration_value, 42);

    let input_line = "atobctnnef4gfieghtxyz";
    let calibration_value = get_enhanced_calibration_value(input_line);
    assert_eq!(calibration_value, 44);

    let input_line = "sevenatobctnnef4gfieghtsix";
    let calibration_value = get_enhanced_calibration_value(input_line);
    assert_eq!(calibration_value, 76);
  }
}
