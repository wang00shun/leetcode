/*
 * @lc app=leetcode.cn id=88 lang=csharp
 *
 * [88] 合并两个有序数组
 */

// @lc code=start
public partial class Solution
{
    public void Merge(int[] nums1, int m, int[] nums2, int n)
    {
        var index = m + n - 1;
        var mIndex = m - 1;
        var nIndex = n - 1;
        while (mIndex >= 0 && nIndex >= 0)
        {
            var mNum = nums1[mIndex];
            var nNum = nums2[nIndex];
            if (mNum > nNum)
            {
                nums1[index] = mNum;
                mIndex--;
            }
            else
            {
                nums1[index] = nNum;
                nIndex--;
            }
            index--;
        }
        if (nIndex >= 0)
        {
            for (int i = 0; i <= nIndex; i++)
            {
                nums1[i] = nums2[i];
            }
        }

    }
}
// @lc code=end

