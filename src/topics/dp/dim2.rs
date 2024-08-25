//? 二维动态规划专题

/**
 * 64. 最小路径和
 * https://leetcode.cn/problems/minimum-path-sum/description/
 */
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![0; n];
    dp[0] = grid[0][0];

    for i in 1..n {
        dp[i] = dp[i - 1] + grid[0][i];
    }

    for i in 1..m {
        dp[0] += grid[i][0];
        for j in 1..n {
            dp[j] = if dp[j - 1] < dp[j] {
                dp[j - 1] + grid[i][j]
            } else {
                dp[j] + grid[i][j]
            }
        }
    }

    dp[n - 1]
}

/**
 * 115. 不同的子序列
 * https://leetcode.cn/problems/distinct-subsequences/description/
 */
pub fn num_distinct(s: String, t: String) -> i32 {
    let t: Vec<char> = t.chars().collect();
    let m = t.len();

    let mut dp = vec![0; m + 1];
    dp[0] = 1;

    for c in s.chars() {
        for i in (1..=m).rev() {
            dp[i] += if c == t[i - 1] { dp[i - 1] } else { 0 };
        }
    }

    dp[m]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_path_sum() {
        assert_eq!(
            min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
        assert_eq!(min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]), 12);
    }

    #[test]
    fn test_num_distinct() {
        assert_eq!(num_distinct("rabbbit".to_string(), "rabbit".to_string()), 3);
        assert_eq!(num_distinct("babgbag".to_string(), "bag".to_string()), 5);
    }
}
