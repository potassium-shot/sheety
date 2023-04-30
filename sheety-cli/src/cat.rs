use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct CatOptions {
    #[arg(short = 'i', long = "image")]
    pub images: Vec<String>,

    #[arg(short = 's', long = "size")]
    pub sizes: Vec<String>,

    #[arg(short = 'S', long = "default_size", default_value_t = String::new())]
    pub default_size: String,
}
