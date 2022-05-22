/*
 * @lc app=leetcode.cn id=219 lang=rust
 *
 * [219] 存在重复元素 II
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if k == 0 {
            return false;
        }
        let mut set: HashSet<i32> = HashSet::new();
        for index in 0..nums.len() {
            let num = nums[index];
            if set.contains(&num) {
                return true;
            }
            if index as i32 >= k {
                set.remove(&nums[index - k as usize]);
            }
            set.insert(num);
        }
        return false;
    }
}
// @lc code=end