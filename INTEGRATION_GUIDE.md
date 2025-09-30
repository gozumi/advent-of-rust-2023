# Yemeya Library Integration Guide

This guide demonstrates how the yemeya library has been integrated into the project and how to use it for game variant management.

## Overview

The yemeya library provides YAML-based configuration for game rules, enabling:
- **Variant Management**: Switch between different game modes without code changes
- **Easy Extensibility**: Add new variants by creating YAML files
- **Centralized Configuration**: All game rules in one location
- **Type Safety**: Rust structs ensure valid rule structures

## Architecture

```
advent-of-rust-2023/
├── lib/
│   └── src/
│       ├── yemeya/           # Rule loading library
│       │   ├── mod.rs        # Core types and loading logic
│       │   └── examples.rs   # Usage examples
│       └── days/
│           └── day_02/       # Example integration
│               └── mod.rs    # Uses yemeya to load rules
├── game_rules/               # YAML configuration files
│   ├── README.md            # Rules documentation
│   ├── birthing_anansi_standard.yaml
│   ├── birthing_anansi_hard.yaml
│   └── birthing_anansi_easy.yaml
└── INTEGRATION_GUIDE.md     # This file
```

## Integration Example: day_02 Module

The `day_02` module demonstrates full integration with yemeya:

### Basic Usage

```rust
use crate::yemeya::GameRules;

// Load rules from file
pub fn execute_day_02(_lines: Vec<String>) -> u32 {
    execute_day_02_with_rules("game_rules/birthing_anansi_standard.yaml")
}

// Allow rule file to be specified
pub fn execute_day_02_with_rules(rules_path: &str) -> u32 {
    match GameRules::from_yaml_file(rules_path) {
        Ok(rules) => {
            // Use the loaded rules
            rules.get_parameter_value("max_value").unwrap_or(1000)
        }
        Err(_) => 1000, // Fallback value
    }
}
```

### Advanced Usage: Rule-Based Logic

```rust
pub fn execute_game_with_rules(rules_path: &str, input: &[String]) -> u32 {
    let rules = GameRules::from_yaml_file(rules_path)
        .expect("Failed to load game rules");
    
    // Use parameters from rules
    let max_value = rules.get_parameter_value("max_value").unwrap_or(1000);
    let multiplier = rules.get_parameter_value("multiplier").unwrap_or(1);
    
    // Process input based on rules
    let mut score = 0;
    for line in input {
        let value = process_line(line);
        score += value.min(max_value) * multiplier;
    }
    
    // Apply rule conditions
    for rule in rules.get_rules() {
        match rule.condition.as_str() {
            "score_above" if score > rule.value => {
                score += 100; // Bonus
            }
            "score_below" if score < rule.value => {
                score = score.saturating_sub(50); // Penalty
            }
            _ => {}
        }
    }
    
    score
}
```

## CLI Integration Pattern

For command-line applications, you can add variant selection:

```rust
// In cli/src/main.rs (example pattern)
use std::env;
use lib::yemeya::GameRules;
use lib::days::day_02::execute_day_02_with_rules;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Allow variant selection via command line
    let variant = args.get(2).map(|s| s.as_str()).unwrap_or("standard");
    
    let rules_path = match variant {
        "easy" => "game_rules/birthing_anansi_easy.yaml",
        "hard" => "game_rules/birthing_anansi_hard.yaml",
        _ => "game_rules/birthing_anansi_standard.yaml",
    };
    
    let result = execute_day_02_with_rules(rules_path);
    println!("Result with {} rules: {}", variant, result);
}
```

### Example CLI Usage

```bash
# Use standard rules (default)
cargo run --bin cli 2

# Use easy mode
cargo run --bin cli 2 easy

# Use hard mode
cargo run --bin cli 2 hard
```

## Web Application Integration

For the Leptos web application, you can add variant selection:

```rust
use leptos::*;
use lib::yemeya::GameRules;

#[component]
pub fn GameVariantSelector() -> impl IntoView {
    let (selected_variant, set_selected_variant) = create_signal("standard".to_string());
    let (rules_info, set_rules_info) = create_signal(None::<String>);
    
    let load_variant = move |variant: String| {
        let rules_path = match variant.as_str() {
            "easy" => "game_rules/birthing_anansi_easy.yaml",
            "hard" => "game_rules/birthing_anansi_hard.yaml",
            _ => "game_rules/birthing_anansi_standard.yaml",
        };
        
        // In a real app, you'd load from a resource or embed
        // For now, this shows the pattern
        set_rules_info(Some(format!("Loaded: {}", rules_path)));
    };
    
    view! {
        <div class="variant-selector">
            <h3>"Select Game Variant"</h3>
            <select on:change=move |ev| {
                let value = event_target_value(&ev);
                set_selected_variant(value.clone());
                load_variant(value);
            }>
                <option value="standard">"Standard"</option>
                <option value="easy">"Easy"</option>
                <option value="hard">"Hard"</option>
            </select>
            <p>{move || rules_info.get()}</p>
        </div>
    }
}
```

