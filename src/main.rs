use std::num;

fn main() {}

struct Solution {}
/*
 * @lc app=leetcode.cn id=78 lang=rust
 *
 * [78] 子集
 */

// @lc code=start
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        return Solution::subsets2(nums);
    }
    pub fn subsets2(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        for num in nums {
            let result_len = result.len();
            for result_index in 0..result_len {
                let mut result_item = result[result_index].to_owned();
                result_item.push(num);
                result.push(result_item);
            }
            result.push(vec![num]);
        }
        result.push(vec![]);
        return result;
    }

    pub fn subsets1(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        Solution::s(&nums, 0, &mut result);
        result.push(Vec::new());
        return result;
    }

    pub fn s(nums: &Vec<i32>, index: usize, result: &mut Vec<Vec<i32>>) {
        if index >= nums.len() {
            return;
        }
        Solution::s(nums, index + 1, result);
        let len = result.len();
        for result_index in 0..len {
            let mut _vec = result[result_index].to_owned();
            _vec.push(nums[index]);
            result.push(_vec);
        }
        result.push(vec![nums[index]]);
    }
}
// @lc code=end
