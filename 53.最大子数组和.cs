/*
 * @lc app=leetcode.cn id=53 lang=csharp
 *
 * [53] 最大子数组和
 */

// @lc code=start
public partial class Solution
{
    public int MaxSubArray(int[] nums)
    {
        int len = nums.Length;
        if (len == 0)
        {
            return 0;
        }
        int max = int.MinValue;
        int sum = int.MinValue;
        for (int i = 0; i < len; i++)
        {
            int num = nums[i];
            if (sum < 0)
            {
                sum = num;
            }
            else
            {
                sum = sum + num;
            }
            if (sum > max)
            {
                max = sum;
            }
        }
        return max;
    }
}
// @lc code=end

