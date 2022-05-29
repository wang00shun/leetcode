/*
 * @lc app=leetcode.cn id=605 lang=csharp
 *
 * [605] 种花问题
 */

// @lc code=start
public partial class Solution
{
    public bool CanPlaceFlowers(int[] flowerbed, int n)
    {
        int result = 0;
        int lastFlowerIndex = -2;
        for (int currentIndex = 0; currentIndex < flowerbed.Length; currentIndex++)
        {
            int current = flowerbed[currentIndex];
            if (current == 0)
            {
                if (currentIndex - lastFlowerIndex >= 2)
                {
                    if (currentIndex + 1 >= flowerbed.Length || flowerbed[currentIndex + 1] == 0)
                    {
                        result += 1;
                        lastFlowerIndex = currentIndex;
                    }
                }
            }
            else
            {
                lastFlowerIndex = currentIndex;
            }
        }
        return result >= n;
    }
}
// @lc code=end

