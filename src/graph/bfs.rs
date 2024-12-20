//! 宽度优先遍历专题
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

#[derive(Eq, PartialEq, Debug)]
pub struct Node {
    row: usize,
    col: usize,
    val: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.val.cmp(&other.val) {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => Ordering::Equal,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Into<Node> for (usize, usize, i32) {
    fn into(self) -> Node {
        Node {
            row: self.0,
            col: self.1,
            val: self.2,
        }
    }
}

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

pub fn trap_rain_water(mut height_map: Vec<Vec<i32>>) -> i32 {
    if height_map.len() <= 2 || height_map[0].len() <= 2 {
        return 0;
    }
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    let row = height_map.len();
    let col = height_map[0].len();

    for r in 0..row {
        heap.push((r, 0, height_map[r][0]).into());
        heap.push((r, col - 1, height_map[r][col - 1]).into());
        height_map[r][0] = -1;
        height_map[r][col - 1] = -1;
    }
    for c in 1..col - 1 {
        heap.push((row - 1, c, height_map[row - 1][c]).into());
        heap.push((0, c, height_map[0][c]).into());
        height_map[row - 1][c] = -1;
        height_map[0][c] = -1;
    }

    let mut sum: i32 = 0;
    let pos: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    while !heap.is_empty() {
        let node = heap.pop().unwrap();
        for i in 0..4 {
            let temp_r = (node.row as i32 + pos[i].0) as usize;
            let temp_c = (node.col as i32 + pos[i].1) as usize;
            if temp_r < row && temp_c < col && height_map[temp_r][temp_c] >= 0 {
                if height_map[temp_r][temp_c] < node.val {
                    sum = sum + node.val - height_map[temp_r][temp_c];
                    heap.push((temp_r, temp_c, node.val).into());
                } else {
                    heap.push((temp_r, temp_c, height_map[temp_r][temp_c]).into());
                }
                height_map[temp_r][temp_c] = -1;
            }
        }
    }
    sum
}
