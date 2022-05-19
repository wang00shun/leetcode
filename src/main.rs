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

/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子数组和
 */

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;
        let mut sum = i32::MIN;
        for num in nums {
            if sum < 0 {
                sum = num;
            } else {
                sum += num;
            }
            if sum > max {
                max = sum;
            }
        }
        return max;
    }
}
// @lc code=end

/*
 * @lc app=leetcode.cn id=66 lang=rust
 *
 * [66] 加一
 */

// @lc code=start
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut add = true;
        let len = digits.len();
        let mut index = len - 1;
        while add {
            let mut num = digits[index];
            num = num + 1;
            if num == 10 {
                add = true;
                num = 0;
            } else {
                add = false;
            }
            digits[index] = num;
            if index == 0 {
                break;
            }
            index -= 1;
        }
        if !add {
            return digits;
        }
        let mut digits = vec![0i32; len + 1];
        digits[0] = 1;
        return digits;
    }
}
// @lc code=end

/*
 * @lc app=leetcode.cn id=88 lang=rust
 *
 * [88] 合并两个有序数组
 */

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut index = m + n - 1;
        let mut m_index = m - 1;
        let mut n_index = n - 1;
        while m_index >= 0 && n_index >= 0 {
            let m_num = nums1[m_index as usize];
            let n_num = nums2[n_index as usize];
            if m_num > n_num {
                nums1[index as usize] = m_num;
                m_index -= 1;
            } else {
                nums1[index as usize] = n_num;
                n_index -= 1;
            }
            index -= 1;
        }
        while n_index >= 0 {
            nums1[n_index as usize] = nums2[n_index as usize];
            n_index -= 1;
        }
    }
}
// @lc code=end
