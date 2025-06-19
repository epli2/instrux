use crate::formats::{FormatResult, get_converter};
use crate::model::types::{InstruxConfiguration, InstruxConfigurationTargetsValue, Targets};
use similar::{ChangeTag, TextDiff};
use std::fs;

/// 指定されたターゲット形式との内容差分を取得
pub fn diff_from_config(config: &InstruxConfiguration, target: Targets) -> Result<String, String> {
    let converter = get_converter(&target, &InstruxConfigurationTargetsValue::default());
    let expected_result = converter.to_format(config)?;

    match expected_result {
        FormatResult::Single(expected) => {
            let path = converter.get_default_path();
            let current = fs::read_to_string(path).unwrap_or_default();
            Ok(make_diff(&current, &expected))
        }
        FormatResult::Multiple(files) => {
            // 複数ファイルの場合は一つの差分にまとめる
            let mut result = String::new();
            for (file_path, expected_content) in files {
                let current = fs::read_to_string(&file_path).unwrap_or_default();
                result.push_str(&format!("--- {}\n", file_path));
                result.push_str(&make_diff(&current, &expected_content));
                result.push_str("\n\n");
            }
            Ok(result)
        }
    }
}

/// 文字列同士の差分をANSIカラー付きで生成
fn make_diff(current: &str, expected: &str) -> String {
    let diff = TextDiff::from_lines(current, expected);
    let mut out = String::new();
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => {
                out.push_str(&format!("\x1b[31m-{}\x1b[0m", change));
            }
            ChangeTag::Insert => {
                out.push_str(&format!("\x1b[32m+{}\x1b[0m", change));
            }
            ChangeTag::Equal => {
                out.push_str(&format!(" {}", change));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::make_diff;

    #[test]
    fn test_make_diff_shows_insertion() {
        let current = "a\nb\n";
        let expected = "a\nb\nc\n";
        let out = make_diff(current, expected);
        assert!(out.contains("+c"));
    }
}
