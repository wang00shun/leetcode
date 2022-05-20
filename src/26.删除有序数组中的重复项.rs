
/*
 * @lc app=leetcode.cn id=26 lang=rust
 *
 * [26] 删除有序数组中的重复项
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut remove_count = 0;
        let mut last = i32::MAX;
        let mut current_index = 0;
        let len = nums.len();
        while current_index < len {
            let current_num = nums[current_index];
            if current_num == last {
                remove_count += 1;
            } else {
                let insert_index = current_index - remove_count;
                nums[insert_index] = current_num;
            }
            last = current_num;
            current_index += 1;
        }
        return (nums.len() - remove_count) as i32;
    }
}
// @lc code=end
