use crate::formats;
use crate::model::parser::parse_instrux_yaml;
use crate::model::types::Targets;
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
/// * `overwrite` - 既存ファイルを上書きし、.bakバックアップを作成
/// * `force` - バックアップを作成せず強制上書き
/// * `watch` - ファイル変更を監視して自動生成
pub fn run(dry_run: bool, overwrite: bool, force: bool, watch: bool) -> Result<(), String> {
    println!(
        "[generate] dry_run: {}, overwrite: {}, force: {}, watch: {}",
        dry_run, overwrite, force, watch
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
                            if let Err(e) = generate_once(dry_run, overwrite, force) {
                                eprintln!("[generate] エラー: {}", e);
                                // watchモードなのでエラーでも継続するが、エラーがあったことは通知
                            }
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
        generate_once(dry_run, overwrite, force)
    }
    // Ok(()) // watchモード以外はgenerate_onceの結果をそのまま返す
}

/// 1回だけファイル生成処理を行う関数
fn generate_once(dry_run: bool, overwrite: bool, force: bool) -> Result<(), String> {
    // instrux.yamlから内部モデルを読み込む
    let config_path = DEFAULT_INSTRUX_CONFIG_PATH;
    let config = parse_instrux_yaml(config_path)
        .map_err(|e| format!("[generate] 設定ファイルの読み込みに失敗: {}", e))?;

    // dry-run: diffコマンドのロジックを呼び出して終了
    if dry_run {
        crate::commands::diff::run(None); // diff::runもResultを返すように変更が必要かも
        return Ok(());
    }

    // 各ターゲットごとにファイル生成
    // 各ターゲットごとにkey, valueをget_converterに渡してファイル生成
    for (target, value) in &config.targets {
        // converterを取得（key, value両方を渡す）
        let converter = formats::get_converter(target, value);
        let format_result = converter
            .to_format(&config)
            .map_err(|e| format!("[generate] {}形式への変換に失敗: {}", target, e))?;

        // FormatResultに応じて処理を分岐
        match format_result {
            formats::FormatResult::Single(output) => {
                // 単一ファイルの場合
                let out_path = converter.get_default_path();
                process_single_file(target, &out_path, &output, overwrite, force)?;
            }
            formats::FormatResult::Multiple(files) => {
                // 複数ファイルの場合
                let base_path = converter.get_default_path();

                // force/overwrite指定時、既存のbase_pathがファイルなら削除してからディレクトリ作成
                if (force || overwrite) && base_path.exists() && base_path.is_file() {
                    if overwrite {
                        match backup_and_remove_file(&base_path) {
                            Ok(bak_path) => {
                                println!(
                                    "[generate] {} をバックアップしました",
                                    bak_path.display()
                                );
                            }
                            Err(msg) => {
                                eprintln!("[generate] {}", msg);
                                return Err(msg);
                            }
                        }
                    } else {
                        match std::fs::remove_file(&base_path) {
                            Ok(_) => {
                                println!(
                                    "[generate] {} (ファイル) を削除しました",
                                    base_path.display()
                                );
                            }
                            Err(e) => {
                                let msg = format!(
                                    "[generate] 既存ファイルの削除に失敗: {}: {}",
                                    base_path.display(),
                                    e
                                );
                                eprintln!("[generate] {}", msg);
                                return Err(msg);
                            }
                        }
                    }
                }

                // ベースディレクトリの作成 (存在しない場合)
                if !base_path.exists() {
                    std::fs::create_dir_all(&base_path).map_err(|e| {
                        format!(
                            "[generate] ディレクトリ作成に失敗: {}: {}",
                            base_path.display(),
                            e
                        )
                    })?;
                }

                // 各ファイルに書き込み
                for (rel_path, content) in files {
                    let file_path = base_path.join(rel_path); // base_pathからの相対パスとして扱う

                    // 親ディレクトリの作成 (存在しない場合)
                    if let Some(parent) = file_path.parent() {
                        if !parent.exists() {
                            std::fs::create_dir_all(parent).map_err(|e| {
                                format!(
                                    "[generate] ディレクトリ作成に失敗: {}: {}",
                                    parent.display(),
                                    e
                                )
                            })?;
                        }
                    }
                    process_single_file(target, &file_path, &content, overwrite, force)?;
                }
            }
        }
    }
    Ok(())
}

