/*
 * @lc app=leetcode.cn id=64 lang=rust
 *
 * [64] 最小路径和
 */

// @lc code=start
impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let row_count = grid.len();
        let column_count = grid[0].len();
        for row_index in 0..row_count {
            for column_index in 0..column_count {
                match (row_index, column_index) {
                    (0, 0) => {}
                    (0, _) => {
                        grid[0][column_index] += grid[0][column_index - 1];
                    }
                    (_, 0) => {
                        grid[row_index][column_index] += grid[row_index - 1][0];
                    }
                    _ => {
                        grid[row_index][column_index] += std::cmp::min(
                            grid[row_index - 1][column_index],
                            grid[row_index][column_index - 1],
                        );
                    }
                }
            }
        }
        return grid[row_count - 1][column_count - 1];
    }

    // 递归超时
    pub fn min(grid: &Vec<Vec<i32>>, row_index: usize, column_index: usize) -> i32 {
        if row_index == 0 && column_index == 0 {
            return grid[0][0];
        }
        if row_index == 0 {
            return grid[row_index][column_index] + Solution::min(grid, 0, column_index - 1);
        }
        if column_index == 0 {
            return grid[row_index][column_index]
                + Solution::min(grid, row_index - 1, column_index);
        }
        return grid[row_index][column_index]
            + std::cmp::min(
                Solution::min(grid, row_index - 1, column_index),
                Solution::min(grid, row_index, column_index - 1),
            );
    }
}
// @lc code=end
