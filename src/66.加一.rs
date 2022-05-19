/*
 * @lc app=leetcode.cn id=66 lang=rust
 *
 * [66] 加一
 */

// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let add = true;
        let len = digits.len();
        let mut index = len - 1;
        while (add && index >= 0) {
            let num = digits[index];
            num = num + 1;
            if (num == 10) {
                add = true;
                num = 0;
            } else {
                add = false;
            }
            digits[index] = num;
            index -= 1;
        }
        if (!add) {
            return digits;
        }
        let digits = vec![];
        return digits;
    }
}
// @lc code=end
