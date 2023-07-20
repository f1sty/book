pub fn add_two(number: isize) -> isize {
    number + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_test() {
        assert_eq!(4, add_two(2));
    }
}
