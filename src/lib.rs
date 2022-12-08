pub mod range_fill;
pub mod range_map;
pub mod fill_rect;

pub use range_fill::*;
pub use self::range_map::*;
pub use self::fill_rect::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

