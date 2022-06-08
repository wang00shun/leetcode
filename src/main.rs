use std::num;

fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 0;
    let result = 1;
    println!("Hello, world! {}", result);
}

struct Solution {}

/*
 * @lc app=leetcode.cn id=48 lang=rust
 *
 * [48] 旋转图像
 */

// @lc code=start
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let time = n / 2;
        for i in 0..time {
            for j in 0..n - 1 {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[n - i - 1][i];
                matrix[n - i - 1][i] = matrix[n - i - 1][n - i - 1];
                matrix[n - i - 1][n - i - 1] = matrix[i][n - i - 1];
                matrix[i][n - i - 1] = tmp;
            }
        }
    }
}
// @lc code=end
