use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct RevOptions {
    #[arg(short = 'i', long = "image")]
    pub image: String,

    #[arg(short = 's', long = "size")]
    pub size: String,
}
