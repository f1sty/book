use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

fn main() -> std::io::Result<()> {
    print!("enter numbers separated by space: ");
    stdout().flush()?;

    let mut list = String::new();
    stdin().read_line(&mut list)?;

    let mut list: Vec<i32> = list
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect();
    list.sort();

    let median_position = list.len() / 2;
    let median = list[median_position];
    let mut frequencies: HashMap<i32, u32> = HashMap::new();

    for number in list {
        let count = frequencies.entry(number).or_insert(0);
        *count += 1;
    }

    let mode = frequencies
        .iter()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap()
        .0;

    dbg!(median);
    dbg!(mode);

    Ok(())
}
