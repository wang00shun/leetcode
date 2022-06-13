/*
 * @lc app=leetcode.cn id=62 lang=rust
 *
 * [62] 不同路径
 */

// @lc code=start
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 1 || n == 1 {
            return 1;
        }
        let m = m as usize;
        let n = n as usize;

        let mut vecs = Vec::new();
        for _ in 0..n {
            let mut _vec = Vec::new();
            for _ in 0..m {
                _vec.push(1);
            }
            vecs.push(_vec);
        }
        for row in 1..n {
            for column in 1..m {
                vecs[row][column] = vecs[row - 1][column] + vecs[row][column - 1];
            }
        }
        return vecs[n - 1][m - 1];
    }

    // 超时
    pub fn unique_paths0(m: i32, n: i32) -> i32 {
        if m == 1 || n == 1 {
            return 1;
        }
        return Solution::unique_paths(m - 1, n) + Solution::unique_paths(m, n - 1);
    }
}
// @lc code=end
