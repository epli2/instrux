use super::{FromFormat, ToFormat, common};
use crate::model::types::{
    InstructionItem, InstructionItemVariant0Targets, InstructionItemVariant1Targets,
    InstructionItemVariant2Targets, InstruxConfiguration, Targets,
};
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
                InstructionItem::Variant0 { targets, .. } => is_target_for_codex(targets),
                InstructionItem::Variant1 { targets, .. } => is_target_for_codex(targets),
                InstructionItem::Variant2 { targets, .. } => is_target_for_codex(targets),
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

fn is_target_for_codex<T>(targets: &T) -> bool
where
    T: TargetsChecker,
{
    targets.is_for_codex()
}

trait TargetsChecker {
    fn is_for_codex(&self) -> bool;
}

impl TargetsChecker for InstructionItemVariant0Targets {
    fn is_for_codex(&self) -> bool {
        match self {
            InstructionItemVariant0Targets::Variant0(list) => list.contains(&Targets::Codex),
            InstructionItemVariant0Targets::Variant1(s) => s == "all",
        }
    }
}

impl TargetsChecker for InstructionItemVariant1Targets {
    fn is_for_codex(&self) -> bool {
        match self {
            InstructionItemVariant1Targets::Variant0(list) => list.contains(&Targets::Codex),
            InstructionItemVariant1Targets::Variant1(s) => s == "all",
        }
    }
}

impl TargetsChecker for InstructionItemVariant2Targets {
    fn is_for_codex(&self) -> bool {
        match self {
            InstructionItemVariant2Targets::Variant0(list) => list.contains(&Targets::Codex),
            InstructionItemVariant2Targets::Variant1(s) => s == "all",
        }
    }
}
