#![allow(unused)]
use clap::Parser;

#[derive(Parser)]
struct Cli {
    // 获取要查找到的字符串
    pattern: String,
    // 获取要查找到的文件
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    // 获取文件内容
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    println!("Hello, world!");
}
