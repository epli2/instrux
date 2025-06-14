use super::common;
use super::{FromFormat, ToFormat};
use crate::model::types::{
    InstructionItem, InstructionItemVariant0Targets, InstructionItemVariant1Targets,
    InstructionItemVariant2Targets, InstruxConfiguration, Targets,
};
use std::path::PathBuf;

/// Converter for Cline format (.clinerules)
pub struct ClineConverter {}

impl ToFormat for ClineConverter {
    fn to_format(&self, config: &InstruxConfiguration) -> Result<String, String> {
        let mut output = String::new();

        // Header for Cline format
        output.push_str("# Cline Rules\n\n");

        common::process_instructions_common(
            &mut output,
            &config.instructions,
            0,
            |item| match item {
                InstructionItem::Variant0 { targets, .. } => is_target_for_cline(targets),
                InstructionItem::Variant1 { targets, .. } => is_target_for_cline(targets),
                InstructionItem::Variant2 { targets, .. } => is_target_for_cline(targets),
            },
        )?;

        Ok(output)
    }

    fn get_default_path(&self) -> PathBuf {
        PathBuf::from(".clinerules")
    }
}

fn is_target_for_cline<T>(targets: &T) -> bool
where
    T: TargetsChecker,
{
    targets.is_for_cline()
}

trait TargetsChecker {
    fn is_for_cline(&self) -> bool;
}

impl TargetsChecker for InstructionItemVariant0Targets {
    fn is_for_cline(&self) -> bool {
        match self {
            InstructionItemVariant0Targets::Variant0(list) => list.contains(&Targets::Cline),
            InstructionItemVariant0Targets::Variant1(s) => s == "all",
        }
    }
}

impl TargetsChecker for InstructionItemVariant1Targets {
    fn is_for_cline(&self) -> bool {
        match self {
            InstructionItemVariant1Targets::Variant0(list) => list.contains(&Targets::Cline),
            InstructionItemVariant1Targets::Variant1(s) => s == "all",
        }
    }
}

impl TargetsChecker for InstructionItemVariant2Targets {
    fn is_for_cline(&self) -> bool {
        match self {
            InstructionItemVariant2Targets::Variant0(list) => list.contains(&Targets::Cline),
            InstructionItemVariant2Targets::Variant1(s) => s == "all",
        }
    }
}

/// Parser for Cline format
pub struct ClineParser {}

impl FromFormat for ClineParser {
    fn from_format(content: &str) -> Result<Vec<InstructionItem>, String> {
        common::parse_markdown_instructions(content, Targets::Cline)
    }
}
