/*
 * @lc app=leetcode.cn id=118 lang=csharp
 *
 * [118] 杨辉三角
 */

// @lc code=start
public partial class Solution
{
    public IList<IList<int>> Generate(int numRows)
    {
        IList<IList<int>> result = new List<IList<int>>();
        for (int row = 0; row < numRows; row++)
        {
            int[] list = new int[row + 1];
            int mid = row / 2;
            for (int leftColumn = 0; leftColumn <= mid; leftColumn++)
            {
                int rightColumn = row - leftColumn;
                if (leftColumn == 0)
                {
                    list[leftColumn] = list[rightColumn] = 1;
                }
                else
                {
                    var lastRow = result[row - 1];
                    list[leftColumn] = list[rightColumn] = lastRow[leftColumn - 1] + lastRow[leftColumn];
                }
            }
            result.Add(list);
        }
        return result;
    }
}
// @lc code=end

