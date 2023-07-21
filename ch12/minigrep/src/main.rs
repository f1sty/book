use std::env;
use std::fs;
use std::path::Path;

struct Config<'a> {
    search_term: &'a str,
    file_path: &'a Path,
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

fn run(config: Config) {
    println!(
        "searching for '{}' at file {:?}\n",
        config.search_term, config.file_path
    );

    let contents = fs::read_to_string(config.file_path).expect("Couldn't open file");

    print!("file text:\n{contents}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    run(config);
}
