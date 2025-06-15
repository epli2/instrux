use super::{FromFormat, ToFormat, common};
use crate::formats::common::TargetsChecker;
use crate::model::types::{InstructionItem, InstruxConfiguration, Targets};
use std::path::PathBuf;

/// Converter for Codex format (CODEX.md)
pub struct CodexConverter {}

impl ToFormat for CodexConverter {
    fn to_format(&self, config: &InstruxConfiguration) -> Result<String, String> {
        let mut output = String::new();
        output.push_str("# Codex Instructions\n\n");
        common::process_instructions_common(
            &mut output,
            &config.instructions,
            0,
            |item| match item {
                InstructionItem::Variant0 { targets, .. } => targets.is_for_target(Targets::Codex),
                InstructionItem::Variant1 { targets, .. } => targets.is_for_target(Targets::Codex),
                InstructionItem::Variant2 { targets, .. } => targets.is_for_target(Targets::Codex),
            },
        )?;
        Ok(output)
    }

    fn get_default_path(&self) -> PathBuf {
        PathBuf::from("CODEX.md")
    }
}

/// Parser for Codex format
pub struct CodexParser {}

impl FromFormat for CodexParser {
    fn from_format(content: &str) -> Result<Vec<InstructionItem>, String> {
        common::parse_markdown_instructions(content, Targets::Codex)
    }
}
