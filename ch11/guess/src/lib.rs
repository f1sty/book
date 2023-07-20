struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Self {
        if value < 1 {
            panic!("Guess must be equal or greater than 1, got: {value}");
        } else if value > 100 {
            panic!("Guess must be equal or less than 100, got: {value}");
        }
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "or less than 100")]
    fn greater_than_100_test() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "or greater than 1")]
    fn less_than_1_test() {
        Guess::new(0);
    }
}
