use rand::Rng;
use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};

fn main() -> std::io::Result<()> {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        print!("Please input your guess: ");
        stdout().flush()?;

        let mut guess = String::new();
        stdin().read_line(&mut guess)?;

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }

    Ok(())
}
