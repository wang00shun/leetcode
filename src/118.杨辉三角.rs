
/*
 * @lc app=leetcode.cn id=118 lang=rust
 *
 * [118] 杨辉三角
 */

// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        for row in 0..num_rows {
            let mut row_result = Vec::new();
            let mid = row / 2;
            for left_column in 0..=mid {
                if left_column == 0 {
                    row_result.push(1);
                } else {
                    let last_row_result = &result[(row - 1) as usize];
                    let v1 = last_row_result[(left_column - 1) as usize];
                    let v2 = last_row_result[(left_column) as usize];
                    row_result.push(v1 + v2);
                }
            }
            for index in (mid + 1)..=row {
                row_result.push(row_result[(row - index) as usize]);
            }
            result.push(row_result);
        }
        return result;
    }
}
// @lc code=end
