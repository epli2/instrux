# 各形式への変換ロジック

## 概要
内部モデルから各ツール形式（Cline, Copilot, Cursor, Junie, Codex）への変換ロジックを実装する

## 詳細
- `formats/`クレートに各ツール形式に特化した変換ロジックを実装
- 内部モデルからの変換と、各ツール形式から内部モデルへの変換の両方をサポート
- 以下の形式をサポート：
  - Cline: `.clinerules`
  - Copilot: `copilot-instructions.md`
  - Cursor: `.cursor/rules`
  - Junie: `.junie/guidelines.md`
  - Codex: `.codex/instructions.md`
- 各形式の特殊な要件や制約に対応

## タスク
1. `formats/src/`に各ツール形式用のモジュールを作成
   - `cline.rs`
   - `copilot.rs`
   - `cursor.rs`
   - `junie.rs`
   - `codex.rs`
2. 各モジュールに「内部モデル→ツール形式」の変換関数を実装
3. 各モジュールに「ツール形式→内部モデル」の変換関数を実装
4. `formats/src/lib.rs`にレジストリと共通ファサードを実装
5. テストケースを作成

## 優先度
中

## 関連項目
- 02-instrux-config-parser.md
- 04-generator-command.md
