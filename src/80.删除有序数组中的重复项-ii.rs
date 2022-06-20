/*
 * @lc app=leetcode.cn id=80 lang=rust
 *
 * [80] 删除有序数组中的重复项 II
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        if len < 3 {
            return len as i32;
        }
        let mut left = 1;
        for index in 2..len {
            let num = nums[index];
            if num != nums[left - 1] {
                nums[left + 1] = num;
                left += 1;
            }
        }

        return (left + 1) as i32;
    }
}
// @lc code=end