/// 指定ファイルを .bak でバックアップし、元ファイルを削除する
fn backup_and_remove_file(path: &Path) -> Result<std::path::PathBuf, String> {
    let bak_path = {
        let mut bak = path.to_path_buf();
        let bak_os = bak
            .file_name()
            .map(|n| {
                let mut s = n.to_os_string();
                s.push(".bak");
                s
            })
            .ok_or("ファイル名の取得に失敗")?;
        bak.set_file_name(bak_os);
        bak
    };
    std::fs::copy(path, &bak_path).map_err(|e| format!("バックアップ作成に失敗: {}", e))?;
    std::fs::remove_file(path).map_err(|e| format!("既存ファイルの削除に失敗: {}", e))?;
    Ok(bak_path)
}

/// 単一ファイルの出力処理
fn process_single_file(
    target: &Targets,
    out_path: &Path,
    output: &str,
    overwrite: bool,
    force: bool,
) -> Result<(), String> {
    // 出力ディレクトリの作成
    if let Some(parent) = out_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).map_err(|e| {
                format!(
                    "[generate] ディレクトリ作成に失敗: {}: {}",
                    parent.display(),
                    e
                )
            })?;
        }
    }

    let file_exists = out_path.exists();

    // overwrite: バックアップ作成して上書き
    if file_exists && overwrite {
        // 既存ファイルと新規内容が異なる場合のみバックアップ・上書き
        let existing_content = fs::read_to_string(out_path)
            .map_err(|e| format!("[generate] 既存ファイルの読み込みに失敗: {}", e))?;

        if existing_content == output {
            // 差分がなければ何もせずスキップ
            println!("[generate] {} に差分なし。スキップ", out_path.display());
            return Ok(());
        } else {
            // .bak拡張子を追加したバックアップパスを生成し、バックアップ
            let bak_path = {
                let mut bak = out_path.to_path_buf();
                let bak_os = bak
                    .file_name()
                    .map(|n| {
                        let mut s = n.to_os_string();
                        s.push(".bak");
                        s
                    })
                    .ok_or_else(|| format!("[generate] バックアップパスの生成に失敗: {:?}", out_path))?;
                bak.set_file_name(bak_os);
                bak
            };
            fs::copy(out_path, &bak_path)
                .map_err(|e| format!("[generate] バックアップ作成に失敗: {}", e))?;
            println!("[generate] {} をバックアップしました", bak_path.display());
        }

        fs::write(out_path, output)
            .map_err(|e| format!("[generate] ファイル出力に失敗: {}", e))?;
        println!("[generate] {} を上書きしました", out_path.display());
        return Ok(());
    }

    // force: バックアップせず強制上書き
    if file_exists && force {
        fs::write(out_path, output).map_err(|e| {
            format!(
                "[generate] ファイル出力に失敗: {}: {}",
                out_path.display(),
                e
            )
        })?;
        println!("[generate] {} を強制上書きしました", out_path.display());
        return Ok(());
    }

    // 既存ファイルがありoverwrite/forceでない場合はスキップ
    if file_exists {
        println!(
            "[generate] {} は既に存在します (--overwrite でバックアップ上書き, --force で強制上書き)",
            out_path.display()
        );
        return Ok(());
    }

    // 新規ファイル出力
    fs::write(out_path, output).map_err(|e| {
        format!(
            "[generate] ファイル出力に失敗: {}: {}",
            out_path.display(),
            e
        )
    })?;
    println!(
        "[generate] {}形式の出力を生成: {}",
        target,
        out_path.display()
    );
    Ok(())
}
