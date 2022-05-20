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
        for (int i = length - 2; i >= 0; i--)
        {
            int numLeft = nums[i];
            int minMax = int.MaxValue;
            int minMaxIndex = i;
            for (int j = i + 1; j < length; j++)
            {
                int numRight = nums[j];
                if (numRight > numLeft && numRight < minMax)
                {
                    minMax = numRight;
                    minMaxIndex = j;
                }
            }
            if (minMaxIndex != i)
            {
                nums[i] = minMax;
                nums[minMaxIndex] = numLeft;
                Array.Sort(nums, i + 1, length - i - 1);
                return;
            }
        }
        Array.Sort(nums);
    }
}
// @lc code=end

