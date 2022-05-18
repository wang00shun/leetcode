/*
 * @lc app=leetcode.cn id=1 lang=csharp
 *
 * [1] 两数之和
 */

// @lc code=start
public partial class Solution
{
    public int[] TwoSum(int[] nums, int target)
    {
        Dictionary<int, int> dict = new Dictionary<int, int>();
        for (int currentIndex = 0; currentIndex < nums.Length; currentIndex++)
        {
            int current = nums[currentIndex];
            int search = target - current;
            if (dict.TryGetValue(search, out int searchIndex))
            {
                return new int[] { searchIndex, currentIndex };
            }
            dict.TryAdd(current, currentIndex);
        }
        return Array.Empty<int>();
    }
}
// @lc code=end

