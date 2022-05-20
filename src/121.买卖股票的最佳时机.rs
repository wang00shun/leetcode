/*
 * @lc app=leetcode.cn id=121 lang=rust
 *
 * [121] 买卖股票的最佳时机
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        if len == 0 {
            return 0;
        }
        let mut low_price = prices[0];
        let mut max_profit = 0;
        for i in 1..len {
            let current_price = prices[i];
            if current_price < low_price {
                low_price = current_price;
            } else {
                let profit = current_price - low_price;
                if profit > max_profit {
                    max_profit = profit;
                }
            }
        }
        return max_profit;
    }
}
