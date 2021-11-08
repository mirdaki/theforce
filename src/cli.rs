use std::fs;

pub struct Args {
    pub path: std::path::PathBuf,
}

pub fn parse_arguments() -> Result<Args, String> {
    let path = match std::env::args().nth(1) {
        Some(path) => path,
        None => return Err("Args could not be parsed".to_string()),
    };

    Ok(Args {
        path: std::path::PathBuf::from(path),
    })
}

pub fn read_source(args: Args) -> Result<String, String> {
    match fs::read_to_string(args.path) {
        Ok(content) => Ok(content),
        Err(_) => Err("File could not be read".to_string()),
    }
}
