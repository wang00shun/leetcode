/*
 * @lc app=leetcode.cn id=11 lang=csharp
 *
 * [11] 盛最多水的容器
 */

// @lc code=start
public partial class Solution
{
    public int MaxArea(int[] height)
    {
        int length = height.Length;
        if (length < 2)
        {
            return 0;
        }

        int max = 0;
        int left = 0;
        int right = length - 1;
        while (left < right)
        {
            int current = (right - left) * Math.Min(height[left], height[right]);
            if (current > max)
            {
                max = current;
            }
            if (height[left] < height[right])
            {
                left++;
            }
            else
            {
                right--;
            }
        }
        return max;
    }
}
// @lc code=end

