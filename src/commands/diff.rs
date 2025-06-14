use crate::diff;
use crate::model::{parser, types::Targets};
use std::path::Path;

/// diffコマンドの基本処理
pub fn run(tool: Option<String>) {
    let config_path = Path::new(".instrux/instrux.yaml");
    let config = match parser::parse_instrux_yaml(config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[diff] 設定ファイルの読み込みに失敗: {}", e);
            return;
        }
    };

    let targets: Vec<Targets> = if let Some(name) = tool {
        match name.to_lowercase().as_str() {
            "copilot" => vec![Targets::Copilot],
            "cline" => vec![Targets::Cline],
            "cursor" => vec![Targets::Cursor],
            "junie" => vec![Targets::Junie],
            "codex" => vec![Targets::Codex],
            other => {
                eprintln!("[diff] 未知のツール形式: {}", other);
                return;
            }
        }
    } else {
        config.targets.keys().cloned().collect()
    };

    for target in targets {
        match diff::diff_from_config(&config, target) {
            Ok(d) => {
                println!("--- {:?} ---\n{}", target, d);
            }
            Err(e) => {
                eprintln!("[diff] {}", e);
            }
        }
    }
}
