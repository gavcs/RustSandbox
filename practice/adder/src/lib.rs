#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(n: i32) -> i32 {
    n + 2
}

pub fn greeting(name: &str) -> String{
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Gavin");
        assert!(result.contains("Gavin"), "Greeting did not contain name, value was '{}'", result);
    }

    // #[test]
    // fn adding_two() {
    //     assert_eq!(add_two(24), 26);
    //     assert_ne!(add_two(3), 4);
    // }

    // #[test]
    // fn exploration() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    // #[test]
    // fn another() {
    //     panic!("Test failed!");
    // }

    // #[test]
    // fn larger_hold_smaller() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };
    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 6,
    //     };
    //     assert!(larger.can_hold(&smaller));
    // }

    // #[test]
    // fn smaller_hold_larger() {
    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 6,
    //     };
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };
    //     assert!(!smaller.can_hold(&larger));
    // }
}
