use std::cmp::max;

//Given a string s, find the length of the longest substring without repeating c
//haracters.
//
//
// Example 1:
//
//
//Input: s = "abcabcbb"
//Output: 3
//Explanation: The answer is "abc", with the length of 3.
//
//
// Example 2:
//
//
//Input: s = "bbbbb"
//Output: 1
//Explanation: The answer is "b", with the length of 1.
//
//
// Example 3:
//
//
//Input: s = "pwwkew"
//Output: 3
//Explanation: The answer is "wke", with the length of 3.
//Notice that the answer must be a substring, "pwke" is a subsequence and not a
//substring.
//
//
// Example 4:
//
//
//Input: s = ""
//Output: 0
//
//
//
// Constraints:
//
//
// 0 <= s.length <= 5 * 104
// s consists of English letters, digits, symbols and spaces.
//
pub fn length_of_longest_substring(s: String) -> usize {
    let mut set = Vec::new();
    let mut max_len = 0;
    for x in s.chars() {
        if let Some(i) = set.iter().position(|&c| c == x) {
            if set.len() > max_len {
                max_len = set.len()
            }
            let new_len = set.len() - (i + 1);
            set.copy_within(i + 1.., 0);
            set.truncate(new_len);
        }
        set.push(x);
    }
    max(max_len, set.len())
}

#[cfg(test)]
mod tests {
    use crate::substring::length_of_longest_substring;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(2 + 2, 4);
        assert_eq!(3, length_of_longest_substring("ynyo".to_string()));
        assert_eq!(3, length_of_longest_substring("pwwkew".to_string()));
        assert_eq!(3, length_of_longest_substring("abcabcbb".to_string()));
        assert_eq!(7, length_of_longest_substring("bpfbhmipx".to_string()));
    }
}
