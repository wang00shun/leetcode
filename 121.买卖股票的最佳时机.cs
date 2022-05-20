/*
 * @lc app=leetcode.cn id=121 lang=csharp
 *
 * [121] 买卖股票的最佳时机
 */

// @lc code=start
public partial class Solution
{
    public int MaxProfit(int[] prices)
    {
        int length = prices.Length;
        if (length == 0)
        {
            return 0;
        }
        int lowPrice = prices[0];
        int maxProfit = 0;
        for (int i = 1; i < length; i++)
        {
            int currentPrice = prices[i];
            if (currentPrice < lowPrice)
            {
                lowPrice = currentPrice;
            }
            else
            {
                int profit = currentPrice - lowPrice;
                if (profit > maxProfit)
                {
                    maxProfit = profit;
                }
            }
        }
        return maxProfit;
    }
}
// @lc code=end

