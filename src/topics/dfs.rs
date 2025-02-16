//! 深度优先遍历专题

/**
 * 200. 岛屿数量
 * https://leetcode.cn/problems/number-of-islands/description/
 */
pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut ans = 0;

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '1' {
                ans += 1;
                dfs(&mut grid, i as i32, j as i32);
            }
        }
    }

    fn dfs(grid: &mut Vec<Vec<char>>, i: i32, j: i32) {
        if i < 0
            || i as usize == grid.len()
            || j < 0
            || j as usize == grid[0].len()
            || grid[i as usize][j as usize] == '0'
        {
            return;
        }
        grid[i as usize][j as usize] = '0';
        dfs(grid, i - 1, j);
        dfs(grid, i + 1, j);
        dfs(grid, i, j - 1);
        dfs(grid, i, j + 1);
    }

    ans
}
