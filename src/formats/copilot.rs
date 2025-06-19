use super::{FormatResult, FromFormat, ToFormat};
use crate::formats::common;
use crate::formats::common::TargetsChecker;
use crate::model::types::{InstructionItem, InstruxConfiguration, Targets};
use std::path::PathBuf;

/// Converter for Copilot format (copilot-instructions.md)
pub struct CopilotConverter {}

impl ToFormat for CopilotConverter {
    fn to_format(&self, config: &InstruxConfiguration) -> Result<FormatResult, String> {
        let mut output = String::new();

        // Add header section with metadata
        output.push_str("# Copilot Instructions\n\n");

        common::process_instructions_common(
            &mut output,
            &config.instructions,
            0,
            |item| match item {
                InstructionItem::Variant0 { targets, .. } => {
                    targets.is_for_target(Targets::Copilot)
                }
                InstructionItem::Variant1 { targets, .. } => {
                    targets.is_for_target(Targets::Copilot)
                }
                InstructionItem::Variant2 { targets, .. } => {
                    targets.is_for_target(Targets::Copilot)
                }
            },
        )?;

        Ok(FormatResult::Single(output))
    }

    fn get_default_path(&self) -> PathBuf {
        PathBuf::from(".github/copilot-instructions.md")
    }
}

/// Parser for Copilot format
pub struct CopilotParser {}

impl FromFormat for CopilotParser {
    fn from_format(content: &str) -> Result<Vec<InstructionItem>, String> {
        common::parse_markdown_instructions(content, Targets::Copilot)
    }
}
