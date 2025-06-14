# Instrux

[![CI](https://github.com/epli2/instrux/actions/workflows/rust.yml/badge.svg)](https://github.com/epli2/instrux/actions/workflows/rust.yml)

WIP

**Unified Instruction Manager for AI-powered Coding Tools**

Instrux is a Rust-powered CLI that lets you keep one source of truth for all your AI assistant rules.  
Write your guidelines once-use them across multiple tools.

## Project Structure

```
instrx/                 # ← Git root / Cargo workspace
├── Cargo.toml          # [workspace] と共通依存を宣言
├── crates/
│   ├── core/           # 変換ロジック（フォーマット非依存部）
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── model/          # モデル・YAML/MD/Package パーサ
│   │   │   ├── init.rs
│   │   │   ├── generate.rs
│   │   │   ├── merge.rs
│   │   │   ├── validate.rs
│   │   │   └── diff.rs
│   │   └── Cargo.toml
│   ├── formats/        # lib：各フォーマットの静的実装を集約
│   │   ├── src/
│   │   │   ├── lib.rs          # registry と facade
│   │   │   ├── cline.rs        # .clinerules
│   │   │   ├── copilot.rs      # copilot-instructions.md
│   │   │   ├── cursor.rs       # .cursor/rules
│   │   │   ├── junie.rs        # .junie/guidelines.md
│   │   │   └── codex.rs        # .codex/instructions.md
│   │   └── Cargo.toml
│   └── cli/             # bin：エンドユーザ CLI
│       ├── src/
│       │   ├── main.rs
│       │   ├── commands/       # init, generate, merge, validate, diff
│       │   └── opts.rs         # clap <–> Config
│       └── Cargo.toml
└── tests/
    ├── golden/          # スナップショット (in↔out) ファイル
    └── integration.rs
```

## 要件

### 対応ファイル形式

| 呼称    | 実ファイル名                    | ツール                             |
| ------- | ------------------------------- | ---------------------------------- |
| Cline   | .clinerules                     | Cline                              |
| Copilot | .github/copilot-instructions.md | GitHub Copilot                     |
| Cursor  | .cursor/rules                   | Cursor IDE                         |
| Junie   | .junie/guidelines.md            | JetBrains Junie                    |
| Codex   | .codex/instructions.md          | OpenAI Codex                       |
| Instrux | .instrux/instrux.yaml           | 本ツールで定義する共通フォーマット |

### CLI 仕様

| サブコマンド | 説明                                                                             | オプション                                 |
| ------------ | -------------------------------------------------------------------------------- | ------------------------------------------ |
| generate     | instrux 設定に応じて各形式の instruction ファイルを生成                          | `--help`, `--dry-run`, `--force` `--watch` |
| init         | instrux の設定ディレクトリを生成(from で特定形式の instruction ファイルから生成) | `--from <tool-name>`                       |
| merge        | 各形式の instruction ファイルでの変更を instrux 設定へマージ                     | `--from <tool-name>`                       |
| diff         | instrux 設定と各形式の diff                                                      | `--tool <tool-name>`                       |
| validate     | instrux 設定の構文・スキーマ検証                                                 |                                            |

### Instrux 設定の仕様

```
.instrx/
  instrux.yaml
  instructions/
    principles.md
    style.yaml
    deny.md
```

#### instrux.yaml

`schema/instrux.schema.json`参照
