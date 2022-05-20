
/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */
// @lc code=start
use std::{collections::HashMap, vec};
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashMap = HashMap::<i32, i32>::new();
        for (current_index, &current_numm) in nums.iter().enumerate() {
            let current_index = current_index as i32;
            let search_num = target - current_numm;
            if let Some(&search_index) = hashMap.get(&search_num) {
                return vec![search_index, current_index];
            } else {
                hashMap.entry(current_numm).or_insert(current_index);
            }
        }
        return vec![];
    }
}
// @lc code=end
