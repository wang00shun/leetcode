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
        let mut total_count = row_count * column_count;
        let mut left = 0;
        let mut right = column_count - 1;
        let mut top = 0;
        let mut bottom = row_count - 1;

        let mut direction = Direction::Right;
        let mut cur_row = 0;
        let mut cur_column = 0;
        let mut count = 1;
        result.push(matrix[0][0]);
        while count < total_count {
            let mut new_row = cur_row;
            let mut new_column = cur_column;
            if direction == Direction::Right {
                if cur_column < right {
                    new_column = cur_column + 1;
                } else {
                    direction = Direction::Bottom;
                    top += 1;
                }
            } else if direction == Direction::Bottom {
                if cur_row < bottom {
                    new_row = cur_row + 1;
                } else {
                    direction = Direction::Left;
                    right -= 1;
                }
            } else if direction == Direction::Left {
                if cur_column > left {
                    new_column = cur_column - 1;
                } else {
                    direction = Direction::Top;
                    bottom -= 1;
                }
            } else {
                if cur_row > top {
                    new_row = cur_row - 1;
                } else {
                    direction = Direction::Right;
                    left += 1;
                }
            }
            if new_row != cur_row || new_column != cur_column {
                count += 1;
                cur_row = new_row;
                cur_column = new_column;
                result.push(matrix[cur_row][cur_column]);
            }
        }
        return result;
    }
}
#[derive(PartialEq)]
enum Direction {
    Right,
    Bottom,
    Left,
    Top,
}
// @lc code=end
