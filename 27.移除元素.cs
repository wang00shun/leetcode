/*
 * @lc app=leetcode.cn id=27 lang=csharp
 *
 * [27] 移除元素
 */

// @lc code=start
public partial class Solution
{
    public int RemoveElement(int[] nums, int val)
    {
        int length = nums.Length;
        int removeCount = 0;
        for (int currentIndex = 0; currentIndex < length; currentIndex++)
        {
            int currentNum = nums[currentIndex];
            if (currentNum == val)
            {
                removeCount++;
            }
            else
            {
                int insertIndex = currentIndex - removeCount;
                if (insertIndex != currentIndex)
                {
                    nums[insertIndex] = currentNum;
                }
            }
        }
        return length - removeCount;
    }
}
// @lc code=end

