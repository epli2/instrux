use super::super::common::process_instructions_common;
use crate::model::types::{InstructionItem, InstructionItemVariant0Targets, Targets};
use std::fs;
use std::path::Path;

#[test]
fn test_process_instructions_common_reads_body_file() {
    // Arrange: テスト用ファイルを作成
    let dir = ".instrux/instructions";
    let _ = fs::create_dir_all(dir);
    let file_path = format!("{}/test.md", dir);
    let file_content = "ファイルの内容です。";
    fs::write(&file_path, file_content).unwrap();

    let instructions = vec![InstructionItem::Variant1 {
        title: "ファイルタイトル".to_string(),
        body_file: "test.md".to_string(),
        description: None,
        disable: false,
        targets: InstructionItemVariant0Targets::Variant0(vec![Targets::All]),
    }];
    let mut output = String::new();

    // Act
    let result = process_instructions_common(
        &mut output,
        &instructions,
        0,
        |_| true,
    );

    // Assert
    assert!(result.is_ok());
    assert!(output.contains("ファイルタイトル"));
    assert!(output.contains(file_content));

    // クリーンアップ
    let _ = fs::remove_file(&file_path);
}
