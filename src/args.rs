use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long, required = true)]
    pub index: Vec<String>,
    #[arg(long, required = true)]
    pub files: Vec<String>,
    #[arg(long)]
    pub displayed: Vec<String>,
    #[arg(long)]
    pub searchable: Vec<String>,
    #[arg(long)]
    pub filterable: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct IndexConfig {
    pub index: String,
    pub files: Vec<String>,
    pub displayed: Vec<String>,
    pub searchable: Vec<String>,
    pub filterable: Vec<String>,
}

pub fn parse_args() -> Result<Vec<IndexConfig>, String> {
    let cli = Cli::parse();
    let n = cli.index.len();
    if cli.files.len() != n {
        return Err(format!("--index 和 --files 数量不一致: {} vs {}", n, cli.files.len()));
    }
    let get_attr = |v: &Vec<String>, i: usize| {
        if i < v.len() {
            v[i].split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
        } else {
            Vec::new()
        }
    };
    use std::collections::HashMap;
    let mut map: HashMap<String, IndexConfig> = HashMap::new();
    let mut order: Vec<String> = Vec::new();
    for i in 0..n {
        let idx = cli.index[i].trim().to_string();
        let files: Vec<String> = cli.files[i]
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let displayed = get_attr(&cli.displayed, i);
        let searchable = get_attr(&cli.searchable, i);
        let filterable = get_attr(&cli.filterable, i);

        if !map.contains_key(&idx) {
            order.push(idx.clone());
            map.insert(idx.clone(), IndexConfig {
                index: idx.clone(),
                files,
                displayed,
                searchable,
                filterable,
            });
        } else {
            // 合并文件，属性以最后一次为准
            if let Some(cfg) = map.get_mut(&idx) {
                cfg.files.extend(files);
                cfg.displayed = displayed;
                cfg.searchable = searchable;
                cfg.filterable = filterable;
            }
        }
    }
    let mut configs: Vec<IndexConfig> = Vec::with_capacity(order.len());
    for k in order {
        if let Some(cfg) = map.remove(&k) {
            configs.push(cfg);
        }
    }
    Ok(configs)
} 
