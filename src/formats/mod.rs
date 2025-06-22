use crate::model::types::{
    InstructionItem, InstruxConfiguration, InstruxConfigurationTargetsValue,
    InstruxConfigurationTargetsValueOutputMode, Targets,
};
use std::collections::HashMap;
use std::path::PathBuf;

mod agentsmd;
mod cline;
mod cline_multiple;
mod common;
mod copilot;
mod copilot_multiple;
mod cursor;
mod junie;

#[cfg(test)]
mod tests;

/// フォーマット変換結果を表す型
/// 単一ファイルの場合は単一の文字列、複数ファイルの場合はパス→内容のマップ
pub enum FormatResult {
    /// 単一ファイルの内容
    Single(String),
    /// 複数ファイルの内容 (ファイルパス→内容のマップ)
    Multiple(HashMap<String, String>),
}

/// Trait for converting from the instrux model to a target format
pub trait ToFormat {
    /// Convert from instrux model to the target format
    /// outputModeなどに応じて単一ファイルまたは複数ファイルを返す
    fn to_format(&self, config: &InstruxConfiguration) -> Result<FormatResult, String>;

    /// Get the default file path for the target format
    /// Single結果の場合のパス、Multiple結果の場合はベースディレクトリ
    fn get_default_path(&self) -> PathBuf;
}

/// Factory to get the converter for a specific target, outputModeも考慮
pub fn get_converter(
    target: &Targets,
    target_config: &InstruxConfigurationTargetsValue,
) -> Box<dyn ToFormat> {
    match target {
        Targets::Copilot => {
            if target_config.output_mode == InstruxConfigurationTargetsValueOutputMode::Multiple {
                return Box::new(copilot_multiple::CopilotMultipleConverter {});
            }
            Box::new(copilot::CopilotConverter {})
        }
        Targets::Cline => {
            if target_config.output_mode == InstruxConfigurationTargetsValueOutputMode::Multiple {
                return Box::new(cline_multiple::ClineMultipleConverter {});
            }
            Box::new(cline::ClineConverter {})
        }
        Targets::Cursor => Box::new(cursor::CursorConverter {}),
        Targets::Junie => Box::new(junie::JunieConverter {}),
        Targets::Agentsmd => Box::new(agentsmd::AgentsMdConverter {}),
        Targets::Codex => unreachable!("Codex is deprecated. Use agentsmd (AGENTS.md) instead."),
    }
}

/// Trait for converting from a target format to the instrux model
pub trait FromFormat {
    /// Convert from the target format to the instrux model
    fn from_format(content: &str) -> Result<Vec<InstructionItem>, String>;
}

pub fn from_format(target: &Targets, content: &str) -> Result<Vec<InstructionItem>, String> {
    match target {
        Targets::Copilot => copilot::CopilotParser::from_format(content),
        Targets::Cline => cline::ClineParser::from_format(content),
        Targets::Cursor => cursor::CursorParser::from_format(content),
        Targets::Junie => junie::JunieParser::from_format(content),
        Targets::Agentsmd => agentsmd::AgentsMdParser::from_format(content),
        Targets::Codex => {
            unreachable!("Codex is deprecated. Use agentsmd (AGENTS.md) instead.")
        }
    }
}
