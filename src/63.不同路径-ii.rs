/*
 * @lc app=leetcode.cn id=63 lang=rust
 *
 * [63] 不同路径 II
 */

// @lc code=start
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let row_count = obstacle_grid.len();
        let column_count = obstacle_grid[0].len();
        let mut vecs: Vec<Vec<i32>> = Vec::new();
        for row_index in 0..row_count {
            let mut _vec = Vec::new();
            for column_index in 0..column_count {
                if obstacle_grid[row_index][column_index] == 1 {
                    _vec.push(0);
                } else {
                    match (row_index, column_index) {
                        (0, 0) => {
                            _vec.push(1);
                        }
                        (0, _) => {
                            _vec.push(_vec[column_index - 1]);
                        }
                        (_, 0) => {
                            _vec.push(vecs[row_index - 1][0]);
                        }
                        _ => {
                            _vec.push(_vec[column_index - 1] + vecs[row_index - 1][column_index]);
                        }
                    }
                }
            }
            vecs.push(_vec)
        }
        return vecs[row_count - 1][column_count - 1];
    }
}
// @lc code=end