use crate::formats::get_converter;
use crate::model::types::{InstruxConfiguration, Targets};
use similar::{ChangeTag, TextDiff};
use std::fs;

/// 指定されたターゲット形式との内容差分を取得
pub fn diff_from_config(config: &InstruxConfiguration, target: Targets) -> Result<String, String> {
    let converter = get_converter(&target);
    let expected = converter.to_format(config)?;
    let path = converter.get_default_path();
    let current = fs::read_to_string(path).unwrap_or_default();
    Ok(make_diff(&current, &expected))
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
