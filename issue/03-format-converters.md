# 各形式への変換ロジック

## 概要
内部モデルから各ツール形式（Cline, Copilot, Cursor, Junie, Codex）への変換ロジックを実装する

## 詳細
- `formats/`モジュールに各ツール形式に特化した変換ロジックを実装
- 内部モデルからの変換と、各ツール形式から内部モデルへの変換の両方をサポート
- 以下の形式をサポート：
  - Cline: `.clinerules`
  - Copilot: `.github/copilot-instructions.md`
  - Cursor: `.cursor/rules`
  - Junie: `.junie/guidelines.md`
  - Codex: `.codex/instructions.md`
- 各形式の特殊な要件や制約に対応

## タスク
1. `src/formats/`に各ツール形式用のモジュールを作成
   - `cline.rs`
   - `copilot.rs`
   - `cursor.rs`
   - `junie.rs`
   - `codex.rs`
2. 各モジュールに「内部モデル→ツール形式」の変換関数を実装
3. 各モジュールに「ツール形式→内部モデル」の変換関数を実装
4. `formats/mod.rs`にレジストリと共通ファサードを実装
5. テストケースを作成

## 進捗状況
- ✅ `formats/`モジュール構造を作成
- ✅ 共通インターフェース `ToFormat` と `FromFormat` を定義
- ✅ Copilot変換器の実装
  - ✅ 内部モデル → Copilot形式 の変換（マークダウン形式）
  - ✅ Copilot形式 → 内部モデル の変換
  - ✅ テストケース作成
- ⬜ Cline変換器の実装
- ⬜ Cursor変換器の実装
- ✅ Junie変換器の実装
- ⬜ Codex変換器の実装

## Copilot形式の実装詳細

### 特徴
- マークダウンベースのシンプルな形式
- ヘッダーの階層で命令の構造を表現
- `.github/copilot-instructions.md`ファイルに保存

### 変換ロジック
- 内部モデル → Copilot形式:
  - 各命令アイテムを見出しに変換
  - 命令本文をマークダウン形式で出力
  - ネストされた命令は見出しレベルで表現
  - targetが"all"または"copilot"の命令のみを変換
- Copilot形式 → 内部モデル:
  - マークダウンのヘッダー構造を解析して命令階層を作成
  - 見出しのレベルに基づいて命令の構造を再構築

### テスト
- 内部モデルからCopilot形式への変換テスト
- Copilot形式から内部モデルへの変換テスト

## 優先度
中

## 関連項目
- 02-instrux-config-parser.md
- 04-generator-command.md
