extern crate clap;
extern crate sheety;

mod cat;

use std::path::PathBuf;

use cat::CatOptions;
use clap::{Parser, Subcommand};
use sheety::{Distribution, SpriteSheet};

fn main() {
    ParsedCommand::parse(Cli::parse()).execute();
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(short = 'd', long = "distribution", global = true, default_value_t = String::from("packed columns"))]
    distribution: String,

    #[arg(short = 'o', long = "output", global = true, default_value_t = String::from("./sheety-result.png"))]
    output: String,
}

#[derive(Debug, Subcommand)]
enum Command {
    Cat(CatOptions),
}

#[derive(Debug)]
enum ParsedCommand {
    Cat {
        files: Vec<FileDiv>,
        dist: Distribution,
        output: PathBuf,
    },
}

impl ParsedCommand {
    fn parse(cli: Cli) -> Self {
        match cli.command {
            Command::Cat(options) => {
                if options.sizes.len() == 0 && options.default_size.len() > 0 {
                    // no sizes given and a default size given
                    Self::Cat {
                        files: options
                            .images
                            .into_iter()
                            .map(|f| FileDiv {
                                file_path: PathBuf::from(f),
                                div: Div::parse(options.default_size.as_str()),
                            })
                            .collect(),
                        dist: parse_distribution(cli.distribution.as_str()),
                        output: PathBuf::from(cli.output),
                    }
                } else if options.sizes.len() == options.images.len() {
                    // a size given for each sprite
                    Self::Cat {
                        files: std::iter::zip(options.images, options.sizes)
                            .map(|(img, size)| FileDiv {
                                file_path: PathBuf::from(img),
                                div: Div::parse(size.as_str()),
                            })
                            .collect(),
                        dist: parse_distribution(cli.distribution.as_str()),
                        output: PathBuf::from(cli.output),
                    }
                } else {
                    panic!("size count should be the same as the image count, or there should be a default size and nothing else");
                }
            }
        }
    }

    fn execute(self) {
        match self {
            Self::Cat {
                files,
                dist,
                output,
            } => {
                SpriteSheet::concat(
                    files.into_iter().map(|f| {
                        f.load()
                            .into_unordered()
                            .expect("could get sprites of sprite sheet:")
                    }),
                    dist,
                )
                .expect("could not concatenate:")
                .save(output)
                .expect("could not save file to disk:");
            }
        }
    }
}

fn parse_distribution(txt: &str) -> Distribution {
    let mut words = txt.split(' ');

    match words
        .next()
        .expect("distribution should be either 'columns', 'lines' or 'packed'")
    {
        "columns" => Distribution::FixedColumns(
            words
                .next()
                .expect("distribution 'columns' expects a number of columns")
                .parse()
                .expect("could not parse distribution column count:"),
        ),
        "lines" => Distribution::FixedLines(
            words
                .next()
                .expect("distribution 'lines' expects a number of lines")
                .parse()
                .expect("could not parse distribution line count:"),
        ),
		"packed" => Distribution::Packed(
			match words.next().expect("distribution 'packed' expects a priority on 'columns' or 'lines'") {
				"columns" => true,
				"lines" => false,
				unknown => panic!("distribution 'packed' expects a priority on 'columns' or 'lines', unknown priority '{unknown}'")
			}
		),
        unknown => panic!("unknown distribution '{unknown}'"),
    }
}

#[derive(Debug)]
struct FileDiv {
    file_path: PathBuf,
    div: Div,
}

impl FileDiv {
    fn load(self) -> SpriteSheet {
        match self.div {
            Div::Cells(div) => {
                SpriteSheet::load_div(self.file_path, div).expect("could not load sprite sheet:")
            }
            Div::Sprite(size) => SpriteSheet::load_cell_size(self.file_path, size)
                .expect("could not load sprite sheet:"),
            Div::Single => {
                SpriteSheet::load_div(self.file_path, (1, 1)).expect("could not load sprite sheet:")
            }
        }
    }
}

#[derive(Debug)]
enum Div {
    Cells((usize, usize)),
    Sprite((usize, usize)),
    Single,
}

impl Div {
    fn parse(txt: &str) -> Self {
        if txt == "single" {
            Self::Single
        } else {
            if let Some((x, y)) = txt.split_once('-') {
                Self::Cells((
                    x.parse().expect("could not parse size"),
                    y.parse().expect("could not parse size"),
                ))
            } else if let Some((x, y)) = txt.split_once('x') {
                Self::Sprite((
                    x.parse().expect("could not parse size"),
                    y.parse().expect("could not parse size"),
                ))
            } else {
                let size = txt.parse().expect("could not parse size");
                Self::Sprite((size, size))
            }
        }
    }
}
