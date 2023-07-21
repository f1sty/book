use std::fs;
use std::path::Path;
use std::error::Error;

pub struct Config<'a> {
    pub search_term: &'a str,
    pub file_path: &'a Path,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Self {
            search_term: &args[1],
            file_path: Path::new(&args[2]),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    print!("file text:\n{contents}");

    Ok(())
}
