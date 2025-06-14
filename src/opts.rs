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
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        force: bool,
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
