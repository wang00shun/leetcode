/*
 * @lc app=leetcode.cn id=169 lang=csharp
 *
 * [169] 多数元素
 */

// @lc code=start
public partial class Solution
{
    public int MajorityElement(int[] nums)
    {
        int num = nums[0];
        int count = 1;
        for (int i = 1; i < nums.Length; i++)
        {
            int currentNum = nums[i];
            if (currentNum == num)
            {
                count++;
            }
            else
            {
                if (count == 0)
                {
                    count = 1;
                    num = currentNum;
                }
                else
                {
                    count--;
                }
            }
        }
        return num;
    }
}
// @lc code=end

