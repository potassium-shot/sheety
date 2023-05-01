#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(clippy::panic)]

extern crate anyhow;
extern crate clap;
extern crate sheety;

mod cat;
mod del;
mod rev;
mod slc;

use std::{ops::Range, path::PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use cat::CatOptions;
use clap::{Parser, Subcommand};
use del::DelOptions;
use rev::RevOptions;
use sheety::{Distribution, SpriteSheet, UnorderedSpriteSheet};
use slc::SlcOptions;

fn main() -> Result<()> {
    ParsedCommand::parse(Cli::parse())?.execute()
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// The distribution can be `"columns <num>"`, `"lines <num>"`, or `"packed columns/lines"`
    #[arg(short = 'd', long = "distribution", global = true, default_value_t = String::from("packed columns"))]
    distribution: String,

    /// The file to write the result into
    #[arg(short = 'o', long = "output", global = true, default_value_t = String::from("./sheety-result.png"))]
    output: String,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Concatenate one or more sprite sheets or sprites together
    Cat(CatOptions),

    /// Delete a sprite or a range of sprites from a sprite sheet
    Del(DelOptions),

    /// Slice and keep a sprite or a range of sprites from a sprite sheet
    Slc(SlcOptions),

    /// Reverse a sprite sheet
    Rev(RevOptions),
}

#[derive(Debug)]
enum ParsedCommand {
    Cat {
        files: Vec<FileDiv>,
        dist: Distribution,
        output: PathBuf,
    },
    Del {
        indices: UnboundRange,
        file: FileDiv,
        dist: Distribution,
        output: PathBuf,
    },
    Slc {
        indices: UnboundRange,
        file: FileDiv,
        dist: Distribution,
        output: PathBuf,
    },
    Rev {
        file: FileDiv,
        dist: Distribution,
        output: PathBuf,
    },
}

impl ParsedCommand {
    fn parse(cli: Cli) -> Result<Self> {
        Ok(match cli.command {
            Command::Cat(options) => {
                if options.sizes.is_empty() && !options.default_size.is_empty() {
                    // no sizes given and a default size given
                    Self::Cat {
                        files: options
                            .images
                            .into_iter()
                            .map(|f| {
                                Ok(FileDiv {
                                    file_path: PathBuf::from(f),
                                    div: Div::parse(options.default_size.as_str())?,
                                })
                            })
                            .collect::<Result<Vec<FileDiv>>>()?,
                        dist: parse_distribution(cli.distribution.as_str())?,
                        output: PathBuf::from(cli.output),
                    }
                } else if options.sizes.len() == options.images.len() {
                    // a size given for each sprite
                    Self::Cat {
                        files: std::iter::zip(options.images, options.sizes)
                            .map(|(img, size)| {
                                Ok(FileDiv {
                                    file_path: PathBuf::from(img),
                                    div: Div::parse(size.as_str())?,
                                })
                            })
                            .collect::<Result<Vec<FileDiv>>>()?,
                        dist: parse_distribution(cli.distribution.as_str())?,
                        output: PathBuf::from(cli.output),
                    }
                } else {
                    bail!("size count should be the same as the image count, or there should be a default size and nothing else");
                }
            }
            Command::Del(options) => Self::Del {
                indices: UnboundRange::parse(options.indices.as_str())?,
                file: FileDiv {
                    file_path: PathBuf::from(options.image),
                    div: Div::parse(options.size.as_str())?,
                },
                dist: parse_distribution(cli.distribution.as_str())?,
                output: PathBuf::from(cli.output),
            },
            Command::Slc(options) => Self::Slc {
                indices: UnboundRange::parse(options.indices.as_str())?,
                file: FileDiv {
                    file_path: PathBuf::from(options.image),
                    div: Div::parse(options.size.as_str())?,
                },
                dist: parse_distribution(cli.distribution.as_str())?,
                output: PathBuf::from(cli.output),
            },
            Command::Rev(options) => Self::Rev {
                file: FileDiv {
                    file_path: PathBuf::from(options.image),
                    div: Div::parse(options.size.as_str())?,
                },
                dist: parse_distribution(cli.distribution.as_str())?,
                output: PathBuf::from(cli.output),
            },
        })
    }

    fn execute(self) -> Result<()> {
        match self {
            Self::Cat {
                files,
                dist,
                output,
            } => {
                let list: Result<Vec<UnorderedSpriteSheet>> = files
                    .into_iter()
                    .map(|f| {
                        f.load()?
                            .into_unordered()
                            .context("could get sprites of sprite sheet")
                    })
                    .collect();

                SpriteSheet::concat(list?.into_iter(), dist)
                    .context("could not concatenate sprite sheets")?
                    .save(output)
                    .context("could not save file to disk")?;
            }
            Self::Del {
                indices,
                file,
                dist,
                output,
            } => {
                let mut sheet = file
                    .load()
                    .context("could not load sprite sheet")?
                    .into_unordered()
                    .context("could not get sprites from sprite sheet")?;

                let len = sheet.len();

                for i in indices.into_range(len).rev() {
                    if i >= sheet.len() {
                        bail!("specified del index/range `{i}` is out of bounds (max: `{len}`)",);
                    }

                    sheet.inner_mut().remove(i);
                }

                SpriteSheet::from_unordered(sheet, dist)
                    .save(output)
                    .context("could not save file to disk")?;
            }
            Self::Slc {
                indices,
                file,
                dist,
                output,
            } => {
                let mut sheet = file
                    .load()
                    .context("could not load sprite sheet")?
                    .into_unordered()
                    .context("could not get sprites from sprite sheet")?;

                let len = sheet.len();

                let indices = indices.into_range(len);

                for i in (indices.end..len).rev() {
                    if i >= sheet.len() {
                        bail!("specified slc index/range `{i}` is out of bounds (max: `{len}`)");
                    }

                    sheet.inner_mut().remove(i);
                }

                for i in (0..indices.start).rev() {
                    sheet.inner_mut().remove(i);
                }

                SpriteSheet::from_unordered(sheet, dist)
                    .save(output)
                    .context("could not save file to disk")?;
            }
            Self::Rev { file, dist, output } => {
                let mut sheet = file
                    .load()
                    .context("could not load sprite sheet")?
                    .into_unordered()
                    .context("could not get sprites from sprite sheet")?;

                sheet.inner_mut().reverse();

                SpriteSheet::from_unordered(sheet, dist)
                    .save(output)
                    .context("could not save file to disk")?;
            }
        }

        Ok(())
    }
}

fn parse_distribution(txt: &str) -> Result<Distribution> {
    let mut words = txt.split(' ');

    Ok(match words
        .next()
        .ok_or(anyhow!("distribution should be either 'columns', 'lines' or 'packed'"))?
    {
        "columns" => Distribution::FixedColumns(
            words
                .next()
                .ok_or(anyhow!("distribution 'columns' expects a number of columns"))?
                .parse()?
        ),
        "lines" => Distribution::FixedLines(
            words
                .next()
                .ok_or(anyhow!("distribution 'lines' expects a number of lines"))?
                .parse()?
        ),
		"packed" => Distribution::Packed(
			match words.next().ok_or(anyhow!("distribution 'packed' expects a priority on 'columns' or 'lines'"))? {
				"columns" => true,
				"lines" => false,
				unknown => bail!("distribution 'packed' expects a priority on 'columns' or 'lines', unknown priority '{unknown}'")
			}
		),
        unknown => bail!("unknown distribution '{unknown}'"),
    })
}

#[derive(Debug)]
struct FileDiv {
    file_path: PathBuf,
    div: Div,
}

impl FileDiv {
    fn load(self) -> Result<SpriteSheet> {
        Ok(match self.div {
            Div::Cells(div) => {
                SpriteSheet::load_div(self.file_path, div).context("could not load sprite sheet")?
            }
            Div::Sprite(size) => SpriteSheet::load_cell_size(self.file_path, size)
                .context("could not load sprite sheet")?,
            Div::Single => SpriteSheet::load_div(self.file_path, (1, 1))
                .context("could not load sprite sheet")?,
        })
    }
}

#[derive(Debug)]
enum Div {
    Cells((usize, usize)),
    Sprite((usize, usize)),
    Single,
}

impl Div {
    fn parse(txt: &str) -> Result<Self> {
        const PARSE_CONTEXT: &str = "could not parse size";

        if txt == "single" {
            Ok(Self::Single)
        } else if let Some((x, y)) = txt.split_once('-') {
            Ok(Self::Cells((
                x.parse().context(PARSE_CONTEXT)?,
                y.parse().context(PARSE_CONTEXT)?,
            )))
        } else if let Some((x, y)) = txt.split_once('x') {
            Ok(Self::Sprite((
                x.parse().context(PARSE_CONTEXT)?,
                y.parse().context(PARSE_CONTEXT)?,
            )))
        } else {
            let size = txt.parse().context(PARSE_CONTEXT)?;
            Ok(Self::Sprite((size, size)))
        }
    }
}

#[derive(Debug)]
struct UnboundRange {
    start: usize,
    end: Option<usize>,
}

impl UnboundRange {
    fn parse(txt: &str) -> Result<Self> {
        const PARSE_CONTEXT: &str = "could not parse deletion range";

        if let Some((min, max)) = txt.split_once('-') {
            Ok(Self {
                start: if min.is_empty() {
                    0
                } else {
                    min.parse().context(PARSE_CONTEXT)?
                },
                end: if max.is_empty() {
                    None
                } else {
                    Some(max.parse().context(PARSE_CONTEXT)?)
                },
            })
        } else {
            let both = txt.parse().context(PARSE_CONTEXT)?;
            Ok(Self {
                start: both,
                end: Some(both + 1),
            })
        }
    }

    fn into_range(self, upper_bound: usize) -> Range<usize> {
        self.start..self.end.unwrap_or(upper_bound)
    }
}
