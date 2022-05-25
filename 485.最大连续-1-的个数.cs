/*
 * @lc app=leetcode.cn id=485 lang=csharp
 *
 * [485] 最大连续 1 的个数
 */

// @lc code=start
public partial class Solution
{
    public int FindMaxConsecutiveOnes(int[] nums)
    {
        int max = 0;
        int count = 0;
        for (int i = 0; i < nums.Length; i++)
        {
            if (nums[i] == 0)
            {
                count = 0;
            }
            else
            {
                count++;
                if (count > max)
                {
                    max = count;
                }
            }
        }
        return max;
    }
}
// @lc code=end

