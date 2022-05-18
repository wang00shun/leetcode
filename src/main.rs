fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 0;
    let result = Solution::search_insert(nums, target);
    println!("Hello, world! {}", result);
}

struct Solution {}

/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */
// @lc code=start
use std::{collections::HashMap, vec};
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashMap = HashMap::<i32, i32>::new();
        for (current_index, &current_numm) in nums.iter().enumerate() {
            let current_index = current_index as i32;
            let search_num = target - current_numm;
            if let Some(&search_index) = hashMap.get(&search_num) {
                return vec![search_index, current_index];
            } else {
                hashMap.entry(current_numm).or_insert(current_index);
            }
        }
        return vec![];
    }
}
// @lc code=end

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

/*
 * @lc app=leetcode.cn id=35 lang=rust
 *
 * [35] 搜索插入位置
 */

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        let mut left: i32 = 0;
        let mut right: i32 = (len - 1) as i32;
        while left <= right {
            let mid = (left + right) / 2;
            let num = nums[mid as usize];
            if num == target {
                return mid;
            }
            if num < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        return left as i32;
    }
}
// @lc code=end
