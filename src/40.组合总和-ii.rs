
/*
 * @lc app=leetcode.cn id=40 lang=rust
 *
 * [40] 组合总和 II
 */

// @lc code=start
impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let mut store = Vec::new();
        candidates.sort();
        dfs(&candidates, &mut results, &mut store, 0, target);
        return results;
    }
}

pub fn dfs(
    candidates: &Vec<i32>,
    mut results: &mut Vec<Vec<i32>>,
    mut store: &mut Vec<i32>,
    mut index: usize,
    target: i32,
) {
    if target == 0 && store.len() != 0 {
        results.push(store.to_owned())
    }
    if index >= candidates.len() {
        return;
    }
    if target > 0 {
        store.push(candidates[index]);
        dfs(
            &candidates,
            &mut results,
            &mut store,
            index + 1,
            target - candidates[index],
        );
        store.remove(store.len() - 1);

        loop {
            index += 1;
            if index >= candidates.len() || candidates[index] != candidates[index - 1] {
                break;
            }
        }
        dfs(&candidates, &mut results, &mut store, index, target);
    }
}
// @lc code=end