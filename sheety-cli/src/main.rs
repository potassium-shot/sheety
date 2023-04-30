extern crate sheety;

mod cat;

use std::{
    env::{args, Args},
    num::ParseIntError,
};

use sheety::{Distribution, SpriteSheet};

static VERSION_MSG: &str = include_str!("version.txt");
static USAGE_MSG: &str = include_str!("usage.txt");
static VER_USAGE_MSG: &str = concat!(include_str!("version.txt"), "\n", include_str!("usage.txt"));

fn main() {
    let options = match collect_options(args()) {
        Ok(options) => options,
        Err(msg) => {
            eprintln!("{}", msg);
            return;
        }
    };

    if let Err(msg) = read_command(args(), options) {
        eprintln!("{}", msg);
    }
}

fn read_command(mut args: Args, options: Options) -> Result<(), String> {
    let _ = args.next();

    if let Some(cmd) = args.next() {
        match cmd.as_str() {
            "cat" => cat::command_cat(args, options),
            dash if dash.starts_with('-') => {
                println!("{}", VER_USAGE_MSG);
                std::process::exit(0);
            }
            unknown => Err(format!("Unknown command '{unknown}'.")),
        }
    } else {
        println!("{}", VER_USAGE_MSG);
        std::process::exit(0);
    }
}

fn collect_options(mut args: Args) -> Result<Options, String> {
    let mut options = Options::default();

    while let Some(arg) = args.next() {
        if arg.starts_with('-') {
            match arg.as_str() {
                "-o" | "--output" => {
                    options.output = args.next().ok_or(String::from(
                        "Option '-o'/'--output' excpects a path as an argument.",
                    ))?;
                }
                "-d" | "--distribution" => {
                    options.distrib = match parse_distribution(&mut args) {
                        Ok(dist) => dist,
                        Err(msg) => return Err(msg),
                    }
                }
                "-h" | "--help" => {
                    println!("{}", USAGE_MSG);
                    std::process::exit(0);
                }
                "-v" | "--version" => {
                    println!("{}", VERSION_MSG);
                    std::process::exit(0);
                }
                unknown => return Err(format!("Unknown option '{unknown}'")),
            }
        }
    }

    Ok(options)
}

#[derive(Default)]
struct Options {
    output: String,
    distrib: Distribution,
}

fn parse_div(args: &mut Args) -> Result<Div, String> {
    if let Some(arg) = args.next() {
        match arg.as_str() {
            "cells" => Ok(Div::Cells(parse_div_size(args)?)),
            "sprite" => Ok(Div::Sprite(parse_div_size(args)?)),
            "singe" => Ok(Div::Single),
            unknown => Err(format!("Unknown div '{unknown}'.")),
        }
    } else {
        Err(String::from("Expected a div after a filepath."))
    }
}

fn parse_div_size(args: &mut Args) -> Result<(usize, usize), String> {
    if let Some(arg) = args.next() {
        Ok(if let Some(split) = arg.split_once('x') {
            (
                split.0.parse().map_err(|err: ParseIntError| {
                    format!("Could not parse div size:\n{}", err.to_string())
                })?,
                split.1.parse().map_err(|err: ParseIntError| {
                    format!("Could not parse div size:\n{}", err.to_string())
                })?,
            )
        } else {
            let result = arg.parse().map_err(|err: ParseIntError| {
                format!("Could not parse div size:\n{}", err.to_string())
            })?;

            (result, result)
        })
    } else {
        Err(String::from("Div expected a size."))
    }
}

enum Div {
    Cells((usize, usize)),
    Sprite((usize, usize)),
    Single,
}

impl Div {
    pub(crate) fn load(self, path: String) -> Result<SpriteSheet, String> {
        match self {
            Self::Cells(cell_size) => SpriteSheet::load_cell_size(path, cell_size),
            Self::Sprite(div) => SpriteSheet::load_div(path, div),
            Self::Single => SpriteSheet::load_div(path, (1, 1)),
        }
        .map_err(|err| format!("Could not load sprite/sprite sheet:\n"))
    }
}

fn parse_file_div(args: &mut Args) -> Result<(String, Div), ParseFileDivError> {
    if let Some(arg) = args.next() {
        match arg.as_str() {
            dash if dash.starts_with('-') => Err(ParseFileDivError::NothingLeft),
            _path => Ok((
                arg,
                parse_div(args).map_err(|err| ParseFileDivError::ParseError(err))?,
            )),
        }
    } else {
        Err(ParseFileDivError::NothingLeft)
    }
}

enum ParseFileDivError {
    ParseError(String),
    NothingLeft,
}

fn parse_distribution(args: &mut Args) -> Result<Distribution, String> {
    match args
        .next()
        .ok_or(String::from(
            "Option '-d'/'--distribution' excpects an argument.",
        ))?
        .as_str()
    {
        "columns" => Ok(Distribution::FixedColumns(
            args.next()
                .ok_or(String::from(
                    "Distribution 'columns' expects a number of columns as a parameter.",
                ))?
                .parse()
                .map_err(|err: ParseIntError| {
                    format!("Could not parse number of columns:\n{}", err.to_string())
                })?,
        )),
        "lines" => Ok(Distribution::FixedLines(
            args.next()
                .ok_or(String::from(
                    "Distribution 'lines' excpects a number of lines as a parameter.",
                ))?
                .parse()
                .map_err(|err: ParseIntError| {
                    format!("Could not parse number of lines:\n{}", err.to_string())
                })?,
        )),
        "packed" => Ok(Distribution::Packed(
            match args
                .next()
                .ok_or(String::from(
                    "Distribution 'packed' expects another parameter 'lines' or 'columns'.",
                ))?
                .as_str()
            {
                "columns" => true,
                "lines" => false,
                unknown => return Err(format!("Unknown packed distribution '{unknown}'")),
            },
        )),
        unknown => Err(format!("Unknown distribution '{unknown}'.")),
    }
}
