use super::common::{self, TargetsChecker};
use super::{FormatResult, FromFormat, ToFormat};
use crate::model::types::{InstructionItem, InstruxConfiguration, Targets};
use std::path::PathBuf;

/// Converter for Cline format (.clinerules)
pub struct ClineConverter {}

impl ToFormat for ClineConverter {
    fn to_format(&self, config: &InstruxConfiguration) -> Result<FormatResult, String> {
        let mut output = String::new();

        // Header for Cline format
        output.push_str("# Cline Rules\n\n");

        common::process_instructions_common(
            &mut output,
            &config.instructions,
            0,
            |item| match item {
                // 共通トレイトでターゲット判定
                InstructionItem::Variant0 { targets, .. } => targets.is_for_target(Targets::Cline),
                InstructionItem::Variant1 { targets, .. } => targets.is_for_target(Targets::Cline),
                InstructionItem::Variant2 { targets, .. } => targets.is_for_target(Targets::Cline),
            },
        )?;

        Ok(FormatResult::Single(output))
    }

    fn get_default_path(&self) -> PathBuf {
        PathBuf::from(".clinerules") // 単一ファイルは .clinerules ファイルに出力
    }
}

/// Parser for Cline format
pub struct ClineParser {}

impl FromFormat for ClineParser {
    fn from_format(content: &str) -> Result<Vec<InstructionItem>, String> {
        common::parse_markdown_instructions(content, Targets::Cline)
    }
}
