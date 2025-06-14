use super::{FromFormat, ToFormat};
use crate::model::types::{InstructionItem, InstruxConfiguration};
use std::path::PathBuf;

/// Converter for Junie format (.junie/guidelines.md)
pub struct JunieConverter {}

impl ToFormat for JunieConverter {
    fn to_format(&self, _config: &InstruxConfiguration) -> Result<String, String> {
        // TODO: Implement conversion to Junie format
        Err("Not implemented yet".to_string())
    }

    fn get_default_path(&self) -> PathBuf {
        PathBuf::from(".junie/guidelines.md")
    }
}

/// Parser for Junie format
pub struct JunieParser {}

impl FromFormat for JunieParser {
    fn from_format(_content: &str) -> Result<Vec<InstructionItem>, String> {
        // TODO: Implement parsing from Junie format
        Err("Not implemented yet".to_string())
    }
}
