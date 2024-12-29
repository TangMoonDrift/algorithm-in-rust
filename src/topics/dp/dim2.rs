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
 * 1143. 最长公共子序列
 * https://leetcode.cn/problems/longest-common-subsequence/description/
 */
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let n = text1.len();
    let m = text2.len();
    let (chars1, chars2, rows, cols) = if n >= m {
        (
            text1.chars().collect::<Vec<char>>(),
            text2.chars().collect::<Vec<char>>(),
            n,
            m,
        )
    } else {
        (
            text2.chars().collect::<Vec<char>>(),
            text1.chars().collect::<Vec<char>>(),
            m,
            n,
        )
    };
    let mut dp = vec![0; cols + 1];

    for i in 1..=rows {
        let mut left_up = 0;
        for j in 1..=cols {
            let backup = dp[j];
            if chars1[i - 1] == chars2[j - 1] {
                dp[j] = left_up + 1;
            } else {
                dp[j] = dp[j].max(dp[j - 1]);
            }
            left_up = backup;
        }
    }

    dp[cols]
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

/**
 * 72. 编辑距离
 * https://leetcode.cn/problems/edit-distance/description/
 */
pub fn min_distance(word1: String, word2: String) -> i32 {
    fn f(word1: Vec<char>, word2: Vec<char>, a: usize, b: usize, c: usize) -> i32 {
        let n = word1.len();
        let m = word2.len();
        let mut dp = vec![0; m + 1];

        for j in 1..=m {
            dp[j] = j * a;
        }

        let (mut left_up, mut backup): (usize, usize);
        for i in 1..=n {
            left_up = (i - 1) * b;
            dp[0] = i * b;
            for j in 1..=m {
                backup = dp[j];
                if word1[i - 1] == word2[j - 1] {
                    dp[j] = left_up;
                } else {
                    dp[j] = (dp[j] + b).min((dp[j - 1] + a).min(left_up + c));
                }
                left_up = backup;
            }
        }

        dp[m] as i32
    }

    f(word1.chars().collect(), word2.chars().collect(), 1, 1, 1)
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

    #[test]
    fn test_min_distance() {
        assert_eq!(min_distance("horse".to_string(), "ros".to_string()), 3);
        assert_eq!(
            min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }
}
