/*
 * @lc app=leetcode.cn id=268 lang=csharp
 *
 * [268] 丢失的数字
 */

// @lc code=start
public partial class Solution
{
    public int MissingNumber(int[] nums)
    {
        // 0 1 3 , len 3 , 2
        int sum = (nums.Length + 1) * nums.Length / 2;
        for (int i = 0; i < nums.Length; i++)
        {
            sum -= nums[i];
        }
        return sum;
    }
}
// @lc code=end

