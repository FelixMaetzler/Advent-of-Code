pub fn number_to_digit_count(x: u64) -> u8 {
    match x.checked_ilog10() {
        Some(x) => (x + 1) as u8,
        None => 1,
    }
}
pub trait Joinable<T> {
    fn join(self, separator: &str) -> String;
}

impl<I, T> Joinable<T> for I
where
    I: Iterator<Item = T>,
    T: ToString,
{
    fn join(self, separator: &str) -> String {
        let mut iter = self.peekable();
        let mut result = String::new();

        if let Some(first) = iter.next() {
            result.push_str(&first.to_string());
        }

        for item in iter {
            result.push_str(separator);
            result.push_str(&item.to_string());
        }

        result
    }
}
pub fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a.abs();
    let mut b = b.abs();

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
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
