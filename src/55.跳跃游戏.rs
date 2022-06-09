/*
 * @lc app=leetcode.cn id=55 lang=rust
 *
 * [55] 跳跃游戏
 */

// @lc code=start
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        return Solution::can_jump1(nums);
    }

    pub fn can_jump1(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let max_index = len - 1;
        if len <= 1 {
            return true;
        }
        let mut max_position = nums[0] as usize;
        for index in 0..len {
            let cur = nums[index] as usize;
            if index == max_position && cur == 0 {
                return false;
            }
            let cur_max = cur + index;
            if cur_max >= max_index {
                return true;
            }
            if cur_max > max_position {
                max_position = cur_max;
            }
        }
        return false;
    }

    pub fn can_jump2(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let len_i = len as i32;
        let mut cur_index = 0;
        while true {
            let mut cur = nums[cur_index as usize];
            if cur_index + cur + 1 >= len_i {
                return true;
            }
            if cur == 0 {
                return false;
            }
            let mut can_max_index = cur_index + cur;
            let mut next_index = cur_index;
            for index in cur_index + 1..=can_max_index {
                let index = index as usize;
                if nums[index] + index as i32 > can_max_index {
                    next_index = index as i32;
                    can_max_index = nums[index] + index as i32;
                }
            }
            if next_index == cur_index {
                return false;
            }
            cur_index = next_index;
        }
        return false;
    }
}
// @lc code=end