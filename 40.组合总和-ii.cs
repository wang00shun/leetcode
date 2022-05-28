/*
 * @lc app=leetcode.cn id=40 lang=csharp
 *
 * [40] 组合总和 II
 */

// @lc code=start
public partial class Solution
{
    public IList<IList<int>> CombinationSum2(int[] candidates, int target)
    {
        IList<IList<int>> results = new List<IList<int>>();
        List<int> store = new List<int>();
        Array.Sort(candidates);
        Dfs(candidates, target, store, results, 0);
        return results;
    }

    public void Dfs(int[] candidates, int target, List<int> store, IList<IList<int>> results, int index)
    {
        if (target == 0)
        {
            results.Add(store.ToList());
            return;
        }
        if (index == candidates.Length)
        {
            return;
        }
        if (target > 0)
        {
            Dfs(candidates, target, store, results, index + 1);
            if (store.Count == 0 && index != 0 && candidates[index] == candidates[index - 1])
            {
                return;
            }

            store.Add(candidates[index]);
            Dfs(candidates, target - candidates[index], store, results, index + 1);
            store.RemoveAt(store.Count - 1);
        }
    }
}
// @lc code=end

