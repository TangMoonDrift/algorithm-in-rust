//? 宽度优先遍历专题

/**
 * 1162. 地图分析
 * https://leetcode.cn/problems/as-far-from-land-as-possible/
 */
pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
    const MAXN: usize = 101;
    const MAXM: usize = 101;
    const MOVE: [i32; 5] = [-1, 0, 1, 0, -1];

    let n = grid.len();
    let mut visited = vec![vec![false; MAXM]; MAXN];
    let mut queue = vec![[0, 0]; MAXN * MAXM];

    let (mut l, mut r, mut seas, mut level) = (0, 0, 0, 0);

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 {
                visited[i][j] = true;
                queue[r][0] = i;
                queue[r][1] = j;
                r += 1;
            } else {
                visited[i][j] = false;
                seas += 1;
            }
        }
    }

    if seas == 0 || seas == n * n {
        return -1;
    }

    while l < r {
        level += 1;
        for _ in 0..(r - l) {
            let x = queue[l][0] as i32;
            let y = queue[l][1] as i32;
            l += 1;
            for i in 0..=3 {
                let next_x = x + MOVE[i];
                let next_y = y + MOVE[i + 1];
                if next_x >= 0
                    && next_x < n as i32
                    && next_y >= 0
                    && next_y < n as i32
                    && !visited[next_x as usize][next_y as usize]
                {
                    visited[next_x as usize][next_y as usize] = true;
                    queue[r][0] = next_x as usize;
                    queue[r][1] = next_y as usize;
                    r += 1;
                }
            }
        }
    }

    level - 1
}
