use super::common;
use super::{FromFormat, ToFormat};
use crate::model::types::{
    InstructionItem, InstructionItemVariant0Targets, InstructionItemVariant1Targets,
    InstructionItemVariant2Targets, InstruxConfiguration, Targets,
};
use std::path::PathBuf;

/// Converter for Junie format (.junie/guidelines.md)
pub struct JunieConverter {}

impl ToFormat for JunieConverter {
    fn to_format(&self, config: &InstruxConfiguration) -> Result<String, String> {
        let mut output = String::new();

        // Header
        output.push_str("# Junie Guidelines\n\n");

        process_instructions(&mut output, &config.instructions, 0)?;

        Ok(output)
    }

    fn get_default_path(&self) -> PathBuf {
        PathBuf::from(".junie/guidelines.md")
    }
}

/// Helper function to process instructions recursively for Junie format
fn process_instructions(
    output: &mut String,
    instructions: &[InstructionItem],
    level: usize,
) -> Result<(), String> {
    for instruction in instructions {
        match instruction {
            InstructionItem::Variant0 {
                title,
                body,
                disable,
                targets,
                ..
            } => {
                if *disable {
                    continue;
                }

                if !is_target_for_junie(targets) {
                    continue;
                }

                output.push_str(&format!("{} {}\n\n", "#".repeat(level + 2), title));
                output.push_str(body);
                output.push_str("\n\n");
            }
            InstructionItem::Variant1 {
                title,
                body_file,
                disable,
                targets,
                ..
            } => {
                if *disable {
                    continue;
                }

                if !is_target_for_junie(targets) {
                    continue;
                }

                output.push_str(&format!("{} {}\n\n", "#".repeat(level + 2), title));
                output.push_str(&format!("<!-- Content from file: {} -->\n\n", body_file));
            }
            InstructionItem::Variant2 {
                title,
                instructions: nested,
                disable,
                targets,
                ..
            } => {
                if *disable {
                    continue;
                }

                if !is_target_for_junie(targets) {
                    continue;
                }

                output.push_str(&format!("{} {}\n\n", "#".repeat(level + 2), title));
                process_instructions(output, nested, level + 1)?;
            }
        }
    }

    Ok(())
}

fn is_target_for_junie<T>(targets: &T) -> bool
where
    T: TargetsChecker,
{
    targets.is_for_junie()
}

trait TargetsChecker {
    fn is_for_junie(&self) -> bool;
}

impl TargetsChecker for InstructionItemVariant0Targets {
    fn is_for_junie(&self) -> bool {
        match self {
            InstructionItemVariant0Targets::Variant0(list) => list.contains(&Targets::Junie),
            InstructionItemVariant0Targets::Variant1(s) => s == "all",
        }
    }
}

impl TargetsChecker for InstructionItemVariant1Targets {
    fn is_for_junie(&self) -> bool {
        match self {
            InstructionItemVariant1Targets::Variant0(list) => list.contains(&Targets::Junie),
            InstructionItemVariant1Targets::Variant1(s) => s == "all",
        }
    }
}

impl TargetsChecker for InstructionItemVariant2Targets {
    fn is_for_junie(&self) -> bool {
        match self {
            InstructionItemVariant2Targets::Variant0(list) => list.contains(&Targets::Junie),
            InstructionItemVariant2Targets::Variant1(s) => s == "all",
        }
    }
}

/// Parser for Junie format
pub struct JunieParser {}

impl FromFormat for JunieParser {
    fn from_format(content: &str) -> Result<Vec<InstructionItem>, String> {
        common::parse_markdown_instructions(content, Targets::Junie)
    }
}
