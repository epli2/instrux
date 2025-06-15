// コマンドライン引数の構造体定義（clapのDerive利用）
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "instrux")]
#[command(about = "Instrux CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Generate {
        /// dry-run: ファイルを生成せず、差分を表示(diffと同等)
        #[arg(long)]
        dry_run: bool,
        /// overwrite: 既存ファイルを上書きし、バックアップファイル(.bak)を作成
        #[arg(long)]
        overwrite: bool,
        /// force: バックアップを作成せず強制上書き
        #[arg(long)]
        force: bool,
        /// watch: ファイル変更を監視して自動生成
        #[arg(long)]
        watch: bool,
    },
    Init {
        #[arg(long, value_name = "tool-name")]
        from: Option<String>,
    },
    Merge {
        #[arg(long, value_name = "tool-name")]
        from: Option<String>,
    },
    Diff {
        #[arg(long, value_name = "tool-name")]
        tool: Option<String>,
    },
    Validate,
}
