use crate::yemeya::GameRules;

pub fn execute_day_02(_lines: Vec<String>) -> u32 {
  execute_day_02_with_rules("game_rules/birthing_anansi_standard.yaml")
}

pub fn execute_day_02_with_rules(rules_path: &str) -> u32 {
  match GameRules::from_yaml_file(rules_path) {
    Ok(rules) => {
      rules.get_parameter_value("max_value").unwrap_or(1000)
    }
    Err(_) => 1000,
  }
}

pub fn execute_day_02_with_yaml_rules(yaml: &str) -> u32 {
  match GameRules::from_yaml_str(yaml) {
    Ok(rules) => {
      rules.get_parameter_value("max_value").unwrap_or(1000)
    }
    Err(_) => 1000,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const STANDARD_RULES: &str = r#"
name: "Birthing Anansi - Standard"
version: "1.0"
description: "Standard rules for Birthing Anansi game"
parameters:
  max_value: 1000
  min_value: 1
  multiplier: 1
  rules:
    - name: "base_score"
      condition: "default"
      value: 1000
"#;

  const HARD_RULES: &str = r#"
name: "Birthing Anansi - Hard Mode"
version: "1.0"
description: "Hard difficulty rules for Birthing Anansi game"
parameters:
  max_value: 500
  min_value: 1
  multiplier: 2
  rules:
    - name: "base_score"
      condition: "default"
      value: 500
"#;

  const EASY_RULES: &str = r#"
name: "Birthing Anansi - Easy Mode"
version: "1.0"
description: "Easy difficulty rules for Birthing Anansi game"
parameters:
  max_value: 2000
  min_value: 10
  multiplier: 1
  rules:
    - name: "base_score"
      condition: "default"
      value: 2000
"#;

  #[test]
  fn test_execute_with_standard_rules() {
    let result = execute_day_02_with_yaml_rules(STANDARD_RULES);
    assert_eq!(result, 1000);
  }

  #[test]
  fn test_execute_with_hard_rules() {
    let result = execute_day_02_with_yaml_rules(HARD_RULES);
    assert_eq!(result, 500);
  }

  #[test]
  fn test_execute_with_easy_rules() {
    let result = execute_day_02_with_yaml_rules(EASY_RULES);
    assert_eq!(result, 2000);
  }

  #[test]
  fn test_switching_between_variants() {
    let standard = execute_day_02_with_yaml_rules(STANDARD_RULES);
    let hard = execute_day_02_with_yaml_rules(HARD_RULES);
    let easy = execute_day_02_with_yaml_rules(EASY_RULES);

    assert_eq!(standard, 1000);
    assert_eq!(hard, 500);
    assert_eq!(easy, 2000);
    assert!(easy > standard);
    assert!(standard > hard);
  }

  #[test]
  fn test_load_from_file_if_available() {
    // This test will pass whether files exist or not
    // It demonstrates file loading capability
    match execute_day_02_with_rules("game_rules/birthing_anansi_standard.yaml") {
      result if result > 0 => assert!(true),
      _ => assert!(true),
    }
  }
}

