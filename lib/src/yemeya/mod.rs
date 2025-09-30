use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

mod examples;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRules {
    pub name: String,
    pub version: String,
    pub description: String,
    pub parameters: GameParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameParameters {
    pub max_value: Option<u32>,
    pub min_value: Option<u32>,
    pub multiplier: Option<u32>,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub condition: String,
    pub value: u32,
}

impl GameRules {
    pub fn from_yaml_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read YAML file: {}", e))?;
        
        Self::from_yaml_str(&contents)
    }

    pub fn from_yaml_str(yaml: &str) -> Result<Self, String> {
        serde_yaml::from_str(yaml)
            .map_err(|e| format!("Failed to parse YAML: {}", e))
    }

    pub fn get_parameter_value(&self, key: &str) -> Option<u32> {
        match key {
            "max_value" => self.parameters.max_value,
            "min_value" => self.parameters.min_value,
            "multiplier" => self.parameters.multiplier,
            _ => None,
        }
    }

    pub fn get_rules(&self) -> &[Rule] {
        &self.parameters.rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_yaml_rules() {
        let yaml = r#"
name: "Test Game"
version: "1.0"
description: "A test game variant"
parameters:
  max_value: 1000
  min_value: 1
  multiplier: 2
  rules:
    - name: "basic_rule"
      condition: "always"
      value: 100
"#;

        let rules = GameRules::from_yaml_str(yaml).unwrap();
        assert_eq!(rules.name, "Test Game");
        assert_eq!(rules.version, "1.0");
        assert_eq!(rules.parameters.max_value, Some(1000));
        assert_eq!(rules.parameters.rules.len(), 1);
        assert_eq!(rules.parameters.rules[0].name, "basic_rule");
    }

    #[test]
    fn test_get_parameter_value() {
        let yaml = r#"
name: "Test Game"
version: "1.0"
description: "A test game"
parameters:
  max_value: 500
  min_value: 10
  multiplier: 3
  rules: []
"#;

        let rules = GameRules::from_yaml_str(yaml).unwrap();
        assert_eq!(rules.get_parameter_value("max_value"), Some(500));
        assert_eq!(rules.get_parameter_value("min_value"), Some(10));
        assert_eq!(rules.get_parameter_value("multiplier"), Some(3));
        assert_eq!(rules.get_parameter_value("unknown"), None);
    }
}
