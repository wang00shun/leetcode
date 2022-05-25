/*
 * @lc app=leetcode.cn id=495 lang=csharp
 *
 * [495] 提莫攻击
 */

// @lc code=start
public partial class Solution
{
    public int FindPoisonedDuration(int[] timeSeries, int duration)
    {
        int len = timeSeries.Length;
        if (len == 0)
        {
            return 0;
        }
        int result = duration;
        for (int i = timeSeries.Length - 2; i >= 0; i--)
        {
            int diff = timeSeries[i + 1] - timeSeries[i];
            result += diff > duration ? duration : diff;
        }
        return result;
    }
}
// @lc code=end

