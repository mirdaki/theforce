extern crate clap;
use clap::{App, Arg, ArgMatches};

use std::fs;

pub fn parse_arguments() -> ArgMatches<'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .after_help(&*format!(
            "For more information, please visit {}",
            env!("CARGO_PKG_HOMEPAGE")
        ))
        .arg(
            Arg::with_name("PATH")
                .help("The path to a `.force` file to run.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("parse_error")
            .long("ERROR_AFTER_PARSE")
            .help("Indicates to error before execution if specified.")
            .required(false)
            .index(2),
        )
        .get_matches()
}

pub fn read_source(args: ArgMatches) -> Result<String, String> {
    match fs::read_to_string(args.value_of("PATH").unwrap()) {
        Ok(content) => Ok(content),
        Err(_) => Err("File could not be read".to_string()),
    }
}
