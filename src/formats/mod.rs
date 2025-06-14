use crate::model::types::{InstructionItem, InstruxConfiguration, Targets};
use std::path::PathBuf;

mod cline;
mod codex;
mod copilot;
mod cursor;
mod junie;

#[cfg(test)]
mod tests;

/// Trait for converting from the instrux model to a target format
pub trait ToFormat {
    /// Convert from instrux model to the target format
    fn to_format(&self, config: &InstruxConfiguration) -> Result<String, String>;

    /// Get the default file path for the target format
    fn get_default_path(&self) -> PathBuf;
}

/// Trait for converting from a target format to the instrux model
pub trait FromFormat {
    /// Convert from the target format to the instrux model
    fn from_format(content: &str) -> Result<Vec<InstructionItem>, String>;
}

/// Factory to get the converter for a specific target
pub fn get_converter(target: &Targets) -> Box<dyn ToFormat> {
    match target {
        Targets::Copilot => Box::new(copilot::CopilotConverter {}),
        Targets::Cline => Box::new(cline::ClineConverter {}),
        Targets::Cursor => Box::new(cursor::CursorConverter {}),
        Targets::Junie => Box::new(junie::JunieConverter {}),
        Targets::Codex => Box::new(codex::CodexConverter {}),
    }
}

pub fn from_format(target: &Targets, content: &str) -> Result<Vec<InstructionItem>, String> {
    match target {
        Targets::Copilot => copilot::CopilotParser::from_format(content),
        Targets::Cline => cline::ClineParser::from_format(content),
        Targets::Cursor => cursor::CursorParser::from_format(content),
        Targets::Junie => junie::JunieParser::from_format(content),
        Targets::Codex => codex::CodexParser::from_format(content),
    }
}
