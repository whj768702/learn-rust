#![allow(unused)]
use std::fmt::Error;

use anyhow_ext::{Context, Result};
use clap::Parser;

use ctrlc;
use std::{thread, time::Duration};

mod test;

#[derive(Parser)]
struct Cli {
    // 获取要查找到的字符串
    pattern: String,
    // 获取要查找到的文件
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    })
    .expect("Error setting Ctrl-C handler");

    let args = Cli::parse();

    // 获取文件内容
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file '{}'", args.path.display()))?;

    grrs_command::find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}
