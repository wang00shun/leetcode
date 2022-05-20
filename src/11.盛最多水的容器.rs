
// @lc code=end

/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let length = height.len();
        if length < 2 {
            return 0;
        }
        let mut max = 0;
        let mut left = 0;
        let mut right = length - 1;
        while left < right {
            let left_height = height[left];
            let right_height = height[right];
            let calc_height;
            if left_height < right_height {
                calc_height = left_height;
            } else {
                calc_height = right_height;
            }
            let current = (calc_height as usize) * (right - left);
            if current > max {
                max = current;
            }
            if left_height < right_height {
                left += 1;
            } else {
                right -= 1;
            }
        }
        return max as i32;
    }
}
// @lc code=end

