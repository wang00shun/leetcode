/*
 * @lc app=leetcode.cn id=18 lang=csharp
 *
 * [18] 四数之和
 */

// @lc code=start
public partial class Solution
{
    public IList<IList<int>> FourSum(int[] nums, int target)
    {
        int length = nums.Length;
        if (length < 4)
        {
            return new List<IList<int>>();
        }
        IList<IList<int>> result = new List<IList<int>>();
        Array.Sort(nums);
        for (int i = 0; i < length - 3; i++)
        {
            if (i != 0 && nums[i] == nums[i - 1])
            {
                continue;
            }
            int num1 = nums[i];
            int outerTarget = target - num1;
            int[] innerArray = new int[length - i - 1];
            Array.Copy(nums, i + 1, innerArray, 0, length - i - 1);
            IList<IList<int>>? innerResults = ThreeSum(innerArray, outerTarget);
            if (innerResults != null)
            {
                foreach (var innerResult in innerResults)
                {
                    innerResult.Add(num1);
                    result.Add(innerResult);
                }

            }
        }

        return result;
    }

    public IList<IList<int>> ThreeSum(int[] nums, int target)
    {
        IList<IList<int>> result = new List<IList<int>>();

        HashSet<int> num1s = new HashSet<int>();
        for (int i = 0; i < nums.Length - 2; i++)
        {
            int num1 = nums[i];
            if (num1s.Contains(num1))
            {
                continue;
            }
            int outerTarget = target - num1;
            Dictionary<int, bool> num2s = new Dictionary<int, bool>();
            for (int j = i + 1; j < nums.Length; j++)
            {
                int num2 = nums[j];
                if (num1s.Contains(num2))
                {
                    continue;
                }
                if (num2s.TryGetValue(num2, out bool success) && success)
                {
                    continue;
                }
                int num3 = outerTarget - num2;
                if (num1s.Contains(num3))
                {
                    continue;
                }
                if (num2s.ContainsKey(num3))
                {
                    result.Add(new List<int>() { num1, num2, num3 });
                    num2s[num2] = num2s[num3] = true;
                }
                else
                {
                    num2s[num2] = false;
                }
            }
            num1s.Add(num1);
        }

        return result;
    }
}
// @lc code=end

