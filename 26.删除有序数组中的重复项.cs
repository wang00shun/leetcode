/*
 * @lc app=leetcode.cn id=26 lang=csharp
 *
 * [26] 删除有序数组中的重复项
 */

// @lc code=start
public partial class Solution
{
    public int RemoveDuplicates(int[] nums)
    {
        int removeCount = 0;
        int last = int.MaxValue;
        for (int currentIndex = 0; currentIndex < nums.Length; currentIndex++)
        {
            int num = nums[currentIndex];
            if (num == last)
            {
                removeCount++;
            }
            else
            {
                int insertIndex = currentIndex - removeCount;
                if (insertIndex != currentIndex)
                {
                    nums[insertIndex] = num;
                }
            }
            last = num;
        }
        return nums.Length - removeCount;
    }
}
// @lc code=end

