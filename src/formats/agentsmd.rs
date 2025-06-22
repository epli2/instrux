use super::{FormatResult, FromFormat, ToFormat, common};
use crate::formats::common::TargetsChecker;
use crate::model::types::{InstructionItem, InstruxConfiguration, Targets};
use std::path::PathBuf;

/// Converter for AGENTS.md format
pub struct AgentsMdConverter {}

impl ToFormat for AgentsMdConverter {
    fn to_format(&self, config: &InstruxConfiguration) -> Result<FormatResult, String> {
        let mut output = String::new();
        output.push_str("# Agents Instructions\n\n");
        common::process_instructions_common(
            &mut output,
            &config.instructions,
            0,
            |item| match item {
                InstructionItem::Variant0 { targets, .. } => {
                    targets.is_for_target(Targets::Agentsmd)
                }
                InstructionItem::Variant1 { targets, .. } => {
                    targets.is_for_target(Targets::Agentsmd)
                }
                InstructionItem::Variant2 { targets, .. } => {
                    targets.is_for_target(Targets::Agentsmd)
                }
            },
        )?;
        Ok(FormatResult::Single(output))
    }

    fn get_default_path(&self) -> PathBuf {
        PathBuf::from("AGENTS.md")
    }
}

/// Parser for AgentsMD format
pub struct AgentsMdParser {}

impl FromFormat for AgentsMdParser {
    fn from_format(content: &str) -> Result<Vec<InstructionItem>, String> {
        common::parse_markdown_instructions(content, Targets::Agentsmd)
    }
}
