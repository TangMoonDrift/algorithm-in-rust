//! 三维动态规划专题

/**
 * 474. 一和零
 * https://leetcode.cn/problems/ones-and-zeroes/description/
 */
pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let mut dp = vec![vec![0; n as usize + 1]; m as usize + 1];

    for s in strs {
        let zeros = s.chars().filter(|&c| c == '0').count();
        let ones = s.chars().filter(|&c| c == '1').count();
        for i in (zeros..=(m as usize)).rev() {
            for j in (ones..=(n as usize)).rev() {
                dp[i][j] = dp[i][j].max(1 + dp[i - zeros][j - ones]);
            }
        }
    }

    dp[m as usize][n as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_form() {
        assert_eq!(
            find_max_form(
                vec![
                    "10".to_string(),
                    "0001".to_string(),
                    "111001".to_string(),
                    "1".to_string(),
                    "0".to_string()
                ],
                5,
                3
            ),
            4
        );
        assert_eq!(
            find_max_form(
                vec!["10".to_string(), "0".to_string(), "1".to_string()],
                1,
                1
            ),
            2
        );
    }
}
