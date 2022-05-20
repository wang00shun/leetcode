
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
