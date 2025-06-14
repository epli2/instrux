use super::{FromFormat, ToFormat};
use crate::model::types::{InstructionItem, InstruxConfiguration};
use std::path::PathBuf;

/// Converter for Codex format (.codex/instructions.md)
pub struct CodexConverter {}

impl ToFormat for CodexConverter {
    fn to_format(&self, _config: &InstruxConfiguration) -> Result<String, String> {
        // TODO: Implement conversion to Codex format
        Err("Not implemented yet".to_string())
    }

    fn get_default_path(&self) -> PathBuf {
        PathBuf::from(".codex/instructions.md")
    }
}

/// Parser for Codex format
pub struct CodexParser {}

impl FromFormat for CodexParser {
    fn from_format(_content: &str) -> Result<Vec<InstructionItem>, String> {
        // TODO: Implement parsing from Codex format
        Err("Not implemented yet".to_string())
    }
}
