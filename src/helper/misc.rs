pub fn number_to_digit_count(x: u64) -> u8 {
    match x.checked_ilog10() {
        Some(x) => (x + 1) as u8,
        None => 1,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_digit_count() {
        assert_eq!(number_to_digit_count(0), 1);
        assert_eq!(number_to_digit_count(1), 1);
        assert_eq!(number_to_digit_count(9), 1);
        assert_eq!(number_to_digit_count(10), 2);
        assert_eq!(number_to_digit_count(99), 2);
        assert_eq!(number_to_digit_count(100), 3);
    }
}
