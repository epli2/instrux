#[cfg(test)]
mod tests {
    use crate::formats::{FromFormat, ToFormat, cline::ClineConverter, cline::ClineParser};
    use crate::model::types::{
        InstructionItem, InstructionItemVariant0Targets, InstruxConfiguration, Targets,
    };
    use std::collections::HashMap;

    fn create_test_config() -> InstruxConfiguration {
        let instruction1 = InstructionItem::Variant0 {
            title: "Sample Instruction".to_string(),
            body: "This is a sample instruction body.".to_string(),
            description: None,
            disable: false,
            targets: InstructionItemVariant0Targets::Variant1("all".to_string()),
        };

        let instruction2 = InstructionItem::Variant0 {
            title: "Cline Specific Instruction".to_string(),
            body: "This instruction is specific to Cline.".to_string(),
            description: None,
            disable: false,
            targets: InstructionItemVariant0Targets::Variant0(vec![Targets::Cline]),
        };

        let mut targets_map = HashMap::new();
        targets_map.insert(Targets::Cline, Default::default());

        let version = "0.1.0".parse().expect("Valid version string");

        InstruxConfiguration {
            instructions: vec![instruction1, instruction2],
            language: Default::default(),
            targets: targets_map,
            version,
        }
    }

    #[test]
    fn test_cline_converter_to_format() {
        let config = create_test_config();
        let converter = ClineConverter {};

        let result = converter.to_format(&config);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("# Cline Rules"));
        assert!(output.contains("## Sample Instruction"));
        assert!(output.contains("This is a sample instruction body."));
        assert!(output.contains("## Cline Specific Instruction"));
        assert!(output.contains("This instruction is specific to Cline."));
    }

    #[test]
    fn test_cline_parser_from_format() {
        let sample_content = r#"# Cline Rules

## Sample Instruction

This is a sample instruction body.

## Cline Specific Instruction

This instruction is specific to Cline.
"#;

        let result = ClineParser::from_format(sample_content);
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
                assert_eq!(title, "Cline Specific Instruction");
                assert!(body.contains("This instruction is specific to Cline."));
            }
            _ => panic!("Expected Variant0"),
        }
    }
}
