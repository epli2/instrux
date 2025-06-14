use super::{FromFormat, ToFormat};
use crate::model::types::{InstructionItem, InstruxConfiguration};
use std::path::PathBuf;

/// Converter for Cursor format (.cursor/rules)
pub struct CursorConverter {}

impl ToFormat for CursorConverter {
    fn to_format(&self, _config: &InstruxConfiguration) -> Result<String, String> {
        // TODO: Implement conversion to Cursor format
        Err("Not implemented yet".to_string())
    }

    fn get_default_path(&self) -> PathBuf {
        PathBuf::from(".cursor/rules")
    }
}

/// Parser for Cursor format
pub struct CursorParser {}

impl FromFormat for CursorParser {
    fn from_format(_content: &str) -> Result<Vec<InstructionItem>, String> {
        // TODO: Implement parsing from Cursor format
        Err("Not implemented yet".to_string())
    }
}
