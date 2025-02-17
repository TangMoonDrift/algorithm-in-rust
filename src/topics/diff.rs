//! 差分专题

/**
 * https://leetcode.cn/problems/corporate-flight-bookings/
 */
pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
    let flights_cnt = n as usize;
    let mut ans = vec![0; flights_cnt + 2];

    for booking in bookings {
        ans[booking[0] as usize] += booking[2];
        ans[booking[1] as usize + 1] -= booking[2];
    }

    for i in 1..(flights_cnt + 2) {
        ans[i] += ans[i - 1];
    }

    ans[1..=flights_cnt].to_vec()
}

/**
 * https://leetcode.cn/problems/range-sum-query-2d-immutable/
 */
struct NumMatrix {
    sum: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut sum = vec![vec![0; n + 1]; m + 1];

        for (i, row) in matrix.iter().enumerate() {
            for (j, x) in row.iter().enumerate() {
                sum[i + 1][j + 1] = sum[i + 1][j] + sum[i][j + 1] - sum[i][j] + x;
            }
        }

        Self { sum }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let r1 = row1 as usize;
        let c1 = col1 as usize;
        let r2 = row2 as usize + 1;
        let c2 = col2 as usize + 1;
        self.sum[r2][c2] - self.sum[r2][c1] - self.sum[r1][c2] + self.sum[r1][c1]
    }
}

/**
 * 1139. 最大的以 1 为边界的正方形
 * https://leetcode.cn/problems/largest-1-bordered-square/
 */
pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len() as i32;
    let m = grid[0].len() as i32;
    let tries = grid.clone();
    let tries = bulid(n as usize, m as usize, tries);

    if sum(&tries, 0, 0, n - 1, m - 1) == 0 {
        return 0;
    }

    let mut ans: i32 = 1;

    for a in 0..n {
        for b in 0..m {
            // (a,b)所有左上角点
            //     (c,d)更大边长的右下角点，k是当前尝试的边长
            let (mut c, mut d, mut k) = (a + ans, b + ans, ans + 1);
            while c < n && d < m {
                let outer_area = sum(&tries, a, b, c, d);
                let inner_area = sum(&tries, a + 1, b + 1, c - 1, d - 1);
                let perimeter = (k - 1) << 2;
                if outer_area - inner_area == perimeter {
                    ans = k;
                }
                c += 1;
                d += 1;
                k += 1;
            }
        }
    }

    fn bulid(n: usize, m: usize, mut tries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for i in 0..n {
            for j in 0..m {
                tries[i][j] += get(i as i32 - 1, j as i32, &tries)
                    + get(i as i32, j as i32 - 1, &tries)
                    - get(i as i32 - 1, j as i32 - 1, &tries);
            }
        }
        tries
    }

    fn get(i: i32, j: i32, grid: &Vec<Vec<i32>>) -> i32 {
        if i < 0 || j < 0 {
            0
        } else {
            grid[i as usize][j as usize]
        }
    }

    fn sum(g: &Vec<Vec<i32>>, a: i32, b: i32, c: i32, d: i32) -> i32 {
        if a > c {
            return 0;
        } else {
            return g[c as usize][d as usize] - get(c, b - 1, g) - get(a - 1, d, g)
                + get(a - 1, b - 1, g);
        }
    }

    ans * ans
}

/**
 * 2132. 用邮票贴满网格图
 * https://leetcode.cn/problems/stamping-the-grid/
 */
pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
    let n = grid.len();
    let m = grid[0].len();
    let mut sum = vec![vec![0; m + 1]; n + 1];
    let mut diff = vec![vec![0; m + 2]; n + 2];

    // sum是前缀和数组
    // 查询原始矩阵中的某个范围的累加和很快速
    for i in 0..n {
        for j in 0..m {
            sum[i + 1][j + 1] = grid[i][j];
        }
    }

    let sum = build(sum);

    // 差分矩阵
    // 当贴邮票的时候，不再原始矩阵里贴，在差分矩阵里贴
    // 原始矩阵就用来判断能不能贴邮票，不进行修改
    // 每贴一张邮票都在差分矩阵里修改
    let mut a: usize = 1;
    let mut c: usize = a + stamp_height as usize - 1;
    while c <= n {
        let mut b: usize = 1;
        let mut d: usize = b + stamp_width as usize - 1;
        while d <= m {
            // 原始矩阵中 (a,b)左上角点
            // 根据邮票规格，h、w，算出右下角点(c,d)
            // 这个区域彻底都是0，那么:
            // sumRegion(sum, a, b, c, d) == 0
            // 那么此时这个区域可以贴邮票
            if sum_region(&sum, a, b, c, d) == 0 {
                diff = add(diff, a, b, c, d);
            }
            b += 1;
            d += 1;
        }
        a += 1;
        c += 1;
    }

    let diff = build(diff);

    fn build(mut sum: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for i in 1..sum.len() {
            for j in 1..sum[0].len() {
                sum[i][j] += sum[i - 1][j] + sum[i][j - 1] - sum[i - 1][j - 1];
            }
        }
        sum
    }

    fn sum_region(sum: &Vec<Vec<i32>>, a: usize, b: usize, c: usize, d: usize) -> i32 {
        sum[c][d] - sum[c][b - 1] - sum[a - 1][d] + sum[a - 1][b - 1]
    }

    fn add(mut diff: Vec<Vec<i32>>, a: usize, b: usize, c: usize, d: usize) -> Vec<Vec<i32>> {
        diff[a][b] += 1;
        diff[c + 1][d + 1] += 1;
        diff[c + 1][b] -= 1;
        diff[a][d + 1] -= 1;
        diff
    }

    // 检查所有的格子！
    for i in 0..n {
        for j in 0..m {
            // 原始矩阵里：grid[i][j] == 0，说明是个洞
            // 差分矩阵里：diff[i + 1][j + 1] == 0，说明洞上并没有邮票
            // 此时返回false
            if grid[i][j] == 0 && diff[i + 1][j + 1] == 0 {
                return false;
            }
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_possible_to_stamp() {
        let grid: Vec<Vec<i32>> = vec![
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
        ];
        let stamp_height = 4;
        let stamp_width = 3;
        assert_eq!(possible_to_stamp(grid, stamp_height, stamp_width), true);
    }
}
