#![allow(unused)]
use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};

mod test;

#[derive(Parser)]
struct Cli {
    // 获取要查找到的字符串
    pattern: String,
    // 获取要查找到的文件
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    // env_logger::init();
    // info!("starting up");
    // warn!("oops, nothing implemented!");
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).expect("Could not read file");
    // let content = std::fs::read_to_string(&args.path)
    //     .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    // 获取文件内容
    // let content = std::fs::read_to_string(&args.path)
    //     .with_context(|| format!("Could not read file '{}'", args.path.display()))?;

    grrs_command::find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}
