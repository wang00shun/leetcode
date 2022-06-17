/*
 * @lc app=leetcode.cn id=75 lang=rust
 *
 * [75] 颜色分类
 */

// @lc code=start
impl Solution {
    // TODO: 错误
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut zero = 0;
        let mut two = len - 1;
        let mut left = 0;
        while left < two {
            let num = nums[left];
            if num == 0 {
                nums[left] = nums[zero];
                nums[zero] = 0;
                zero += 1;
            }

            if num == 2 {
                nums[left] = nums[two];
                nums[two] = 2;
                two -= 1;
                left -= 1;
            }

            left += 1;
        }
    }

    pub fn sort_colors1(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut count0 = 0;
        let mut count1 = 0;
        for index in 0..len {
            let num = nums[index];
            if num == 0 {
                count0 += 1;
            } else if num == 1 {
                count1 += 1;
            }
        }
        for index in 0..count0 {
            nums[index] = 0;
        }
        for index in count0..count0 + count1 {
            nums[index] = 1;
        }
        for index in count0 + count1..len {
            nums[index] = 2;
        }
    }
}
// @lc code=end
