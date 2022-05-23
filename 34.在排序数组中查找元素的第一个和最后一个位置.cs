/*
 * @lc app=leetcode.cn id=34 lang=csharp
 *
 * [34] 在排序数组中查找元素的第一个和最后一个位置
 */

// @lc code=start
public partial class Solution
{
    public int[] SearchRange(int[] nums, int target)
    {
        if (nums.Length == 0)
        {
            return new int[] { -1, -1 };
        }
        return SearchNum(nums, target, 0, nums.Length - 1);
    }

    public int[] SearchNum(int[] nums, int target, int start, int end)
    {
        if (start > end)
        {
            return new int[] { -1, -1 };
        }
        int mid = (start + end) / 2;
        int midNum = nums[mid];
        if (midNum == target)
        {
            int[] result = new int[] { mid, mid };
            var leftResult = SearchNum(nums, target, start, mid - 1);
            if (leftResult[0] != -1)
            {
                result[0] = leftResult[0];
            }
            var rightResult = SearchNum(nums, target, mid + 1, end);
            if (rightResult[1] != -1)
            {
                result[1] = rightResult[1];
            }
            return result;
        }
        else if (midNum < target)
        {
            return SearchNum(nums, target, mid + 1, end);
        }
        return SearchNum(nums, target, start, mid - 1);

    }
}
// @lc code=end

