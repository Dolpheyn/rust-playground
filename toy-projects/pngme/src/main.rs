mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::{App, Arg, SubCommand};
use std::error;
use std::fmt;

pub type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct StrError<'a>(&'a str);
impl<'a> error::Error for StrError<'a> {}

impl<'a> fmt::Display for StrError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to the Display impl for `&str`:
        self.0.fmt(f)
    }
}

fn main() -> Result<()> {
    // Subcommands:
    //  Encode:
    //      Parameters: File Path, Chunk type, Message, Output file(optional)
    //  Decode
    //      Parameters: File Path, Chunk type
    //  Remove
    //      Parameters: File Path, Chunk type
    //  Print
    //      Parameters: File Path

    let matches = App::new("pngme")
        .subcommands(vec![
            SubCommand::with_name("encode")
                .about("Encode image with message")
                .args(&[
                    Arg::with_name("input file").index(1).required(true),
                    Arg::with_name("chunk type").index(2).required(true),
                    Arg::with_name("message").index(3).required(true),
                    Arg::with_name("output file").index(4).required(false),
                ]),
            SubCommand::with_name("decode")
                .about("Decode message in image")
                .args(&[
                    Arg::with_name("input file").index(1).required(true),
                    Arg::with_name("chunk type").index(2).required(true),
                ]),
            SubCommand::with_name("remove")
                .about("Decode message in image")
                .args(&[
                    Arg::with_name("input file").index(1).required(true),
                    Arg::with_name("chunk type").index(2).required(true),
                ]),
            SubCommand::with_name("print")
                .about("Decode message in image")
                .args(&[Arg::with_name("input file").index(1).required(true)]),
        ])
        .get_matches();

    if let Some(matches_encode) = matches.subcommand_matches("encode") {
        let input_file_path = matches_encode.value_of("input file").unwrap();
        let chunk_type = matches_encode.value_of("chunk type").unwrap();
        let message = matches_encode.value_of("message").unwrap();
        println!("{:?}", input_file_path);
        println!("{:?}", chunk_type);
        println!("{:?}", message);
    }

    if let Some(matches_encode) = matches.subcommand_matches("decode") {
        let input_file_path = matches_encode.value_of("input file").unwrap();
        let chunk_type = matches_encode.value_of("chunk type").unwrap();
        println!("{:?}", input_file_path);
        println!("{:?}", chunk_type);
    }

    if let Some(matches_encode) = matches.subcommand_matches("remove") {
        let input_file_path = matches_encode.value_of("input file").unwrap();
        let chunk_type = matches_encode.value_of("chunk type").unwrap();
        println!("{:?}", input_file_path);
        println!("{:?}", chunk_type);
    }

    if let Some(matches_encode) = matches.subcommand_matches("print") {
        let input_file_path = matches_encode.value_of("input file").unwrap();
        println!("{:?}", input_file_path);
    }

    Ok(())
}
