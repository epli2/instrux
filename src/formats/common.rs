use crate::model::types::{InstructionItem, InstructionItemVariant0Targets, Targets};

/// 共通: 再帰的にInstructionItemをMarkdown出力
pub fn process_instructions_common<F>(
    output: &mut String,
    instructions: &[InstructionItem],
    level: usize,
    is_target: F,
) -> Result<(), String>
where
    F: Fn(&InstructionItem) -> bool + Copy,
{
    for instruction in instructions {
        match instruction {
            InstructionItem::Variant0 {
                title,
                body,
                disable,
                ..
            } => {
                if *disable || !is_target(instruction) {
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
                ..
            } => {
                if *disable || !is_target(instruction) {
                    continue;
                }
                output.push_str(&format!("{} {}\n\n", "#".repeat(level + 2), title));
                // TODO: ファイルからの内容読み込み
                output.push_str(&format!("<!-- Content from file: {} -->\n\n", body_file));
            }
            InstructionItem::Variant2 {
                title,
                instructions: nested,
                disable,
                ..
            } => {
                if *disable || !is_target(instruction) {
                    continue;
                }
                output.push_str(&format!("{} {}\n\n", "#".repeat(level + 2), title));
                process_instructions_common(output, nested, level + 1, is_target)?;
            }
        }
    }
    Ok(())
}

/// 共通: Markdownヘッダ区切りでInstructionItemを抽出
pub fn parse_markdown_instructions(
    content: &str,
    target: Targets,
) -> Result<Vec<InstructionItem>, String> {
    let mut instructions = Vec::new();
    let mut sections = Vec::new();
    let mut current_section = String::new();
    let mut current_level = 0;
    let mut current_title = String::new();

    for line in content.lines() {
        if line.starts_with('#') {
            let level = line.chars().take_while(|&c| c == '#').count();
            let title = line.trim_start_matches('#').trim().to_string();
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
    if !current_title.is_empty() {
        sections.push((
            current_level,
            current_title.clone(),
            current_section.clone(),
        ));
    }
    for (level, title, body) in sections {
        if level <= 1 {
            continue;
        }
        let instruction = InstructionItem::Variant0 {
            title,
            body: body.trim().to_string(),
            description: None,
            disable: false,
            targets: InstructionItemVariant0Targets::Variant0(vec![target]),
        };
        instructions.push(instruction);
    }
    if instructions.is_empty() {
        return Err("No valid instructions found in the format file".to_string());
    }
    Ok(instructions)
}
