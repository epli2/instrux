# Instrux設定ファイルパーサー

## 概要
`.instrux/instrux.yaml`およびその他の設定ファイルを読み取り、内部モデルに変換する機能を実装する

## 詳細
- YAMLパーサーライブラリ（`serde_yaml`）を使用して設定ファイルを読み取る
- Markdownパーサーライブラリを使用して指示ファイルを読み取る
- 以下のファイル形式をサポート：
  - `.instrux/instrux.yaml` - 主設定ファイル
  - `.instrux/instructions/*.md` - Markdown形式の指示ファイル
  - `.instrux/instructions/*.yaml` - YAML形式の指示ファイル
- 設定ファイルの検証機能を実装

## タスク
1. Cargo.tomlに必要な依存関係を追加（`serde`, `serde_yaml`, Markdownパーサー）
2. `core/src/model/`に設定ファイル用のデータ構造を定義
3. `core/src/model/`にYAMLパーサー関数を実装
4. `core/src/model/`にMarkdownパーサー関数を実装
5. パース結果の検証ロジックを実装（`validate.rs`）
6. エラー処理を実装

## 優先度
高（最優先で実装すべき）

## 関連項目
- 01-cli-interface.md
- 03-format-converters.md
