//? 区间DP专题

/**
 * 486. 预测赢家
 * https://leetcode.cn/problems/predict-the-winner/description/
 */
pub fn predict_the_winner(nums: Vec<i32>) -> bool {
    let sum: i32 = nums.iter().sum();
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

    if first() >= sum - first() {
        true
    } else {
        false
    }
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
