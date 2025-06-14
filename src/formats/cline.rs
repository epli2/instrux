use super::{FromFormat, ToFormat};
use crate::model::types::{InstructionItem, InstruxConfiguration};
use std::path::PathBuf;

/// Converter for Cline format (.clinerules)
pub struct ClineConverter {}

impl ToFormat for ClineConverter {
    fn to_format(&self, _config: &InstruxConfiguration) -> Result<String, String> {
        // TODO: Implement conversion to Cline format
        Err("Not implemented yet".to_string())
    }

    fn get_default_path(&self) -> PathBuf {
        PathBuf::from(".clinerules")
    }
}

/// Parser for Cline format
pub struct ClineParser {}

impl FromFormat for ClineParser {
    fn from_format(_content: &str) -> Result<Vec<InstructionItem>, String> {
        // TODO: Implement parsing from Cline format
        Err("Not implemented yet".to_string())
    }
}
