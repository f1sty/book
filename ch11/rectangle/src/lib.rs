#![allow(dead_code)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, rect: &Self) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller_test() {
        let smaller = Rectangle {
            width: 32,
            height: 32,
        };
        let larger = Rectangle {
            width: 64,
            height: 64,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger_test() {
        let smaller = Rectangle { width: 24, height: 24 };
        let larger = Rectangle { width: 32, height: 32};
        
        assert!(!smaller.can_hold(&larger));
    }
}
