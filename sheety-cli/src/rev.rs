use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct RevOptions {
    /// Image file of the target sprite sheet
    #[arg(short = 'i', long = "image")]
    pub image: String,

    /// Size of the target sprite sheet, e.g. `40x50` for pixel size, `3-3` for cell count,
    /// `single` for a single image; `10` is the same as `10x10`
    #[arg(short = 's', long = "size")]
    pub size: String,
}
