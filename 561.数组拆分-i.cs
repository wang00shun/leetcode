/*
 * @lc app=leetcode.cn id=561 lang=csharp
 *
 * [561] 数组拆分 I
 */

// @lc code=start
public partial class Solution
{
    public int ArrayPairSum(int[] nums)
    {
        Array.Sort(nums);
        int sum = 0;
        int count = nums.Length / 2;
        for (int i = 0; i < count; i++)
        {
            sum += nums[i * 2];
        }
        return sum;
    }
}
// @lc code=end

