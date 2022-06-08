use std::num;

fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 0;
    let result = 1;
    println!("Hello, world! {}", result);
}

struct Solution {}

/*
 * @lc app=leetcode.cn id=54 lang=rust
 *
 * [54] 螺旋矩阵
 */

// @lc code=start
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        let row_count = matrix.len();
        let column_count = matrix[0].len();
        let times = (row_count + 1) / 2;
        for time in 0..times {
            let start_row_index = time;
            let end_row_inex = row_count - 1 - start_row_index;
            let start_column_index = time;
            let end_column_index = column_count - 1 - start_column_index;
            for column_index in start_column_index..=end_column_index {
                result.push(matrix[start_row_index][column_index]);
            }
            for row_index in (start_row_index + 1)..end_row_inex {
                result.push(matrix[end_column_index][row_index]);
            }
            for column_index in (start_column_index..=end_column_index).rev() {
                result.push(matrix[end_row_inex][column_index]);
            }
            for row_index in ((start_row_index + 1)..(end_row_inex)).rev() {
                result.push(matrix[start_column_index][row_index]);
            }
        }
        return result;
    }
}
// @lc code=end
