/*
 * @lc app=leetcode.cn id=628 lang=rust
 *
 * [628] 三个数的最大乘积
 */

// @lc code=start
impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let len = nums.len();
        let mut multiple = 1;
        multiple *= nums[len - 1];
        let m1 = nums[0] * nums[1];
        let m2 = nums[len - 2] * nums[len - 3];
        if multiple > 0 {
            multiple *= core::cmp::max(m1, m2);
        } else {
            multiple *= core::cmp::min(m1, m2);
        }
        return multiple;
    }
}
// @lc code=end
