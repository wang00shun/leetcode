/*
 * @lc app=leetcode.cn id=74 lang=rust
 *
 * [74] 搜索二维矩阵
 */

// @lc code=start
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let row_count = matrix.len();
        let column_count = matrix[0].len();
        let mut left = 0;
        let mut right = row_count * column_count - 1;
        while left <= right {
            let mid = (left + right) / 2;
            let column_index = mid % column_count;
            let row_index = mid / column_count;
            let num = matrix[row_index][column_index];
            if num == target {
                return true;
            }
            if num > target {
                if mid == 0 {
                    return false;
                }
                right = mid - 1;
            } else {
                if mid == usize::MAX {
                    return false;
                }
                left = mid + 1;
            }
        }
        return false;
    }
}
// @lc code=end
