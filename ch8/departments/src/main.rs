use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::str::FromStr;

#[derive(Debug)]
enum Command {
    Add { name: String, department: String },
    ListDepartment(String),
    List,
    Quit,
    Unknown,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();

        match (tokens[0].to_lowercase().as_ref(), tokens.len()) {
            ("add", 4) => {
                let name = tokens[1].to_string();
                let department = tokens[3].to_string();
                Ok(Command::Add { name, department })
            }
            ("list", 2) => {
                let department = tokens[1].to_string();
                Ok(Command::ListDepartment(department))
            }
            ("list", 1) => Ok(Command::List),
            ("quit", 1) => Ok(Command::Quit),
            _ => Ok(Command::Unknown),
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        print!("> ");
        stdout().flush()?;
        stdin().read_line(&mut input)?;

        match input.parse() {
            Ok(Command::Add { name, department }) => {
                let employees = company.entry(department).or_default();
                employees.push(name);
            }
            Ok(Command::ListDepartment(department)) => {
                let employees = company.entry(department).or_default();
                employees.sort();
                employees.iter().for_each(|employee| println!("{employee}"));
            }
            Ok(Command::List) => {
                let mut departments: Vec<_> = company.keys().collect();
                departments.sort();
                departments.iter().for_each(|department| {
                    println!("{department}:");

                    let mut employees = company.get(*department).unwrap().to_owned();

                    employees.sort();
                    employees
                        .iter()
                        .for_each(|employee| println!("\t{employee}"));
                });
            }
            Ok(Command::Quit) => break,
            Ok(Command::Unknown) => eprintln!("unknown command"),
            Err(error) => eprintln!("error: {error}"),
        }

        input.clear();
    }

    Ok(())
}
