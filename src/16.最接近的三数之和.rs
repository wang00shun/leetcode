/*
 * @lc app=leetcode.cn id=16 lang=rust
 *
 * [16] 最接近的三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let length = nums.len();
        if length < 3 {
            return i32::MAX;
        }
        nums.sort();
        let mut min_diff = i32::MAX;
        for i in 0..length - 3 {
            let num1 = nums[i];
            let outer_target = target - num1;
            let mut left = i + 1;
            let mut right = length - 1;
            while left < right {
                let num2 = nums[left];
                let num3 = nums[right];
                let diff = outer_target - num2 - num3;
                if diff == 0 {
                    return target;
                }
                if diff > 0 {
                    loop {
                        left += 1;
                        if left >= right || nums[left] != nums[left - 1] {
                            break;
                        }
                    }
                }
                if diff < 0 {
                    loop {
                        right -= 1;
                        if left >= right || nums[right] != nums[right + 1] {
                            break;
                        }
                    }
                }
                if min_diff.abs() > diff.abs() {
                    min_diff = diff;
                }
            }
        }

        return target - min_diff;
    }
}

// @lc code=end
