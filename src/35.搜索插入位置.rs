
/*
 * @lc app=leetcode.cn id=35 lang=rust
 *
 * [35] 搜索插入位置
 */

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        let mut left: i32 = 0;
        let mut right: i32 = (len - 1) as i32;
        while left <= right {
            let mid = (left + right) / 2;
            let num = nums[mid as usize];
            if num == target {
                return mid;
            }
            if num < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        return left as i32;
    }
}
// @lc code=end
