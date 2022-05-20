/*
 * @lc app=leetcode.cn id=31 lang=rust
 *
 * [31] 下一个排列
 */

// @lc code=start
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();
        if len <= 1 {
            return;
        }
        let mut left_index = len - 2;
        while left_index >= 0 {
            let left_num = nums[left_index];
            let mut min_max = i32::MAX;
            let mut min_max_index = left_index;

            for right_index in left_index + 1..len {
                let right_num = nums[right_index];
                if right_num > left_num && right_num < min_max {
                    min_max = right_num;
                    min_max_index = right_index;
                }
            }

            if min_max_index != left_index {
                nums[left_index] = min_max;
                nums[min_max_index] = left_num;
                &nums[left_index + 1..].sort();
                return;
            }

            if left_index == 0 {
                break;
            }
            left_index -= 1;
        }

        nums.sort();
    }
}
// @lc code=end

