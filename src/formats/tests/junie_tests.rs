#[cfg(test)]
mod tests {
    use crate::formats::{FromFormat, ToFormat, junie::JunieConverter, junie::JunieParser};
    use crate::model::types::{
        InstructionItem, InstructionItemVariant0Targets, InstruxConfiguration, Targets,
    };
    use std::collections::HashMap;

    fn create_test_config() -> InstruxConfiguration {
        let instruction1 = InstructionItem::Variant0 {
            title: "Sample Instruction".to_string(),
            body: "This is a sample instruction body.".to_string(),
            description: Some("Description of the instruction".to_string()),
            disable: false,
            targets: InstructionItemVariant0Targets::Variant1("all".to_string()),
        };

        let instruction2 = InstructionItem::Variant0 {
            title: "Junie Specific Instruction".to_string(),
            body: "This instruction is specific to Junie.".to_string(),
            description: None,
            disable: false,
            targets: InstructionItemVariant0Targets::Variant0(vec![Targets::Junie]),
        };

        let mut targets_map = HashMap::new();
        targets_map.insert(Targets::Junie, Default::default());

        let version = "0.1.0".parse().expect("Valid version string");

        InstruxConfiguration {
            instructions: vec![instruction1, instruction2],
            language: Default::default(),
            targets: targets_map,
            version,
        }
    }

    #[test]
    fn test_junie_converter_to_format() {
        let config = create_test_config();
        let converter = JunieConverter {};

        let result = converter.to_format(&config);
        assert!(result.is_ok());

        let format_result = result.unwrap();
        let output = match format_result {
            crate::formats::FormatResult::Single(text) => text,
            _ => panic!("Expected FormatResult::Single"),
        };
        assert!(output.contains("# Junie Guidelines"));
        assert!(output.contains("## Sample Instruction"));
        assert!(output.contains("This is a sample instruction body."));
        assert!(output.contains("## Junie Specific Instruction"));
        assert!(output.contains("This instruction is specific to Junie."));
    }

    #[test]
    fn test_junie_parser_from_format() {
        let sample_content = r#"# Junie Guidelines

## Sample Instruction

This is a sample instruction body.

## Junie Specific Instruction

This instruction is specific to Junie.
"#;

        let result = JunieParser::from_format(sample_content);
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
                assert_eq!(title, "Junie Specific Instruction");
                assert!(body.contains("This instruction is specific to Junie."));
            }
            _ => panic!("Expected Variant0"),
        }
    }
}
