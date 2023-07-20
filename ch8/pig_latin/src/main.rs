use std::io::Result as IoResult;
use std::io::{stdin, stdout, Write};

fn main() -> IoResult<()> {
    print!("enter your phrase: ");
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let pig_latin: Vec<String> = input
        .split_whitespace()
        .map(|word| {
            if word.starts_with(['a', 'e', 'i', 'o', 'u']) {
                let mut pig_word = String::from(word);
                pig_word.push_str("-hay");
                pig_word
            } else {
                let first_char = word.chars().nth(0).unwrap();
                let mut pig_word: String = word.chars().skip(1).collect();
                pig_word.push('-');
                pig_word.push(first_char);
                pig_word.push_str("ay");
                pig_word
            }
        })
        .collect();
    let pig_latin = pig_latin.join(" ");

    println!("{pig_latin}");

    Ok(())
}
