//! # Adding 2 Test
//! 
//! `add2test` contains a single function `add_two` and is used to learn documentation and testing.

/// adds two to the number given.
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = add2test::add_two(arg);
/// 
/// assert_eq!(7, answer);
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}