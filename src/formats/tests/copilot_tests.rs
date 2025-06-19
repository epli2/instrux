#[cfg(test)]
mod tests {
    use crate::formats::{FromFormat, ToFormat, copilot::CopilotConverter, copilot::CopilotParser};
    use crate::model::types::{
        InstructionItem, InstructionItemVariant0Targets, InstruxConfiguration, Targets,
    };
    use std::collections::HashMap;

    // Helper function to create a sample configuration
    fn create_test_config() -> InstruxConfiguration {
        let instruction1 = InstructionItem::Variant0 {
            title: "Sample Instruction".to_string(),
            body: "This is a sample instruction body.".to_string(),
            description: Some("Description of the instruction".to_string()),
            disable: false,
            targets: InstructionItemVariant0Targets::Variant1("all".to_string()),
        };

        let instruction2 = InstructionItem::Variant0 {
            title: "Copilot Specific Instruction".to_string(),
            body: "This instruction is specific to Copilot.".to_string(),
            description: None,
            disable: false,
            targets: InstructionItemVariant0Targets::Variant0(vec![Targets::Copilot]),
        };

        let mut targets_map = HashMap::new();
        targets_map.insert(Targets::Copilot, Default::default());

        // Create the version with a valid string pattern
        let version = "0.1.0".parse().expect("Valid version string");

        InstruxConfiguration {
            instructions: vec![instruction1, instruction2],
            language: Default::default(),
            targets: targets_map,
            version,
        }
    }

    #[test]
    fn test_copilot_converter_to_format() {
        let config = create_test_config();
        let converter = CopilotConverter {};

        let result = converter.to_format(&config);
        assert!(result.is_ok());

        let format_result = result.unwrap();
        let output = match format_result {
            crate::formats::FormatResult::Single(text) => text,
            _ => panic!("Expected FormatResult::Single"),
        };

        // Check if output contains expected sections
        assert!(output.contains("# Copilot Instructions"));
        assert!(output.contains("## Sample Instruction"));
        assert!(output.contains("This is a sample instruction body."));
        assert!(output.contains("## Copilot Specific Instruction"));
        assert!(output.contains("This instruction is specific to Copilot."));
    }

    #[test]
    fn test_copilot_parser_from_format() {
        let sample_content = r#"# Copilot Instructions

## Sample Instruction

This is a sample instruction body.

## Copilot Specific Instruction

This instruction is specific to Copilot.
"#;

        let result = CopilotParser::from_format(sample_content);

        assert!(result.is_ok());

        let instructions = result.unwrap();
        assert_eq!(instructions.len(), 2);

        match &instructions[0] {
            InstructionItem::Variant0 { title, body, .. } => {
                assert_eq!(title, "Sample Instruction");
                assert!(body.contains("This is a sample instruction body."));
            }
            _ => panic!("Expected Variant0"),
        }

        match &instructions[1] {
            InstructionItem::Variant0 { title, body, .. } => {
                assert_eq!(title, "Copilot Specific Instruction");
                assert!(body.contains("This instruction is specific to Copilot."));
            }
            _ => panic!("Expected Variant0"),
        }
    }
}
