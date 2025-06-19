use super::common;
use super::{FormatResult, FromFormat, ToFormat};
use crate::formats::common::TargetsChecker;
use crate::model::types::{InstructionItem, InstruxConfiguration, Targets};
use std::path::PathBuf;

/// Converter for Junie format (.junie/guidelines.md)
pub struct JunieConverter {}

impl ToFormat for JunieConverter {
    fn to_format(&self, config: &InstruxConfiguration) -> Result<FormatResult, String> {
        let mut output = String::new();

        // Header
        output.push_str("# Junie Guidelines\n\n");

        common::process_instructions_common(
            &mut output,
            &config.instructions,
            0,
            |item| match item {
                InstructionItem::Variant0 { targets, .. } => targets.is_for_target(Targets::Junie),
                InstructionItem::Variant1 { targets, .. } => targets.is_for_target(Targets::Junie),
                InstructionItem::Variant2 { targets, .. } => targets.is_for_target(Targets::Junie),
            },
        )?;

        Ok(FormatResult::Single(output))
    }

    fn get_default_path(&self) -> PathBuf {
        PathBuf::from(".junie/guidelines.md")
    }
}

/// Parser for Junie format
pub struct JunieParser {}

impl FromFormat for JunieParser {
    fn from_format(content: &str) -> Result<Vec<InstructionItem>, String> {
        common::parse_markdown_instructions(content, Targets::Junie)
    }
}
