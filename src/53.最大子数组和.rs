
/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子数组和
 */

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;
        let mut sum = i32::MIN;
        for num in nums {
            if sum < 0 {
                sum = num;
            } else {
                sum += num;
            }
            if sum > max {
                max = sum;
            }
        }
        return max;
    }
}
// @lc code=end
