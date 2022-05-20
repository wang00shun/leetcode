
/*
 * @lc app=leetcode.cn id=66 lang=rust
 *
 * [66] 加一
 */

// @lc code=start
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut add = true;
        let len = digits.len();
        let mut index = len - 1;
        while add {
            let mut num = digits[index];
            num = num + 1;
            if num == 10 {
                add = true;
                num = 0;
            } else {
                add = false;
            }
            digits[index] = num;
            if index == 0 {
                break;
            }
            index -= 1;
        }
        if !add {
            return digits;
        }
        let mut digits = vec![0i32; len + 1];
        digits[0] = 1;
        return digits;
    }
}
// @lc code=end
