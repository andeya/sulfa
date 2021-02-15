pub mod number;
pub mod substring;


#[cfg(test)]
mod tests {
    use crate::basic_type::number::reverse_i32;
    use crate::basic_type::substring::{length_of_longest_substring, longest_palindrome};

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(2 + 2, 4);
        assert_eq!(3, length_of_longest_substring("ynyo".to_string()));
        assert_eq!(3, length_of_longest_substring("pwwkew".to_string()));
        assert_eq!(3, length_of_longest_substring("abcabcbb".to_string()));
        assert_eq!(7, length_of_longest_substring("bpfbhmipx".to_string()));
    }

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(2 + 2, 4);
        assert_eq!("xaabacxcabaax", longest_palindrome("xaabacxcabaaxcabaax".to_string()));
        assert_eq!("bab", longest_palindrome("babad".to_string()));
    }

    #[test]
    fn test_reverse_i32() {
        assert_eq!(0, reverse_i32(1534236469));
        assert_eq!(0, reverse_i32(-2147483648));
        assert_eq!(54321, reverse_i32(12345));
        assert_eq!(-54321, reverse_i32(-12345));
    }
}
