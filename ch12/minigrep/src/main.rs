use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let regex = &args[1];
    let path = Path::new(&args[2]);

    println!("searching for '{regex}' at file {path:?}");
}
