// ClineMultipleConverterのテスト
// TDD: まずテストを書く
//
// ClineMultipleConverterはoutputMode=multipleの場合、.clinerules/*.md形式で複数ファイルを出力する

use crate::formats::ToFormat;
use crate::formats::cline_multiple::ClineMultipleConverter;
use crate::model::types::{
    InstructionItem, InstructionItemVariant0Targets, InstruxConfiguration, Targets,
};
use std::collections::HashMap;

/// テスト用のサンプル設定を生成
fn create_test_config() -> InstruxConfiguration {
    let instruction1 = InstructionItem::Variant0 {
        title: "インストラクション1".to_string(),
        body: "本文1".to_string(),
        description: Some("説明1".to_string()),
        disable: false,
        targets: InstructionItemVariant0Targets::Variant0(vec![Targets::Cline]),
    };
    let instruction2 = InstructionItem::Variant0 {
        title: "インストラクション2".to_string(),
        body: "本文2".to_string(),
        description: None,
        disable: false,
        targets: InstructionItemVariant0Targets::Variant0(vec![Targets::Cline]),
    };
    let mut targets_map = HashMap::new();
    targets_map.insert(Targets::Cline, Default::default());
    let version = "0.1.0".parse().expect("Valid version string");
    InstruxConfiguration {
        instructions: vec![instruction1, instruction2],
        language: Default::default(),
        targets: targets_map,
        version,
    }
}

#[test]
fn test_cline_multiple_converter_to_format() {
    // ClineMultipleConverterのインスタンス生成
    let config = create_test_config();
    let converter = ClineMultipleConverter {};

    // to_formatでファイル名と内容のペアを取得
    let result = converter.to_format(&config);
    assert!(result.is_ok());

    // FormatResultからファイル名と内容のペアを取得
    let format_result = result.unwrap();
    let files = match format_result {
        crate::formats::FormatResult::Multiple(files) => files,
        _ => panic!("Expected FormatResult::Multiple"),
    };

    // 2つのファイルが生成されること
    assert_eq!(files.len(), 2);

    // ファイル名と内容の検証
    let expected_files = vec![
        (
            ".clinerules/インストラクション1.md",
            "# インストラクション1\n\n本文1\n",
        ),
        (
            ".clinerules/インストラクション2.md",
            "# インストラクション2\n\n本文2\n",
        ),
    ];
    for (expected_path, expected_content) in expected_files {
        let content = files.get(expected_path).expect("ファイルが存在すること");
        assert_eq!(content, expected_content);
    }
}
