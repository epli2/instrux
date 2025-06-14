# バリデーションコマンド実装

## 概要
`validate`コマンドを実装し、instrux設定の構文・スキーマ検証を行う

## 詳細
- `validate.rs`モジュールに検証ロジックを実装
- YAMLスキーマ検証を実装
- Markdown構文検証を実装
- ファイル参照の整合性検証を実装
- わかりやすいエラーメッセージの生成

## タスク
1. `core/src/validate.rs`に検証ロジックを実装
2. `cli/src/commands/validate.rs`にコマンド処理関数を実装
3. YAMLスキーマ定義を作成
4. Markdown検証ロジックを実装
5. エラーメッセージテンプレートを作成
6. エラーハンドリングを実装

## 優先度
中

## 関連項目
- 01-cli-interface.md
- 02-instrux-config-parser.md
