/*
 * @lc app=leetcode.cn id=217 lang=csharp
 *
 * [217] 存在重复元素
 */

// @lc code=start
public partial class Solution
{
    public bool ContainsDuplicate(int[] nums)
    {
        HashSet<int> visited = new HashSet<int>();
        for (int i = 0; i < nums.Length; i++)
        {
            if (visited.Contains(nums[i]))
            {
                return true;
            }
            visited.Add(nums[i]);
        }
        return false;
    }
}
// @lc code=end

