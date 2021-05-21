mod lazer;
pub use lazer::*;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;

    #[test]
    fn test_print() {
        assert_eq!(3, 3);
    }
}