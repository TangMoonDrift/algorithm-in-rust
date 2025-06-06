//! 区间DP专题

/**
 * 1312. 让字符串成为回文串的最少插入次数
 * https://leetcode.cn/problems/minimum-insertion-steps-to-make-a-string-palindrome/description/
 */
pub fn min_insertions(s: String) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    if n == 1 {
        return 0;
    } else if n == 2 {
        return if bytes[0] == bytes[1] { 0 } else { 1 };
    }

    let mut dp = vec![0; n];
    dp[n - 1] = if bytes[n - 1] == bytes[n - 2] { 0 } else { 1 };

    for l in (0..(n - 2)).rev() {
        let mut left_down = dp[l + 1];
        dp[l + 1] = if bytes[l] == bytes[l + 1] { 0 } else { 1 };
        for r in (l + 2)..n {
            let backup = dp[r];
            if bytes[l] == bytes[r] {
                dp[r] = left_down;
            } else {
                dp[r] = dp[r].min(dp[r - 1]) + 1;
            }
            left_down = backup;
        }
    }

    dp[n - 1]
}

/**
 * 486. 预测赢家
 * https://leetcode.cn/problems/predict-the-winner/description/
 */
pub fn predict_the_winner(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n == 1 || n == 2 {
        return true;
    }

    let first = || -> i32 {
        let mut dp = vec![vec![0; n]; n];

        for i in 0..n - 1 {
            dp[i][i] = nums[i];
            dp[i][i + 1] = nums[i].max(nums[i + 1]);
        }

        dp[n - 1][n - 1] = nums[n - 1];

        for l in (0..=(n - 3)).rev() {
            for r in (l + 2)..n {
                dp[l][r] = (nums[l] + dp[l + 2][r].min(dp[l + 1][r - 1]))
                    .max(nums[r] + dp[l + 1][r - 1].min(dp[l][r - 2]));
            }
        }

        dp[0][n - 1]
    };

    if first() >= nums.iter().sum::<i32>() - first() {
        true
    } else {
        false
    }
}

/**
 * 312. 戳气球
 * https://leetcode.cn/problems/burst-balloons/description/
 */
pub fn max_coins(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut balloons = nums.clone();
    balloons.push(1);
    balloons.insert(0, 1);

    let mut dp = vec![vec![0; n + 2]; n + 2];

    for i in 1..=n {
        dp[i][i] = balloons[i - 1] * balloons[i] * balloons[i + 1];
    }

    for l in (1..=n).rev() {
        for r in (l + 1)..=n {
            let mut ans = (balloons[l - 1] * balloons[l] * balloons[r + 1] + dp[l + 1][r])
                .max(balloons[l - 1] * balloons[r] * balloons[r + 1] + dp[l][r - 1]);
            for k in (l + 1)..r {
                ans = ans.max(
                    balloons[l - 1] * balloons[k] * balloons[r + 1] + dp[l][k - 1] + dp[k + 1][r],
                );
            }
            dp[l][r] = ans;
        }
    }

    dp[1][n]
}

/**
 * https://leetcode.cn/problems/boolean-evaluation-lcci/description/
 * 08.14. 布尔运算
 */
pub fn count_eval(s: String, result: i32) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    let mut dp = vec![vec![vec![-1, -1]; n]; n];
    fn f(s: &Vec<char>, l: usize, r: usize, dp: &mut Vec<Vec<Vec<i32>>>) -> [i32; 2] {
        if dp[l][r][0] != -1 && dp[l][r][1] != -1 {
            return [dp[l][r][0], dp[l][r][1]];
        }

        let mut flag_true = 0;
        let mut flag_false = 0;
        if l == r {
            if s[l] == '0' {
                flag_false = 1;
            } else {
                flag_true = 1;
            }
        } else {
            for k in ((l + 1)..r).step_by(2) {
                let res = f(s, l, k - 1, dp);
                let a = res[0];
                let b = res[1];
                let res = f(s, k + 1, r, dp);
                let c = res[0];
                let d = res[1];
                if s[k] == '|' {
                    flag_false += a * c;
                    flag_true += a * d + b * d + b * c;
                } else if s[k] == '&' {
                    flag_false += a * d + a * c + b * c;
                    flag_true += b * d;
                } else if s[k] == '^' {
                    flag_false += a * c + b * d;
                    flag_true += a * d + b * c;
                }
            }
        }

        let ans = [flag_false, flag_true];
        dp[l][r] = ans.to_vec();
        ans
    }

    f(&s, 0, n - 1, &mut dp)[result as usize]
}

/**
 * 664. 奇怪的打印机
 * https://leetcode.cn/problems/strange-printer/description/
 */
pub fn strange_printer(s: String) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    if n == 1 {
        return 1;
    }

    let mut dp = vec![vec![0; n]; n];
    dp[n - 1][n - 1] = 1;
    for i in 0..(n - 1) {
        dp[i][i] = 1;
        dp[i][i + 1] = if s[i] == s[i + 1] { 1 } else { 2 };
    }

    for l in (0..(n - 2)).rev() {
        for r in (l + 2)..n {
            if s[l] == s[r] {
                dp[l][r] = dp[l][r - 1];
            } else {
                let mut ans = i32::MAX;
                for m in l..r {
                    ans = ans.min(dp[l][m] + dp[m + 1][r]);
                }
                dp[l][r] = ans;
            }
        }
    }

    dp[0][n - 1]
}

/**
 * 730. 统计不同回文子序列
 * https://leetcode.cn/problems/count-different-palindromic-subsequences/description/
 */
pub fn count_palindromic_subsequences(s: String) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let s = s.as_bytes();
    let n = s.len();

    // 初始化 last 数组
    let mut last = vec![-1; 256];
    let mut left = vec![0; n];
    for i in 0..n {
        left[i] = last[s[i] as usize] as usize;
        last[s[i] as usize] = i as i32;
    }

    // 重置 last 数组
    last.fill(n as i32);
    let mut right = vec![0; n];
    for i in (0..n).rev() {
        right[i] = last[s[i] as usize] as usize;
        last[s[i] as usize] = i as i32;
    }

    // 初始化 dp 数组
    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        dp[i][i] = 1;
    }

    // 动态规划计算
    for i in (0..n - 1).rev() {
        for j in i + 1..n {
            if s[i] != s[j] {
                dp[i][j] = (dp[i][j - 1] + dp[i + 1][j] - dp[i + 1][j - 1] + MOD) % MOD;
            } else {
                let l = right[i];
                let r = left[j];
                if l > r {
                    dp[i][j] = (dp[i + 1][j - 1] * 2 + 2) % MOD;
                } else if l == r {
                    dp[i][j] = (dp[i + 1][j - 1] * 2 + 1) % MOD;
                } else {
                    dp[i][j] = (dp[i + 1][j - 1] * 2 - dp[l + 1][r - 1] + MOD) % MOD;
                }
            }
        }
    }

    dp[0][n - 1] as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_the_winner() {
        assert_eq!(predict_the_winner(vec![1, 5, 2]), false);
        assert_eq!(predict_the_winner(vec![1, 5, 233, 7]), true);
        assert_eq!(predict_the_winner(vec![1, 5, 233, 7, 1]), true);
        assert_eq!(predict_the_winner(vec![0]), true);
        assert_eq!(predict_the_winner(vec![0, 1]), true);
    }
}
