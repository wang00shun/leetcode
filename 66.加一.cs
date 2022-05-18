/*
 * @lc app=leetcode.cn id=66 lang=csharp
 *
 * [66] 加一
 */

// @lc code=start
public partial class Solution
{
    public int[] PlusOne(int[] digits)
    {
        int length = digits.Length;
        bool add = true;
        for (int i = length - 1; i >= 0; i--)
        {
            int num = digits[i];
            if (add)
            {
                num += 1;
                if (num == 10)
                {
                    add = true;
                    num = 0;
                }
                else
                {
                    add = false;
                }
                digits[i] = num;
            }
            else
            {
                break;
            }
        }
        if (!add)
        {
            return digits;
        }
        int[] result = new int[length + 1];
        result[0] = 1;
        Array.Copy(digits, 0, result, 1, length);
        return result;
    }
}
// @lc code=end

