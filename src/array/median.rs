use std::cmp::{max, min};

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
// Time complexity O(logmin(m,n)))
// Space complexity: O(1)
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    if nums1.len() > nums2.len() {
        return find_median_sorted_arrays(nums2, nums1);
    }
    let (m, n) = (nums1.len(), nums2.len());
    let (mut left, mut right) = (0, m);
    let (mut median1, mut median2) = (0, 0);
    while left <= right {
        let i = (left + right) / 2;
        let j = (m + n + 1) / 2 - i;
        let mut nums_im1 = i32::MIN;
        if i != 0 {
            nums_im1 = nums1[i - 1];
        }
        let mut nums_i = i32::MAX;
        if i != m {
            nums_i = nums1[i]
        }
        let mut nums_jm1 = i32::MIN;
        if j != 0 {
            nums_jm1 = nums2[j - 1];
        }
        let mut nums_j = i32::MAX;
        if j != n {
            nums_j = nums2[j]
        }
        if nums_im1 <= nums_j {
            median1 = max(nums_im1, nums_jm1);
            median2 = min(nums_i, nums_j);
            left = i + 1
        } else {
            right = i - 1
        }
    }
    if (m + n) % 2 == 0 {
        return (median1 + median2) as f64 / 2.0
    }
    return median1 as f64;
}
