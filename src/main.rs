fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 0;
    let result = 1;
    println!("Hello, world! {}", result);
}

struct Solution {}

/*
 * @lc app=leetcode.cn id=283 lang=rust
 *
 * [283] 移动零
 */

// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zero_index = 0;
        let len = nums.len();
        for fast_index in 0..len {
            let num = nums[fast_index];
            if num != 0 {
                nums[fast_index] = 0;
                nums[zero_index] = num;
                zero_index += 1;
            }
        }
    }
}
// @lc code=end
