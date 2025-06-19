use crate::formats::common::TargetsChecker;
use crate::formats::{FormatResult, ToFormat};
use crate::model::types::{InstructionItem, InstruxConfiguration, Targets};
use std::collections::HashMap;
use std::path::PathBuf;

/// CopilotMultipleConverter
/// outputMode=multipleの場合、各InstructionItemを個別ファイル(.github/instructions/*.instructions.md)として出力するコンバータ
pub struct CopilotMultipleConverter {}

impl CopilotMultipleConverter {
    /// ネストされたInstructionItemを再帰的にMarkdown化
    fn instruction_to_md(instruction: &InstructionItem, level: usize) -> String {
        match instruction {
            InstructionItem::Variant0 { title, body, .. } => {
                format!(
                    "{hashes} {title}\n\n{body}\n",
                    hashes = "#".repeat(level),
                    title = title,
                    body = body
                )
            }
            InstructionItem::Variant1 {
                title, body_file, ..
            } => {
                let path = format!(".instrux/instructions/{}", body_file);
                let file_content = std::fs::read_to_string(&path).unwrap_or_else(|_| {
                    format!("<!-- Content from file: {body_file} (not found) -->\n")
                });
                format!(
                    "{hashes} {title}\n\n{content}\n",
                    hashes = "#".repeat(level),
                    title = title,
                    content = file_content
                )
            }
            InstructionItem::Variant2 {
                title,
                instructions: nested,
                ..
            } => {
                let nested_md: String = nested
                    .iter()
                    .map(|nested_item| Self::instruction_to_md(nested_item, level + 1))
                    .collect();
                format!(
                    "{hashes} {title}\n\n{nested}\n",
                    hashes = "#".repeat(level),
                    title = title,
                    nested = nested_md
                )
            }
        }
    }
}

impl ToFormat for CopilotMultipleConverter {
    /// 複数ファイル形式で出力する
    /// 戻り値: FormatResult::Multiple(ファイルパス→内容のマップ)
    fn to_format(&self, config: &InstruxConfiguration) -> Result<FormatResult, String> {
        let mut files = HashMap::new();
        for instruction in &config.instructions {
            // Copilotターゲットのみ対象
            let is_copilot = match instruction {
                InstructionItem::Variant0 {
                    targets, disable, ..
                } => !*disable && targets.is_for_target(Targets::Copilot),
                InstructionItem::Variant1 {
                    targets, disable, ..
                } => !*disable && targets.is_for_target(Targets::Copilot),
                InstructionItem::Variant2 {
                    targets, disable, ..
                } => !*disable && targets.is_for_target(Targets::Copilot),
            };
            if !is_copilot {
                continue;
            }
            let title = match instruction {
                InstructionItem::Variant0 { title, .. } => title,
                InstructionItem::Variant1 { title, .. } => title,
                InstructionItem::Variant2 { title, .. } => title,
            };
            let description = match instruction {
                InstructionItem::Variant0 { description, .. } => description.as_ref(),
                InstructionItem::Variant1 { description, .. } => description.as_ref(),
                InstructionItem::Variant2 { description, .. } => description.as_ref(),
            };
            let frontmatter = if let Some(description) = description {
                format!("---\ndescription: {}\n---\n\n", description)
            } else {
                String::new()
            };
            let content = Self::instruction_to_md(instruction, 1);
            let file_path = format!(".github/instructions/{}.instructions.md", title);
            files.insert(file_path, format!("{}{}", frontmatter, content));
        }
        Ok(FormatResult::Multiple(files))
    }

    /// Get the default file path for the target format
    /// 複数ファイル出力の場合は格納ディレクトリを返す
    fn get_default_path(&self) -> PathBuf {
        PathBuf::from(".github/instructions")
    }
}
