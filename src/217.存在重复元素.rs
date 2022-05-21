/*
 * @lc app=leetcode.cn id=217 lang=rust
 *
 * [217] 存在重复元素
 */

// @lc code=start

use std::collections::HashSet;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for index in 0..nums.len() {
            if set.contains(&nums[index]) {
                return true;
            }
            set.insert(nums[index]);
        }
        return false;
    }
}
// @lc code=end