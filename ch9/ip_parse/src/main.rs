use std::io::{self, Write};

fn main() {
    loop {
        print!("enter IPv4 address: ");

        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Coudn't read line");

        if let Ok(address) = input.trim_end().parse::<std::net::IpAddr>() {
            println!("{address:?}");
        } else {
            println!("Wrong IPv4 address format");
        }
    }
}
