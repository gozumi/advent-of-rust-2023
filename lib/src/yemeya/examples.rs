/// Example usage of the yemeya library for loading game rules

#[cfg(test)]
mod tests {
    use crate::yemeya::GameRules;

    #[test]
    fn example_loading_and_comparing_variants() {
        let variants = vec![
            ("standard", include_str!("../../../game_rules/birthing_anansi_standard.yaml")),
            ("hard", include_str!("../../../game_rules/birthing_anansi_hard.yaml")),
            ("easy", include_str!("../../../game_rules/birthing_anansi_easy.yaml")),
        ];

        println!("\n=== Game Rules Comparison ===\n");

        for (name, yaml) in variants {
            match GameRules::from_yaml_str(yaml) {
                Ok(rules) => {
                    println!("Variant: {}", name);
                    println!("  Name: {}", rules.name);
                    println!("  Description: {}", rules.description);
                    println!("  Max Value: {:?}", rules.get_parameter_value("max_value"));
                    println!("  Min Value: {:?}", rules.get_parameter_value("min_value"));
                    println!("  Multiplier: {:?}", rules.get_parameter_value("multiplier"));
                    println!("  Rules Count: {}", rules.get_rules().len());
                    println!();
                }
                Err(e) => println!("Failed to load {}: {}", name, e),
            }
        }
    }

    #[test]
    fn example_rule_validation() {
        let yaml = r#"
name: "Test Validation"
version: "1.0"
description: "Testing rule validation"
parameters:
  max_value: 100
  min_value: 10
  rules:
    - name: "test_rule"
      condition: "test"
      value: 50
"#;

        let rules = GameRules::from_yaml_str(yaml).unwrap();
        
        // Validate max > min
        let max = rules.get_parameter_value("max_value").unwrap_or(0);
        let min = rules.get_parameter_value("min_value").unwrap_or(0);
        assert!(max > min, "Max value must be greater than min value");

        // Validate rules exist
        assert!(!rules.get_rules().is_empty(), "Rules should not be empty");

        println!("\n=== Validation Passed ===");
        println!("Max ({}) > Min ({}): ✓", max, min);
        println!("Rules present: ✓");
    }

    #[test]
    fn example_dynamic_rule_selection() {
        fn select_variant_by_skill_level(skill_level: u32) -> &'static str {
            match skill_level {
                0..=3 => include_str!("../../../game_rules/birthing_anansi_easy.yaml"),
                4..=6 => include_str!("../../../game_rules/birthing_anansi_standard.yaml"),
                _ => include_str!("../../../game_rules/birthing_anansi_hard.yaml"),
            }
        }

        println!("\n=== Dynamic Rule Selection Example ===\n");

        for skill in [2, 5, 8] {
            let yaml = select_variant_by_skill_level(skill);
            let rules = GameRules::from_yaml_str(yaml).unwrap();
            println!("Skill level {}: Selected '{}'", skill, rules.name);
        }
    }
}
