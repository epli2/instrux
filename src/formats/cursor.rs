use super::{FromFormat, ToFormat};
use crate::formats::common::TargetsChecker;
use crate::{
    formats::common,
    model::types::{InstructionItem, InstruxConfiguration, Targets},
};
use std::path::PathBuf;

/// Converter for Cursor format (.cursor/rules)
pub struct CursorConverter {}

impl ToFormat for CursorConverter {
    fn to_format(&self, config: &InstruxConfiguration) -> Result<String, String> {
        let mut output = String::new();
        let header = r#"---
description: Project Rules
globs: "**/*"
alwaysApply: true
---

"#;
        output.push_str(header);

        common::process_instructions_common(
            &mut output,
            &config.instructions,
            0,
            |item| match item {
                InstructionItem::Variant0 { targets, .. } => targets.is_for_target(Targets::Cursor),
                InstructionItem::Variant1 { targets, .. } => targets.is_for_target(Targets::Cursor),
                InstructionItem::Variant2 { targets, .. } => targets.is_for_target(Targets::Cursor),
            },
        )?;

        Ok(output)
    }

    fn get_default_path(&self) -> PathBuf {
        PathBuf::from(".cursor/rules/rule.mdc")
    }
}

pub struct CursorParser {}

impl FromFormat for CursorParser {
    fn from_format(content: &str) -> Result<Vec<InstructionItem>, String> {
        common::parse_markdown_instructions(content, Targets::Junie)
    }
}
