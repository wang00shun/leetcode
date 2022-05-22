/*
 * @lc app=leetcode.cn id=228 lang=csharp
 *
 * [228] 汇总区间
 */

// @lc code=start
public partial class Solution
{
    public IList<string> SummaryRanges(int[] nums)
    {
        IList<string> result = new List<string>();
        int startNum = 0;
        int count = 0;
        for (int i = 0; i < nums.Length; i++)
        {
            int currentNum = nums[i];
            if (count == 0 || (startNum + count != currentNum))
            {
                if (count != 0)
                {
                    result.Add(GetString(startNum, count));
                }
                startNum = currentNum;
                count = 1;
            }
            else
            {
                count++;
            }
        }
        if (count != 0)
        {
            result.Add(GetString(startNum, count));
        }
        return result;
    }

    private string GetString(int startNum, int count)
    {
        if (count == 1)
        {
            return startNum.ToString();
        }
        else
        {
            return $"{startNum}->{startNum + count - 1}";
        }
    }
}
// @lc code=end

