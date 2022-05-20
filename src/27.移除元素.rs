
/*
 * @lc app=leetcode.cn id=27 lang=rust
 *
 * [27] 移除元素
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let len = nums.len();
        let mut remove_count = 0;
        let mut current_index = 0;
        while current_index < len {
            let current_num = nums[current_index];
            if current_num == val {
                remove_count += 1;
            } else {
                let insert_index = current_index - remove_count;
                if insert_index != current_index {
                    nums[insert_index] = current_num;
                }
            }
            current_index += 1;
        }
        return (len - remove_count) as i32;
    }
}
// @lc code=end
