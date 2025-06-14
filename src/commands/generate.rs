use crate::formats;
use crate::model::parser::parse_instrux_yaml;
use crate::model::types::Targets;
use std::fs;

// generateコマンドの基本処理
pub fn run(dry_run: bool, force: bool, watch: bool) {
    println!(
        "[generate] dry_run: {}, force: {}, watch: {}",
        dry_run, force, watch
    );

    // 1. instrux.yamlから内部モデルを読み込む
    let config_path = ".instrux/instrux.yaml";
    let config = match parse_instrux_yaml(config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("[generate] 設定ファイルの読み込みに失敗: {}", e);
            return;
        }
    };

    // 2. Copilot形式に変換
    if config.targets.contains_key(&Targets::Copilot) {
        let converter = formats::get_converter(&Targets::Copilot);
        let output = match converter.to_format(&config) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[generate] Copilot形式への変換に失敗: {}", e);
                return;
            }
        };

        // 3. ファイル出力
        let out_path = converter.get_default_path();
        if dry_run {
            println!(
                "[generate] dry-run: {} に出力される内容:\n\n{}",
                out_path.display(),
                output
            );
        } else if let Err(e) = fs::write(&out_path, output) {
            eprintln!("[generate] ファイル出力に失敗: {}", e);
        } else {
            println!("[generate] {} を出力しました", out_path.display());
        }
    }
}
