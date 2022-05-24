/*
 * @lc app=leetcode.cn id=448 lang=csharp
 *
 * [448] 找到所有数组中消失的数字
 */

// @lc code=start
public partial class Solution
{
    public IList<int> FindDisappearedNumbers(int[] nums)
    {
        int leftIndex = 0;
        while (leftIndex < nums.Length)
        {
            int num = nums[leftIndex];
            if (nums[leftIndex] == nums[num - 1])
            {
                leftIndex++;
                continue;
            }
            nums[leftIndex] = nums[num - 1];
            nums[num - 1] = num;
        }
        List<int> result = new List<int>();
        for (int i = 0; i < nums.Length; i++)
        {
            if (nums[i] != i + 1)
            {
                result.Add(i + 1);
            }
        }
        return result;
    }
}
// @lc code=end

