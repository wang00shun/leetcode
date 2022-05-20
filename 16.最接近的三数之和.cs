/*
 * @lc app=leetcode.cn id=16 lang=csharp
 *
 * [16] 最接近的三数之和
 */

// @lc code=start
public partial class Solution
{
    public int ThreeSumClosest(int[] nums, int target)
    {
        int length = nums.Length;
        if (length < 3)
        {
            return 0;
        }
        Array.Sort(nums);
        int minDifference = int.MaxValue;
        for (int i = 0; i < length - 2; i++)
        {
            int num1 = nums[i];
            int outerTarget = target - num1;
            int left = i + 1;
            int right = length - 1;
            while (left < right)
            {
                int num2 = nums[left];
                int num3 = nums[right];
                int difference = outerTarget - num2 - num3;
                if (difference == 0)
                {
                    return target;
                }
                if (difference > 0)
                {
                    do
                    {
                        left++;
                    } while (left < right && nums[left] == nums[left - 1]);
                }
                if (difference < 0)
                {
                    do
                    {
                        right--;
                    } while (left < right && nums[right] == nums[right + 1]);
                }
                if (Math.Abs(minDifference) > Math.Abs(difference))
                {
                    minDifference = difference;
                }
            }
        }

        return target - minDifference;
    }
}
// @lc code=end

