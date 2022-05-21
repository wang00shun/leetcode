/*
 * @lc app=leetcode.cn id=169 lang=rust
 *
 * [169] 多数元素
 */

// @lc code=start
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut count = 1;
        for index in 1..nums.len() {
            let current_num = nums[index];
            if current_num == result {
                count += 1;
            } else {
                if count == 0 {
                    count = 1;
                    result = current_num;
                } else {
                    count -= 1;
                }
            }
        }
        return result;
    }
}
// @lc code=end