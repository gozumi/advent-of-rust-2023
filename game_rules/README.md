# Game Rules Configuration

This directory contains YAML configuration files for game variants supported by the yemeya library.

## Overview

The yemeya library provides a flexible system for defining and loading game rules from YAML files. This allows you to:
- Define multiple game variants without changing code
- Easily switch between different rule sets
- Create custom game modes by editing YAML files
- Maintain game rules in a centralized, version-controlled location

## YAML Schema

Each game rules file follows this structure:

```yaml
name: "Game Name"
version: "1.0"
description: "Description of the game variant"
parameters:
  max_value: 1000        # Optional: Maximum allowed value
  min_value: 1           # Optional: Minimum allowed value
  multiplier: 1          # Optional: Score multiplier
  rules:                 # List of game rules
    - name: "rule_name"
      condition: "when_to_apply"
      value: 100
```

### Required Fields

- `name` (string): Human-readable name of the game variant
- `version` (string): Version number (e.g., "1.0", "2.1")
- `description` (string): Brief description of what makes this variant unique
- `parameters` (object): Container for all game parameters
- `parameters.rules` (array): List of rule definitions

### Optional Parameters

- `parameters.max_value` (integer): Maximum value constraint
- `parameters.min_value` (integer): Minimum value constraint
- `parameters.multiplier` (integer): Multiplier applied to scores

### Rule Definition

Each rule in the `rules` array must have:
- `name` (string): Identifier for the rule
- `condition` (string): When the rule applies (e.g., "default", "score_above", "score_below")
- `value` (integer): Numeric value associated with the rule

## Example Variants

### Standard Mode
File: `birthing_anansi_standard.yaml`

The default, balanced game experience:
- Max value: 1000
- Multiplier: 1x
- Suitable for most players

### Hard Mode
File: `birthing_anansi_hard.yaml`

A challenging variant:
- Max value: 500 (reduced)
- Multiplier: 2x
- Higher skill requirements

### Easy Mode
File: `birthing_anansi_easy.yaml`

A relaxed game experience:
- Max value: 2000 (increased)
- Min value: 10
- Lower difficulty

## Using Game Rules in Code

### Loading Rules from a File

```rust
use lib::yemeya::GameRules;

// Load rules from a YAML file
let rules = GameRules::from_yaml_file("game_rules/birthing_anansi_standard.yaml")
    .expect("Failed to load rules");

// Access parameters
let max_value = rules.get_parameter_value("max_value");
let multiplier = rules.get_parameter_value("multiplier");

// Get all rules
let game_rules = rules.get_rules();
```

### Loading Rules from a String

```rust
use lib::yemeya::GameRules;

let yaml = r#"
name: "Custom Game"
version: "1.0"
description: "My custom variant"
parameters:
  max_value: 1500
  rules: []
"#;

let rules = GameRules::from_yaml_str(yaml)
    .expect("Failed to parse YAML");
```

### Switching Between Variants

```rust
// Use standard rules
let result = execute_game_with_rules("game_rules/birthing_anansi_standard.yaml");

// Switch to hard mode
let result = execute_game_with_rules("game_rules/birthing_anansi_hard.yaml");

// Try easy mode
let result = execute_game_with_rules("game_rules/birthing_anansi_easy.yaml");
```

## Creating New Variants

To create a new game variant:

1. Copy an existing YAML file as a starting point:
   ```bash
   cp birthing_anansi_standard.yaml birthing_anansi_custom.yaml
   ```

2. Edit the new file:
   - Update the `name` field
   - Modify `description` to explain your variant
   - Adjust `parameters` values as needed
   - Add, remove, or modify rules

3. Test your variant:
   ```rust
   let rules = GameRules::from_yaml_file("game_rules/birthing_anansi_custom.yaml")
       .expect("Failed to load custom rules");
   ```

4. Validate the YAML syntax:
   - Ensure proper indentation (use spaces, not tabs)
   - Check that all required fields are present
   - Verify numeric values are valid

## Best Practices

1. **Version Control**: Always increment the `version` field when making changes
2. **Documentation**: Provide clear descriptions of what makes each variant unique
3. **Testing**: Test new variants thoroughly before deployment
4. **Naming**: Use descriptive names that indicate the variant's characteristics
5. **Compatibility**: Maintain backward compatibility when updating existing variants

## Troubleshooting

### "Failed to read YAML file"
- Check that the file path is correct
- Ensure the file exists and is readable
- Verify file permissions

### "Failed to parse YAML"
- Check YAML syntax (indentation, colons, dashes)
- Validate with a YAML linter
- Ensure all required fields are present

### Rules Not Loading
- Verify the file is in the correct location
- Check that the application has the correct working directory
- Look for error messages in the console

## Advanced Usage

### Dynamic Rule Selection

You can implement logic to select rules based on user preferences or game state:

```rust
fn get_rules_path(difficulty: &str) -> &str {
    match difficulty {
        "easy" => "game_rules/birthing_anansi_easy.yaml",
        "hard" => "game_rules/birthing_anansi_hard.yaml",
        _ => "game_rules/birthing_anansi_standard.yaml",
    }
}

let rules_path = get_rules_path(user_preference);
let rules = GameRules::from_yaml_file(rules_path)?;
```

### Merging Rule Sets

You can load multiple rule files and merge their configurations programmatically to create hybrid variants.

## Contributing

When adding new game variants:
1. Follow the YAML schema defined above
2. Add documentation explaining the variant
3. Include test cases that verify the rules load correctly
4. Update this README with information about your variant
