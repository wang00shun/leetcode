fn main() {
    let input: Vec<Vec<char>> = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'E', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = "ABCESEEEFS".to_owned();

    // let mut a: u8 = 0;
    // let entry = &mut a;
    // *entry |= 1;
    // *entry |= 2;
    // *entry |= 4;
    // *entry |= 8;
    // println!("a:{}", a)
}

struct Solution {}

use std::collections::HashMap;

/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let count1 = nums1.len();
        let count2 = nums2.len();
        let left1 = 0;
        let right1 = count1 - 1;
        let left2 = 0;
        let right2 = count2 - 1;
        let remove_count = (count1 + count2 - 1) / 2;
        while remove_count > 0 {}
        return 0f64;
    }
}
// @lc code=end
