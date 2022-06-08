/*
 * @lc app=leetcode.cn id=48 lang=rust
 *
 * [48] 旋转图像
 */

// @lc code=start
impl Solution {
    // 从外到内,一层一层旋转
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let time = n / 2;
        for i in 0..time {
            for j in i..(n - 1 - i) {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[n - j - 1][i];
                matrix[n - j - 1][i] = matrix[n - i - 1][n - j - 1];
                matrix[n - i - 1][n - j - 1] = matrix[j][n - i - 1];
                matrix[j][n - i - 1] = tmp;
            }
        }
    }
}
// @lc code=end