/*
 * @lc app=leetcode.cn id=283 lang=csharp
 *
 * [283] 移动零
 */

// @lc code=start
public partial class Solution
{
    public void MoveZeroes(int[] nums)
    {
        int count = 0;
        int len = nums.Length;
        for (int i = 0; i < len; i++)
        {
            int num = nums[i];
            if (num == 0)
            {
                count++;
            }
            else
            {
                nums[i - count] = num;
            }
        }
        for (int i = len - 1; i > len - 1 - count; i--)
        {
            nums[i] = 0;
        }
    }
}
// @lc code=end

