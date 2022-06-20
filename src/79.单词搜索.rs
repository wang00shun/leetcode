/*
 * @lc app=leetcode.cn id=79 lang=rust
 *
 * [79] 单词搜索
 */

// @lc code=start
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let chars: Vec<char> = word.chars().collect();
        if chars.len() == 0 {
            return true;
        }

        let row_count = board.len();
        let column_count = board[0].len();
        let total = row_count * column_count;
        if total < chars.len() {
            return false;
        }

        let mut cache = Vec::new();
        for _ in 0..row_count {
            let mut cache_vec = Vec::new();
            for _ in 0..column_count {
                cache_vec.push(false);
            }
            cache.push(cache_vec);
        }

        for row_index in 0..row_count {
            for column_index in 0..column_count {
                if board[row_index][column_index] == chars[0] {
                    let flag =
                        Solution::next(&chars, 0, &board, &mut cache, row_index, column_index);
                    if flag {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    pub fn next(
        chars: &Vec<char>,
        mut char_index: usize,
        board: &Vec<Vec<char>>,
        cache: &mut Vec<Vec<bool>>,
        cur_row: usize,
        cur_column: usize,
    ) -> bool {
        if chars[char_index] != board[cur_row][cur_column] {
            return false;
        }

        char_index += 1;
        if char_index >= chars.len() {
            return true;
        }
        cache[cur_row][cur_column] = true;

        // top
        if cur_row >= 1 && !cache[cur_row - 1][cur_column] {
            let flag = Solution::next(&chars, char_index, &board, cache, cur_row - 1, cur_column);
            if flag {
                return true;
            }
        }

        // bottom
        if cur_row < board.len() - 1 && !cache[cur_row + 1][cur_column] {
            let flag = Solution::next(&chars, char_index, &board, cache, cur_row + 1, cur_column);
            if flag {
                return true;
            }
        }

        // left
        if cur_column >= 1 && !cache[cur_row][cur_column - 1] {
            let flag = Solution::next(&chars, char_index, &board, cache, cur_row, cur_column - 1);
            if flag {
                return true;
            }
        }

        // right
        if cur_column < board[0].len() - 1 && !cache[cur_row][cur_column + 1] {
            let flag = Solution::next(&chars, char_index, &board, cache, cur_row, cur_column + 1);
            if flag {
                return true;
            }
        }

        cache[cur_row][cur_column] = false;
        return false;
    }
}

// @lc code=end
