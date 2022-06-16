/*
 * @lc app=leetcode.cn id=73 lang=rust
 *
 * [73] 矩阵置零
 */

// @lc code=start
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        return Solution::set_zeroes3(matrix);
    }

    // O(1) 的额外空间
    pub fn set_zeroes3(matrix: &mut Vec<Vec<i32>>) {
        let row_count = matrix.len();
        let column_count = matrix[0].len();
        let mut row0flag = false;
        let mut column0flag = false;
        for row_index in 0..row_count {
            if matrix[row_index][0] == 0 {
                column0flag = true;
                break;
            }
        }
        for column_index in 0..column_count {
            if matrix[0][column_index] == 0 {
                row0flag = true;
                break;
            }
        }
        for row_index in 1..row_count {
            for column_index in 1..column_count {
                if matrix[row_index][column_index] == 0 {
                    matrix[row_index][0] = 0;
                    matrix[0][column_index] = 0;
                }
            }
        }

        for row_index in 1..row_count {
            for column_index in 1..column_count {
                if matrix[row_index][0] == 0 || matrix[0][column_index] == 0 {
                    matrix[row_index][column_index] = 0;
                }
            }
        }

        if row0flag {
            for column_index in 0..column_count {
                matrix[0][column_index] = 0;
            }
        }
        if column0flag {
            for row_index in 0..row_count {
                matrix[row_index][0] = 0;
            }
        }
    }

    // O(m+n) 的额外空间
    pub fn set_zeroes2(matrix: &mut Vec<Vec<i32>>) {
        let row_count = matrix.len();
        let column_count = matrix[0].len();
        let mut column_cache = Vec::new();
        let mut row_cache = Vec::new();
        for _ in 0..row_count {
            row_cache.push(1);
        }
        for _ in 0..column_count {
            column_cache.push(1);
        }
        for row_index in 0..row_count {
            for column_index in 0..column_count {
                if matrix[row_index][column_index] == 0 {
                    row_cache[row_index] = 0;
                    column_cache[column_index] = 0;
                }
            }
        }
        for row_index in 0..row_count {
            for column_index in 0..column_count {
                if row_cache[row_index] == 0 || column_cache[column_index] == 0 {
                    matrix[row_index][column_index] = 0;
                }
            }
        }
    }

    // O(mn) 的额外空间
    pub fn set_zeroes1(matrix: &mut Vec<Vec<i32>>) {
        let row_count = matrix.len();
        let column_count = matrix[0].len();
        let mut cache = Vec::new();
        for _ in 0..row_count {
            let mut _vec = Vec::new();
            for _ in 0..column_count {
                _vec.push(1);
            }
            cache.push(_vec);
        }
        for row_index in 0..row_count {
            for column_index in 0..column_count {
                if matrix[row_index][column_index] == 0 {
                    for i in 0..column_count {
                        cache[row_index][i] = 0;
                    }
                    for i in 0..row_count {
                        cache[i][column_index] = 0;
                    }
                }
            }
        }
        for row_index in 0..row_count {
            for column_index in 0..column_count {
                if cache[row_index][column_index] == 0 {
                    matrix[row_index][column_index] = 0;
                }
            }
        }
    }
}
// @lc code=end
