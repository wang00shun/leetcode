/*
 * @lc app=leetcode.cn id=56 lang=rust
 *
 * [56] 合并区间
 */

// @lc code=start
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut start = intervals[0][0];
        let mut end = intervals[0][1];
        for item in &intervals[1..] {
            if item[0] > end {
                result.push(vec![start, end]);
                start = item[0];
                end = item[1];
            } else {
                if item[1] > end {
                    end = item[1];
                }
            }
        }
        result.push(vec![start, end]);
        return result;
    }
}
// @lc code=end
