/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        nums.sort();

        for i in 0..nums.len() {
            let num1 = nums[i];
            if i != 0 {
                if num1 == nums[i - 1] {
                    continue;
                }
            }
            let target = -num1;
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let num2 = nums[left];
                let num3 = nums[right];
                if (num2 + num3) <= target {
                    if (num2 + num3) == target {
                        result.push(vec![num1, num2, num3]);
                        right = nums.len() - 1;
                    }
                    loop {
                        left += 1;
                        if left >= right || nums[left] > num2 {
                            break;
                        }
                    }
                } else {
                    loop {
                        right -= 1;
                        if left >= right || nums[right] < num3 {
                            break;
                        }
                    }
                }
            }
        }

        return result;
    }
}
// @lc code=end
