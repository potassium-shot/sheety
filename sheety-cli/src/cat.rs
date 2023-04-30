use std::env::Args;

use sheety::SpriteSheet;

use crate::{parse_file_div, Div, Options, ParseFileDivError};

pub(crate) fn command_cat(mut args: Args, options: Options) -> Result<(), String> {
    let mut files: Vec<(String, Div)> = Vec::new();

    loop {
        match parse_file_div(&mut args) {
            Ok(path_div) => files.push(path_div),
            Err(err) => match err {
                ParseFileDivError::NothingLeft => break,
                ParseFileDivError::ParseError(msg) => return Err(msg),
            },
        }
    }

    let sprites = files.into_iter().filter_map(|file_div| {
        Some(
            file_div
                .1
                .load(file_div.0)
                .ok()?
                .into_unordered()
                .expect("Sprite sheets should not be empty"),
        )
    });

    SpriteSheet::concat(sprites, options.distrib)
        .map_err(|err| format!("Could not concatenate sprites:\n"))?
        .save(options.output)
        .map_err(|err| format!("Could not save sprite sheet:\n"))?;

    Ok(())
}
