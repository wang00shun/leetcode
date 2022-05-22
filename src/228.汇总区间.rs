/*
 * @lc app=leetcode.cn id=228 lang=rust
 *
 * [228] 汇总区间
 */

// @lc code=start
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();

        let len = nums.len();
        let mut left = 0;
        while left < len {
            let mut right = left;
            loop {
                right += 1;
                if right >= len || nums[right] - nums[right - 1] != 1 {
                    break;
                }
            }

            if left == right - 1 {
                result.push(nums[left].to_string());
            } else {
                result.push(format!("{}->{}", nums[left], nums[right - 1]));
            }

            left = right;
        }

        return result;
    }
}
// @lc code=end