/*
 * @lc app=leetcode.cn id=33 lang=csharp
 *
 * [33] 搜索旋转排序数组
 */

// @lc code=start
public partial class Solution
{
    public int Search(int[] nums, int target)
    {
        int len = nums.Length;
        bool front = false;
        int firstNum = nums[0];
        int lastNum = nums[len - 1];
        if (nums[len - 1] == target)
        {
            return len - 1;
        }
        if (nums[0] == target)
        {
            return 0;
        }
        int left = 0;
        int right = len - 1;
        while (left <= right)
        {
            int mid = (left + right) / 2;
            int midNum = nums[mid];
            if (midNum == target)
            {
                return mid;
            }
            if (target > midNum)
            {
                if (target > lastNum)
                {
                    right = mid - 1;
                }
                else
                {
                    left = mid + 1;
                }
            }
            else
            {
                if (target < firstNum)
                {
                    left = mid + 1;
                }
                else
                {
                    right = mid - 1;
                }
            }
        }
        return -1;
    }
}
// @lc code=end

