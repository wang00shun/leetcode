/*
 * @lc app=leetcode.cn id=57 lang=rust
 *
 * [57] 插入区间
 */

// @lc code=start
impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let len = intervals.len();
        for index in (0..=len - 1).rev() {
            let cur = intervals[index];
        }
        return intervals;
    }
}
// @lc code=end