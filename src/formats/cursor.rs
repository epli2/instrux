use super::{FromFormat, ToFormat};
use crate::{
    formats::common,
    model::types::{
        InstructionItem, InstructionItemVariant0Targets, InstructionItemVariant1Targets,
        InstructionItemVariant2Targets, InstruxConfiguration, Targets,
    },
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
                InstructionItem::Variant0 { targets, .. } => is_target_for_cursor(targets),
                InstructionItem::Variant1 { targets, .. } => is_target_for_cursor(targets),
                InstructionItem::Variant2 { targets, .. } => is_target_for_cursor(targets),
            },
        )?;

        Ok(output)
    }

    fn get_default_path(&self) -> PathBuf {
        PathBuf::from(".cursor/rules/rule.mdc")
    }
}

/// Check if an instruction is targeted for Cursor
fn is_target_for_cursor<T>(targets: &T) -> bool
where
    T: TargetsChecker,
{
    targets.is_for_cursor()
}

/// Trait to check if targets include Cursor
trait TargetsChecker {
    fn is_for_cursor(&self) -> bool;
}

impl TargetsChecker for InstructionItemVariant0Targets {
    fn is_for_cursor(&self) -> bool {
        match self {
            InstructionItemVariant0Targets::Variant0(target_list) => {
                target_list.contains(&Targets::Cursor)
            }
            InstructionItemVariant0Targets::Variant1(target_str) => target_str == "all",
        }
    }
}

impl TargetsChecker for InstructionItemVariant1Targets {
    fn is_for_cursor(&self) -> bool {
        match self {
            InstructionItemVariant1Targets::Variant0(target_list) => {
                target_list.contains(&Targets::Cursor)
            }
            InstructionItemVariant1Targets::Variant1(target_str) => target_str == "all",
        }
    }
}

impl TargetsChecker for InstructionItemVariant2Targets {
    fn is_for_cursor(&self) -> bool {
        match self {
            InstructionItemVariant2Targets::Variant0(target_list) => {
                target_list.contains(&Targets::Cursor)
            }
            InstructionItemVariant2Targets::Variant1(target_str) => target_str == "all",
        }
    }
}

pub struct CursorParser {}

impl FromFormat for CursorParser {
    fn from_format(content: &str) -> Result<Vec<InstructionItem>, String> {
        common::parse_markdown_instructions(content, Targets::Junie)
    }
}
