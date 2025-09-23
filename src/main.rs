mod args;
mod dump;

use args::parse_args;
use std::process;
use crate::dump::generate_dump;

fn main() {
    let configs = match parse_args() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    println!("=== MeiliSearch Dump文件生成器 (Rust版本) ===");
    println!("输出目录: meilisearch_dump");
    println!("索引和文件配对:");
    for c in &configs {
        println!("  - 索引: {} -> 文件: {}", c.index, c.files.join(", "));
        println!("    displayed: {:?}", c.displayed);
        println!("    searchable: {:?}", c.searchable);
        println!("    filterable: {:?}", c.filterable);
    }
    println!();

    if let Err(e) = generate_dump(&configs) {
        eprintln!("❌ 生成dump失败: {}", e);
        process::exit(1);
    }
    println!("✅ dump生成完毕，可直接导入MeiliSearch！");
}
