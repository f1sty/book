use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (regexp, file_path) = parse_config(&args);

    println!("searching for '{regexp}' at file {file_path:?}\n");

    let contents = fs::read_to_string(file_path).expect("Couldn't open file {path:?}");

    print!("file text:\n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &Path) {
    let regex = &args[1];
    let file_path = Path::new(&args[2]);

    (regex, file_path)
}
