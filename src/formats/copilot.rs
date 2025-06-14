use super::{FromFormat, ToFormat};
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

        // Process instructions
        process_instructions(&mut output, &config.instructions, 0)?;

        Ok(output)
    }

    fn get_default_path(&self) -> PathBuf {
        PathBuf::from(".github/copilot-instructions.md")
    }
}

/// Helper function to process instructions recursively
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
                // Skip disabled instructions
                if *disable {
                    continue;
                }

                // Skip if not targeted for Copilot
                if !is_target_for_copilot(targets) {
                    continue;
                }

                // Add section header with proper heading level
                output.push_str(&format!("{} {}\n\n", "#".repeat(level + 2), title));

                // Add instruction body
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
                // Skip disabled instructions
                if *disable {
                    continue;
                }

                // Skip if not targeted for Copilot
                if !is_target_for_copilot(targets) {
                    continue;
                }

                // Add section header with proper heading level
                output.push_str(&format!("{} {}\n\n", "#".repeat(level + 2), title));

                // For a complete implementation, we would load content from the file
                // This requires access to the file system and a base path
                output.push_str(&format!("<!-- Content from file: {} -->\n\n", body_file));
            }
            InstructionItem::Variant2 {
                title,
                instructions: nested_instructions,
                disable,
                targets,
                ..
            } => {
                // Skip disabled instructions
                if *disable {
                    continue;
                }

                // Skip if not targeted for Copilot
                if !is_target_for_copilot(targets) {
                    continue;
                }

                // Add section header with proper heading level
                output.push_str(&format!("{} {}\n\n", "#".repeat(level + 2), title));

                // Process nested instructions
                process_instructions(output, nested_instructions, level + 1)?;
            }
        }
    }

    Ok(())
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
        // Parse Markdown content to extract instructions
        let mut instructions = Vec::new();

        // Split content by headers
        let mut sections = Vec::new();
        let mut current_section = String::new();
        let mut current_level = 0;
        let mut current_title = String::new();

        for line in content.lines() {
            if line.starts_with('#') {
                // Count the level (number of # characters)
                let level = line.chars().take_while(|&c| c == '#').count();
                let title = line.trim_start_matches('#').trim().to_string();

                // If we have a current section, save it
                if !current_title.is_empty() {
                    sections.push((
                        current_level,
                        current_title.clone(),
                        current_section.clone(),
                    ));
                    current_section.clear();
                }

                current_level = level;
                current_title = title;
            } else {
                current_section.push_str(line);
                current_section.push('\n');
            }
        }

        // Add the last section
        if !current_title.is_empty() {
            sections.push((
                current_level,
                current_title.clone(),
                current_section.clone(),
            ));
        }

        // Process sections to create instruction items
        // We don't actually use the section_stack in this implementation, but it would be used
        // for a more sophisticated parsing strategy that preserves the hierarchy

        for (level, title, body) in sections {
            // Skip the top-level header (usually "Copilot Instructions")
            if level <= 1 {
                continue;
            }

            // Create a new instruction
            let instruction = InstructionItem::Variant0 {
                title,
                body: body.trim().to_string(),
                description: None,
                disable: false,
                targets: InstructionItemVariant0Targets::Variant0(vec![Targets::Copilot]),
            };

            instructions.push(instruction);
        }

        if instructions.is_empty() {
            return Err("No valid instructions found in the Copilot format file".to_string());
        }

        Ok(instructions)
    }
}
