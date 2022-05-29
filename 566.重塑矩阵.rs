/*
 * @lc app=leetcode.cn id=566 lang=rust
 *
 * [566] 重塑矩阵
 */

// @lc code=start
impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let r = r as usize;
        let c = c as usize;
        let o_r = mat.len();
        let o_c = mat[0].len();
        let len = r * c;
        if len != o_r * o_c {
            return mat;
        }
        let mut results: Vec<Vec<i32>> = Vec::new();
        for row_index in 0..r {
            results.push(Vec::new());
        }
        let mut new_row_index;
        let mut new_column_index;
        let mut old_row_index;
        let mut old_column_index;
        for index in 0..len {
            new_row_index = index / c;
            new_column_index = index % c;
            old_row_index = index / o_c;
            old_column_index = index % o_c;
            results[new_row_index].push(mat[old_row_index][old_column_index]);
        }
        return results;
    }
}
// @lc code=end