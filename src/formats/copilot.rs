use super::{FromFormat, ToFormat};
use crate::formats::common;
use crate::model::types::{
    InstructionItem, InstructionItemVariant0Targets, InstructionItemVariant1Targets,
    InstructionItemVariant2Targets, InstruxConfiguration, Targets,
};
use std::path::PathBuf;

/// Converter for Copilot format (copilot-instructions.md)
pub struct CopilotConverter {}

impl ToFormat for CopilotConverter {
    fn to_format(&self, config: &InstruxConfiguration) -> Result<String, String> {
        let mut output = String::new();

        // Add header section with metadata
        output.push_str("# Copilot Instructions\n\n");

        common::process_instructions_common(
            &mut output,
            &config.instructions,
            0,
            |item| match item {
                InstructionItem::Variant0 { targets, .. } => is_target_for_copilot(targets),
                InstructionItem::Variant1 { targets, .. } => is_target_for_copilot(targets),
                InstructionItem::Variant2 { targets, .. } => is_target_for_copilot(targets),
            },
        )?;

        Ok(output)
    }

    fn get_default_path(&self) -> PathBuf {
        PathBuf::from(".github/copilot-instructions.md")
    }
}

/// Check if an instruction is targeted for Copilot
fn is_target_for_copilot<T>(targets: &T) -> bool
where
    T: TargetsChecker,
{
    targets.is_for_copilot()
}

/// Trait to check if targets include Copilot
trait TargetsChecker {
    fn is_for_copilot(&self) -> bool;
}

impl TargetsChecker for InstructionItemVariant0Targets {
    fn is_for_copilot(&self) -> bool {
        match self {
            InstructionItemVariant0Targets::Variant0(target_list) => {
                target_list.contains(&Targets::Copilot)
            }
            InstructionItemVariant0Targets::Variant1(target_str) => target_str == "all",
        }
    }
}

impl TargetsChecker for InstructionItemVariant1Targets {
    fn is_for_copilot(&self) -> bool {
        match self {
            InstructionItemVariant1Targets::Variant0(target_list) => {
                target_list.contains(&Targets::Copilot)
            }
            InstructionItemVariant1Targets::Variant1(target_str) => target_str == "all",
        }
    }
}

impl TargetsChecker for InstructionItemVariant2Targets {
    fn is_for_copilot(&self) -> bool {
        match self {
            InstructionItemVariant2Targets::Variant0(target_list) => {
                target_list.contains(&Targets::Copilot)
            }
            InstructionItemVariant2Targets::Variant1(target_str) => target_str == "all",
        }
    }
}

/// Parser for Copilot format
pub struct CopilotParser {}

impl FromFormat for CopilotParser {
    fn from_format(content: &str) -> Result<Vec<InstructionItem>, String> {
        common::parse_markdown_instructions(content, Targets::Copilot)
    }
}
