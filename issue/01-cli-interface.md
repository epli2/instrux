# CLIインターフェース実装（clap）

## 概要
`clap`ライブラリを使用してInstruxのコマンドラインインターフェースを実装する

## 詳細
- `clap`のDeriveアプローチを使用してコマンドライン引数を定義する
- 以下のサブコマンドを実装：
  - `generate`: instrux設定に応じて各形式のinstructionファイルを生成
    - オプション: `--help`, `--dry-run`, `--force`, `--watch`
  - `init`: instruxの設定ディレクトリを生成
    - オプション: `--from <tool-name>`
  - `merge`: 各形式のinstructionファイルでの変更をinstrux設定へマージ
    - オプション: `--from <tool-name>`
  - `diff`: instrux設定と各形式の差分を表示
    - オプション: `--tool <tool-name>`
  - `validate`: instrux設定の構文・スキーマ検証
- コマンドライン引数をパースし、適切なコマンド処理関数に渡す

## タスク
1. Cargo.tomlに`clap`の依存関係を追加
2. `src/opts.rs`ファイルを作成し、コマンドライン引数の構造体を定義
3. `src/commands/`ディレクトリを作成し、各サブコマンドの基本的な処理関数を実装
4. `main.rs`を更新して、コマンドライン引数を処理するエントリーポイントを実装

## 優先度
高（最優先で実装すべき）

## 関連項目
- 02-instrux-config-parser.md
