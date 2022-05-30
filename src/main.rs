use std::num;

fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 0;
    let result = 1;
    println!("Hello, world! {}", result);
}

struct Solution {}

/*
 * @lc app=leetcode.cn id=643 lang=rust
 *
 * [643] 子数组最大平均数 I
 */

// @lc code=start
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut max = f64::MIN;
        let mut result: f64 = 0.0;
        let mut vec = vec![];
        let k_len = k as usize;
        let k_f = k as f64;
        for index in 0..k_len {
            let cur = (nums[index] as f64) / k_f;
            vec.push(cur);
            result += cur;
        }
        for index in k_len..nums.len() {
            result -= vec.remove(0);
            let cur = (nums[index] as f64) / k_f;
            vec.push(cur);
            result += cur;
            if result > max {
                max = result;
            }
        }
        return max;
    }
}
// @lc code=end
