use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct DelOptions {
    pub indices: String,

    #[arg(short = 'i', long = "image")]
    pub image: String,

    #[arg(short = 's', long = "size")]
    pub size: String,
}