## Testing Variants

The library includes comprehensive tests demonstrating usage:

### Running Tests

```bash
# Run all tests
cargo test --lib

# Run specific test module
cargo test --lib yemeya

# Run example tests with output
cargo test --lib yemeya::examples -- --nocapture
```

### Test Coverage

- ✅ YAML parsing and validation
- ✅ Parameter access
- ✅ Rule loading from files
- ✅ Rule loading from strings
- ✅ Variant switching
- ✅ Dynamic variant selection
- ✅ Error handling

## Creating New Variants

### Step 1: Create YAML File

```yaml
name: "My Custom Variant"
version: "1.0"
description: "A custom game mode"
parameters:
  max_value: 1500
  min_value: 5
  multiplier: 1
  rules:
    - name: "custom_rule"
      condition: "custom_condition"
      value: 750
```

### Step 2: Save to game_rules/

```bash
# Save as: game_rules/birthing_anansi_custom.yaml
```

### Step 3: Use in Code

```rust
let result = execute_day_02_with_rules(
    "game_rules/birthing_anansi_custom.yaml"
);
```

### Step 4: Test

```rust
#[test]
fn test_custom_variant() {
    let yaml = include_str!("../../game_rules/birthing_anansi_custom.yaml");
    let rules = GameRules::from_yaml_str(yaml).unwrap();
    assert_eq!(rules.get_parameter_value("max_value"), Some(1500));
}
```

## Best Practices

### 1. Error Handling

Always handle errors gracefully:

```rust
match GameRules::from_yaml_file(path) {
    Ok(rules) => {
        // Use rules
    }
    Err(e) => {
        eprintln!("Failed to load rules: {}", e);
        // Use defaults or return error
    }
}
```

### 2. Validation

Validate loaded rules:

```rust
let rules = GameRules::from_yaml_file(path)?;

// Ensure sensible values
if let Some(max) = rules.get_parameter_value("max_value") {
    if let Some(min) = rules.get_parameter_value("min_value") {
        assert!(max > min, "max_value must be greater than min_value");
    }
}
```

### 3. Default Values

Always provide fallbacks:

```rust
let max_value = rules.get_parameter_value("max_value").unwrap_or(1000);
```

### 4. Documentation

Document which rules your code uses:

```rust
/// Executes the game using rules from the specified YAML file.
/// 
/// # Rules Used
/// - `max_value`: Maximum allowed score
/// - `multiplier`: Score multiplier
/// - Rules with condition "score_above" for bonuses
/// 
/// # Example
/// ```
/// let result = execute_with_rules("game_rules/standard.yaml");
/// ```
pub fn execute_with_rules(path: &str) -> u32 {
    // Implementation
}
```

## Troubleshooting

### Problem: "Failed to read YAML file"

**Solution**: Check file path and working directory
```bash
# Check current directory
pwd

# Verify file exists
ls -la game_rules/
```

### Problem: "Failed to parse YAML"

**Solution**: Validate YAML syntax
```bash
# Use online validator or:
python3 -c "import yaml; yaml.safe_load(open('game_rules/file.yaml'))"
```

### Problem: Tests can't find files

**Solution**: Use embedded strings in tests
```rust
const RULES: &str = include_str!("../../game_rules/file.yaml");
let rules = GameRules::from_yaml_str(RULES)?;
```

## Performance Considerations

### Caching Rules

For frequently accessed rules, cache them:

```rust
use std::sync::OnceLock;

static STANDARD_RULES: OnceLock<GameRules> = OnceLock::new();

fn get_standard_rules() -> &'static GameRules {
    STANDARD_RULES.get_or_init(|| {
        GameRules::from_yaml_file("game_rules/birthing_anansi_standard.yaml")
            .expect("Failed to load standard rules")
    })
}
```

### Embedded Rules

For production builds, embed rules in the binary:

```rust
const STANDARD_RULES: &str = include_str!("../../game_rules/birthing_anansi_standard.yaml");

fn load_embedded_rules() -> GameRules {
    GameRules::from_yaml_str(STANDARD_RULES)
        .expect("Embedded rules should always be valid")
}
```

## Future Enhancements

Potential improvements to the system:

1. **Rule Composition**: Merge multiple rule files
2. **Schema Validation**: Validate rules against a JSON schema
3. **Hot Reloading**: Watch for file changes and reload
4. **Rule Inheritance**: Extend base rules with variants
5. **Conditional Parameters**: Support expressions in rules
6. **Rule Auditing**: Log which rules are applied

## Conclusion

The yemeya library provides a robust foundation for managing game rules through YAML configuration. By following the patterns in this guide, you can easily add support for multiple game variants and maintain clean separation between code and configuration.

For more information, see:
- `game_rules/README.md` - YAML schema documentation
- `lib/src/yemeya/mod.rs` - Implementation details
- `lib/src/yemeya/examples.rs` - Usage examples
- `lib/src/days/day_02/mod.rs` - Integration example
