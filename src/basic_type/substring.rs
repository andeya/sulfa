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
    max_len.max(set.len())
}


//Given two sorted arrays nums1 and nums2 of size m and n respectively, return t
//he median of the two sorted arrays.
//
// Follow up: The overall run time complexity should be O(log (m+n)).
//
//
// Example 1:
//
//
//Input: nums1 = [1,3], nums2 = [2]
//Output: 2.00000
//Explanation: merged array = [1,2,3] and median is 2.
//
//
// Example 2:
//
//
//Input: nums1 = [1,2], nums2 = [3,4]
//Output: 2.50000
//Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
//
//
// Example 3:
//
//
//Input: nums1 = [0,0], nums2 = [0,0]
//Output: 0.00000
//
//
// Example 4:
//
//
//Input: nums1 = [], nums2 = [1]
//Output: 1.00000
//
//
// Example 5:
//
//
//Input: nums1 = [2], nums2 = []
//Output: 2.00000
//
//
//
// Constraints:
//
//
// nums1.length == m
// nums2.length == n
// 0 <= m <= 1000
// 0 <= n <= 1000
// 1 <= m + n <= 2000
// -106 <= nums1[i], nums2[i] <= 106
//
pub fn longest_palindrome(s: String) -> String {
    if s.len() <= 1 {
        return s;
    }
    let (mut idx, mut l, mut r) = (0 as usize, 0 as usize, 0 as usize);
    let mut max_length = 0 as usize;
    let sbytes = s.as_bytes();
    while idx < s.len() - max_length / 2 {
        let mut left = idx;
        let mut right = idx;
        while left > 0 && sbytes[left - 1] == sbytes[idx] {
            left -= 1;
        }
        while right < s.len() - 1 && sbytes[right + 1] == sbytes[idx] {
            right += 1;
        }
        idx = right + 1;
        while left > 0 && right < s.len() - 1 && sbytes[left - 1] == sbytes[right + 1] {
            left -= 1;
            right += 1;
        }
        if max_length < right - left + 1 {
            max_length = right - left + 1;
            l = left;
            r = right;
        }
    }
    String::from(&s[l..=r])
}
