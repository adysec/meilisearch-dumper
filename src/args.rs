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
    pub file: String,
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
    let mut configs = Vec::new();
    for i in 0..n {
        configs.push(IndexConfig {
            index: cli.index[i].clone(),
            file: cli.files[i].clone(),
            displayed: get_attr(&cli.displayed, i),
            searchable: get_attr(&cli.searchable, i),
            filterable: get_attr(&cli.filterable, i),
        });
    }
    Ok(configs)
} 
