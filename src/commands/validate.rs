use crate::model::parser;
use std::path::Path;

// validateコマンドの基本処理
pub fn run() {
    println!("[validate]");
    let config_path = Path::new(".instrux/instrux.yaml");
    match parser::parse_instrux_yaml(config_path) {
        Ok(config) => {
            println!("設定ファイルのパース成功: {:?}", config);
        }
        Err(e) => {
            eprintln!("設定ファイルのパース失敗: {}", e);
        }
    }
}
