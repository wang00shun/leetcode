/*
 * @lc app=leetcode.cn id=39 lang=rust
 *
 * [39] 组合总和
 */

// @lc code=start
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut store: Vec<i32> = Vec::new();
        dfs(&mut result, &candidates, &mut store, 0, target);
        return result;
    }
}

pub fn dfs(
    mut result: &mut Vec<Vec<i32>>,
    candidates: &Vec<i32>,
    mut store: &mut Vec<i32>,
    index: usize,
    target: i32,
) {
    if index == candidates.len() {
        return;
    }

    if target == 0 {
        result.push(store.to_owned());
        return;
    }

    if target > 0 {
        store.push(candidates[index]);
        dfs(
            &mut result,
            &candidates,
            &mut store,
            index,
            target - candidates[index],
        );
        store.remove(store.len() - 1);
        
        dfs(&mut result, &candidates, &mut store, index + 1, target);
    }
}
// @lc code=end