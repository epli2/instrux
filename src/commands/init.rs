use crate::formats;
use crate::model::types::{InstruxConfigurationTargetsValue, Targets};
use std::fs;

// initコマンドの基本処理
pub fn run(from: Option<String>) {
    println!("[init] from: {:?}", from);
    if let Some(tool) = from {
        // ツール名をTargetsに変換
        let target = match tool.to_lowercase().as_str() {
            "copilot" => Targets::Copilot,
            "cline" => Targets::Cline,
            "cursor" => Targets::Cursor,
            "junie" => Targets::Junie,
            "codex" => Targets::Codex,
            _ => {
                eprintln!("[init] 未知のツール形式: {}", tool);
                return;
            }
        };
        // 入力ファイルパスを決定
        let in_path = formats::get_converter(&target).get_default_path();
        let content = match fs::read_to_string(&in_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[init] 入力ファイルの読み込みに失敗: {}", e);
                return;
            }
        };
        // ツール形式→内部モデル
        let instructions = crate::formats::from_format(&target, &content);
        let instructions = match instructions {
            Ok(items) => items,
            Err(e) => {
                eprintln!("[init] {}形式からのパースに失敗: {}", tool, e);
                return;
            }
        };
        // InstruxConfigurationを組み立て
        let config = crate::model::types::InstruxConfiguration {
            instructions,
            language: crate::model::types::InstruxConfigurationLanguage::English,
            targets: {
                let mut map = std::collections::HashMap::new();
                map.insert(target, InstruxConfigurationTargetsValue::default());
                map
            },
            version: "0.1.0".parse().unwrap(),
        };
        // YAMLとして出力
        let yaml = match serde_yaml::to_string(&config) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[init] YAMLシリアライズに失敗: {}", e);
                return;
            }
        };
        if let Err(e) = fs::write(".instrux/instrux.yaml", yaml) {
            eprintln!("[init] instrux.yamlの出力に失敗: {}", e);
        } else {
            println!("[init] instrux.yaml を出力しました");
        }
    }
}
