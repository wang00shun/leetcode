/*
 * @lc app=leetcode.cn id=31 lang=csharp
 *
 * [31] 下一个排列
 */

// @lc code=start
public partial class Solution
{
    public void NextPermutation(int[] nums)
    {
        int length = nums.Length;
        for (int i = length - 1; i >= 0; i--)
        {
            int numRight = nums[i];
            for (int j = i - 1; j >= 0; j--)
            {
                int numLeft = nums[j];
                if (numLeft < numRight)
                {
                    nums[j] = numRight;
                    nums[i] = numLeft;
                    Array.Sort(nums, j + 1, length - j - 1);
                    return;
                }
            }
        }
        Array.Sort(nums);
    }
}
// @lc code=end

