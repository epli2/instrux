# Instrux

[![CI](https://github.com/epli2/instrux/actions/workflows/rust.yml/badge.svg)](https://github.com/epli2/instrux/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/epli2/instrux/branch/main/graph/badge.svg?token=YOUR_CODECOV_TOKEN_HERE)](https://codecov.io/gh/epli2/instrux)
<!-- TODO: Replace YOUR_CODECOV_TOKEN_HERE with the actual token if needed, or remove if public repo and token is not strictly necessary -->

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
│   │   │   └── agentsmd.rs        # AGENTS.md (Google Jules/OpenAI Codex)
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

| 呼称      | 実ファイル名                    | ツール                                                                                                                                                                                                   |
| --------- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Cline     | .clinerules                     | Cline                                                                                                                                                                                                    |
| Copilot   | .github/copilot-instructions.md | GitHub Copilot                                                                                                                                                                                           |
| Cursor    | .cursor/rules                   | Cursor IDE                                                                                                                                                                                               |
| Junie     | .junie/guidelines.md            | JetBrains Junie                                                                                                                                                                                          |
| AGENTS.md | AGENTS.md                       | [Google Jules](https://jules.google/docs/#include-agentsmd-file) / [OpenAI Codex](https://github.com/openai/codex/blob/b73426c1c40187ca13c74c03912a681072c2884f/codex-cli/src/utils/config.ts#L229-L233) |
| Instrux   | .instrux/instrux.yaml           | 本ツールで定義する共通フォーマット                                                                                                                                                                       |

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

## Code Coverage

This project uses `cargo-llvm-cov` for code coverage. To generate a coverage report locally:

1. Install `cargo-llvm-cov`:
   ```bash
   cargo install cargo-llvm-cov --locked
   ```
2. Install `llvm-tools`:
   ```bash
   # On Debian/Ubuntu
   sudo apt-get update && sudo apt-get install -y llvm-tools
   # On macOS (using Homebrew)
   # brew install llvm
   ```
3. Run the coverage command from the root of the project:
   ```bash
   cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
   ```
   This will generate an LCOV report at `lcov.info`. You can then use tools like `genhtml` (part of LCOV) to view it as HTML:
   ```bash
   # Install lcov if you don't have it
   # sudo apt-get install lcov (Debian/Ubuntu)
   # brew install lcov (macOS)
   genhtml lcov.info --output-directory coverage-html
   ```
   Then open `coverage-html/index.html` in your browser.

Coverage is also automatically tracked on CI using Codecov.
