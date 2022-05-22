/*
 * @lc app=leetcode.cn id=219 lang=csharp
 *
 * [219] 存在重复元素 II
 */

// @lc code=start
public partial class Solution
{
    public bool ContainsNearbyDuplicate(int[] nums, int k)
    {
        for (int i = 0; i < nums.Length - 1; i++)
        {
            int num1 = nums[i];
            int rightLimit = Math.Min(nums.Length, i + k + 1);
            for (int j = i + 1; j < rightLimit; j++)
            {
                int num2 = nums[j];
                if (num1 == num2)
                {
                    return true;
                }
            }
        }
        return false;
    }
}
// @lc code=end

