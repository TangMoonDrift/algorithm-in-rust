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

/**
 * 130. 被围绕的区域
 * https://leetcode.cn/problems/surrounded-regions/description/
 */
pub fn solve(board: &mut Vec<Vec<char>>) {
    let n = board.len();
    if n == 0 {
        return;
    }
    let m = board[0].len();

    for i in 0..m {
        if board[0][i] == 'O' {
            dfs(board, 0 as i32, i as i32, n, m);
        }
        if board[n - 1][i] == 'O' {
            dfs(board, (n - 1) as i32, i as i32, n, m);
        }
    }
    for j in 1..(n - 1) {
        if board[j][0] == 'O' {
            dfs(board, j as i32, 0 as i32, n, m);
        }
        if board[j][m - 1] == 'O' {
            dfs(board, j as i32, (m - 1) as i32, n, m);
        }
    }

    for i in 0..n {
        for j in 0..m {
            if board[i][j] == 'O' {
                board[i][j] = 'X'; // 修改为赋值操作
            }
            if board[i][j] == 'F' {
                board[i][j] = 'O'; // 修改为赋值操作
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, i: i32, j: i32, n: usize, m: usize) {
        if i < 0
            || (i as usize) == n
            || j < 0
            || (j as usize) == m
            || board[i as usize][j as usize] != 'O'
        {
            return;
        }
        board[i as usize][j as usize] = 'F';
        dfs(board, i - 1, j, n, m);
        dfs(board, i + 1, j, n, m);
        dfs(board, i, j - 1, n, m);
        dfs(board, i, j + 1, n, m);
    }
}
