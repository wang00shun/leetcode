/*
 * @lc app=leetcode.cn id=39 lang=csharp
 *
 * [39] 组合总和
 */

// @lc code=start
public partial class Solution
{
    public IList<IList<int>> CombinationSum(int[] candidates, int target)
    {
        Array.Sort(candidates);
        Array.Reverse(candidates);
        IList<IList<int>> results = new List<IList<int>>();
        int len = candidates.Length;
        // store index
        Stack<int> stack = new Stack<int>();
        int sum = 0;
        int currentIndex = 0;
        while (currentIndex < len)
        {
            if (sum < target)
            {
                stack.Push(currentIndex);
                sum += candidates[currentIndex];
            }
            else if (sum >= target)
            {
                if (sum == target)
                {
                    results.Add(stack.ToArray().Select(i => candidates[i]).ToArray());
                }
                if (stack.Count > 0)
                {
                    while (stack.Count > 0)
                    {
                        int topIndex = stack.Pop();
                        sum -= candidates[topIndex];
                        currentIndex = topIndex + 1;
                        if (currentIndex < len)
                        {
                            break;
                        }
                    }
                }
                else
                {
                    currentIndex++;
                }
            }
        }
        return results;
    }
}
// @lc code=end

