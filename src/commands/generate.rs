use crate::formats;
use crate::model::parser::parse_instrux_yaml;
use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

const DEFAULT_INSTRUX_CONFIG_PATH: &str = ".instrux/instrux.yaml";

/// generateコマンドの基本処理
///
/// # 引数
/// * `dry_run` - ファイルを出力せず内容のみ表示
/// * `force` - 既存ファイルを強制的に上書き
/// * `watch` - ファイル変更を監視して自動生成
pub fn run(dry_run: bool, force: bool, watch: bool) {
    println!(
        "[generate] dry_run: {}, force: {}, watch: {}",
        dry_run, force, watch
    );

    // watchモードの場合は監視ループに入る
    if watch {
        // チャンネル作成
        let (tx, rx) = channel();
        // ファイル監視用Watcherを作成
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default()).unwrap();
        // instrux.yamlを監視対象に追加
        watcher
            .watch(
                Path::new(DEFAULT_INSTRUX_CONFIG_PATH),
                RecursiveMode::NonRecursive,
            )
            .unwrap();
        println!(
            "[generate] {} の変更を監視中...",
            DEFAULT_INSTRUX_CONFIG_PATH
        );
        loop {
            // 変更イベントを待つ
            match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(event) => {
                    if let Ok(ev) = event {
                        if matches!(ev.kind, EventKind::Modify(_)) {
                            println!("[generate] 構成ファイルが変更されました。再生成します...");
                            generate_once(dry_run, force);
                        }
                    }
                }
                Err(_) => {
                    // タイムアウト時は何もしない（Ctrl+Cで抜ける想定）
                }
            }
        }
    } else {
        // 通常の1回生成
        generate_once(dry_run, force);
    }
}

/// 1回だけファイル生成処理を行う関数
fn generate_once(dry_run: bool, force: bool) {
    // instrux.yamlから内部モデルを読み込む
    let config_path = DEFAULT_INSTRUX_CONFIG_PATH;
    let config = match parse_instrux_yaml(config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("[generate] 設定ファイルの読み込みに失敗: {}", e);
            return;
        }
    };

    // 各ターゲットごとにファイル生成
    for target in config.targets.keys() {
        let converter = formats::get_converter(target);
        let output = match converter.to_format(&config) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[generate] {}形式への変換に失敗: {}", target, e);
                return;
            }
        };

        // 出力先パスを取得
        let out_path = converter.get_default_path();

        // dry-run: 内容のみ表示しファイルは生成しない
        if dry_run {
            println!(
                "[generate] dry-run: {} に出力される内容:\n\n{}",
                out_path.display(),
                output
            );
            continue;
        }

        // 既存ファイルがありforceでない場合はスキップ
        if Path::new(&out_path).exists() && !force {
            println!(
                "[generate] {} は既に存在します (--forceで上書き)",
                out_path.display()
            );
            continue;
        }

        // ファイル出力
        if let Err(e) = fs::write(&out_path, output) {
            eprintln!("[generate] ファイル出力に失敗: {}", e);
        } else {
            println!("[generate] {} を出力しました", out_path.display());
        }
    }
}
