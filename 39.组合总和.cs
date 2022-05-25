/*
 * @lc app=leetcode.cn id=39 lang=csharp
 *
 * [39] 组合总和
 */

// @lc code=start
public partial class Solution
{
    public partial IList<IList<int>> CombinationSum(int[] candidates, int target)
    {
        Array.Sort(candidates);
        IList<IList<int>> result = new List<IList<int>>();
        int len = candidates.Length;
        int left = 0;
        Stack<int> stack = new Stack<int>();
        int sum = 0;
        while (left < len)
        {

            while (sum < target)
            {
                sum += candidates[left];
            }

        }

        return result;
    }
}
// @lc code=end

