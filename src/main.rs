use std::num;

fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 0;
    let result = 1;
    println!("Hello, world! {}", result);
}

struct Solution {}

/*
 * @lc app=leetcode.cn id=45 lang=rust
 *
 * [45] 跳跃游戏 II
 */

// @lc code=start
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len <= 1 {
            return 0;
        }
        let mut count = 0;
        let mut index = 0;
        while index < len {
            count += 1;
            let cur = nums[index] as usize;
            if cur + index >= len - 1 {
                break;
            }
            let mut max_Index = 0;
            let mut max = 0;
            for add in 1..=cur {
                let next_index = index + add;
                let next_num = nums[next_index] as usize;
                let next_max = next_num + next_index;
                if next_max >= max {
                    max = next_max;
                    max_Index = next_index;
                }
            }
            index = max_Index;
        }
        return count;
    }
}
// @lc code=end
