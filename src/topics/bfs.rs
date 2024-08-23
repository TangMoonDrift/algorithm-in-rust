//? 宽度优先遍历专题

use std::collections::VecDeque;

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

/**
 * 1368. 使网格图至少有一条有效路径的最小代价
 * https://leetcode.cn/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid/
 */
pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
    const MOVE: [[i32; 2]; 5] = [[0, 0], [0, 1], [0, -1], [1, 0], [-1, 0]];
    let n = grid.len();
    let m = grid[0].len();
    let mut distance = vec![vec![i32::MAX; m]; n];

    let mut deque = VecDeque::<[i32; 2]>::new();
    distance[0][0] = 0;
    deque.push_front([0, 0]);

    while !deque.is_empty() {
        let record = deque.pop_front().unwrap();
        let x = record[0];
        let y = record[1];

        if x == (n - 1) as i32 && y == (m - 1) as i32 {
            return distance[x as usize][y as usize];
        }

        for i in 1..=4 {
            let next_x = x + MOVE[i][0];
            let next_y = y + MOVE[i][1];
            let weight = if grid[x as usize][y as usize] == i as i32 {
                0
            } else {
                1
            };
            if next_x >= 0
                && next_x < n as i32
                && next_y >= 0
                && next_y < m as i32
                && distance[x as usize][y as usize] + weight
                    < distance[next_x as usize][next_y as usize]
            {
                distance[next_x as usize][next_y as usize] =
                    distance[x as usize][y as usize] + weight;
                if weight == 0 {
                    deque.push_front([next_x, next_y]);
                } else {
                    deque.push_back([next_x, next_y]);
                }
            }
        }
    }

    -1
}
