use std::fs;
use std::path::Path;

use crate::model::types::InstruxConfiguration;

pub fn parse_instrux_yaml<P: AsRef<Path>>(path: P) -> Result<InstruxConfiguration, String> {
    let content = fs::read_to_string(&path).map_err(|e| format!("ファイル読み込み失敗: {}", e))?;
    serde_yaml::from_str(&content).map_err(|e| format!("YAMLパース失敗: {}", e))
}
