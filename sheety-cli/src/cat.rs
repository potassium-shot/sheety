use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct CatOptions {
    /// Each sprite sheet file path defined after a -i
    #[arg(short = 'i', long = "image")]
    pub images: Vec<String>,

    /// The size of each sprite sheet, defined after a -s; their must be as many as the number of images, or none if using -S
    #[arg(short = 's', long = "size")]
    pub sizes: Vec<String>,

    /// The default size, use it instead of -s if all sprite sheets have the same srpite size
    #[arg(short = 'S', long = "default_size", default_value_t = String::new())]
    pub default_size: String,
}
