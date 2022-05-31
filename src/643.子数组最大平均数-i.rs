/*
 * @lc app=leetcode.cn id=643 lang=rust
 *
 * [643] 子数组最大平均数 I
 */

// @lc code=start
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut max = i32::MIN;
        let mut result: i32 = 0;
        let k_len = k as usize;
        let k_f = k as f64;
        for index in 0..k_len {
            let cur = nums[index];
            result += cur;
        }
        if result > max {
            max = result;
        }
        for index in k_len..nums.len() {
            result -= nums[index - k_len];
            let cur = nums[index];
            result += cur;
            if result > max {
                max = result;
            }
        }
        return (max as f64) / k_f;
    }
}
// @lc code=end