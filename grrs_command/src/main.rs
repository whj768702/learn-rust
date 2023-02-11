#![allow(unused)]
use std::fmt::Error;

use anyhow_ext::{Context, Result};
use clap::Parser;

mod test;

#[derive(Parser)]
struct Cli {
    // 获取要查找到的字符串
    pattern: String,
    // 获取要查找到的文件
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // 获取文件内容
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file '{}'", args.path.display()))?;

    grrs_command::find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}
