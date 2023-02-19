#[cfg(test)]
mod test_math {
    use rust_test::math::Math;

    #[test]
    fn test_add() {
        assert_eq!(Math::add(4, 5), 9);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(Math::subtract(10, 5), 5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(Math::multiply(10, 5), 50);
    }

    #[test]
    fn divide_test() {
        assert_eq!(Math::divide(10, 5), 2);
    }
}
