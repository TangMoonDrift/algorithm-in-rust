//！ 背包专题
// use std::collections::HashMap;

/**
 * 494. 目标和
 * https://leetcode.cn/problems/target-sum/description/
 */
pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    // 记忆化搜索解法
    // let mut map: HashMap<(usize, i32), i32> = HashMap::new();
    // fn f(
    //     nums: &Vec<i32>,
    //     target: i32,
    //     i: usize,
    //     sum: i32,
    //     map: &mut HashMap<(usize, i32), i32>,
    // ) -> i32 {
    //     if i == nums.len() {
    //         return if sum == target { 1 } else { 0 };
    //     }
    //     match map.get(&(i, sum)) {
    //         Some(&val) => val,
    //         None => {
    //             let ans = f(nums, target, i + 1, sum + nums[i], map)
    //                 + f(nums, target, i + 1, sum - nums[i], map);
    //             map.insert((i, sum), ans);
    //             return ans;
    //         }
    //     }
    // }
    // f(&nums, target, 0, 0, &mut map)

    // 动态规划解法
    // let sum: i32 = nums.iter().sum();
    // if target < -sum || target > sum {
    //     return 0;
    // }
    // let n = nums.len();
    // let m = sum * 2 + 1;
    // let mut dp = vec![vec![0; m as usize]; n + 1];
    // dp[n][target as usize + sum as usize] = 1;
    // for i in (0..=(n - 1)).rev() {
    //     for j in -sum..=sum {
    //         if j + nums[i] + sum < m {
    //             dp[i][j as usize + sum as usize] =
    //                 dp[i + 1][j as usize + nums[i] as usize + sum as usize];
    //         }
    //         if j - nums[i] + sum >= 0 {
    //             dp[i][j as usize + sum as usize] +=
    //                 dp[i + 1][j as usize - nums[i] as usize + sum as usize];
    //         }
    //     }
    // }
    // dp[0][sum as usize]

    // 01 背包解法（最优解）
    let sum: i32 = nums.iter().sum();
    if target < -sum || target > sum {
        return 0;
    }
    if (target & 1) ^ (sum & 1) == 1 {
        return 0;
    }
    fn sub_set(nums: &Vec<i32>, t: i32) -> i32 {
        if t < 0 {
            return 0;
        }
        let mut dp = vec![0; t as usize + 1];
        dp[0] = 1;
        for &num in nums {
            for i in (num..=t).rev() {
                dp[i as usize] += dp[i as usize - num as usize];
            }
        }
        dp[t as usize]
    }
    sub_set(&nums, (target + sum) / 2)
}

/**
 * 1049. 最后一块石头的重量 II
 * https://leetcode.cn/problems/last-stone-weight-ii/description/
 */
pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
    let sum: i32 = stones.iter().sum();

    fn near(stones: &Vec<i32>, t: usize) -> i32 {
        let mut dp = vec![0; t + 1];

        for &stone in stones {
            for i in (stone as usize..=t).rev() {
                dp[i] = dp[i].max(dp[i - stone as usize] + stone);
            }
        }

        dp[t]
    }

    sum - near(&stones, (sum / 2) as usize) * 2
}

/**
 * 2218. 从栈中取出 K 个硬币的最大面值和
 * https://leetcode.cn/problems/maximum-value-of-k-coins-from-piles/description/
 */
pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
    let k = k as usize;
    let mut dp = vec![0; k + 1];
    piles.iter().for_each(|pile| {
        let t = pile.len().min(k);
        let mut prev_sum = vec![0; t + 1];
        let mut sum = 0;
        for i in 0..t {
            sum += pile[i];
            prev_sum[i + 1] = sum;
        }

        for i in (1..=k).rev() {
            let right = t.min(i);
            for j in 1..=right {
                dp[i] = dp[i].max(dp[i - j] + prev_sum[j]);
            }
        }
    });

    dp[k]
}

/**
 * 10. 正则表达式匹配
 * https://leetcode.cn/problems/regular-expression-matching/description/
 */
pub fn is_match(s: String, p: String) -> bool {
    let n = s.len();
    let m = p.len();
    let chars_1: Vec<char> = s.chars().collect();
    let chars_2: Vec<char> = p.chars().collect();

    let mut dp = vec![vec![false; m + 1]; n + 1];
    dp[n][m] = true;

    for i in (0..=(m - 1)).rev() {
        dp[n][i] = i + 1 < m && chars_2[i + 1] == '*' && dp[n][i + 2];
    }

    for i in (0..=(n - 1)).rev() {
        for j in (0..=(m - 1)).rev() {
            if j + 1 == m || chars_2[j + 1] != '*' {
                dp[i][j] = (chars_1[i] == chars_2[j] || chars_2[j] == '.') && dp[i + 1][j + 1];
            } else {
                dp[i][j] = dp[i][j + 2]
                    || ((chars_1[i] == chars_2[j] || chars_2[j] == '.') && dp[i + 1][j]);
            }
        }
    }

    dp[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_target_sum_ways() {
        assert_eq!(find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
        assert_eq!(find_target_sum_ways(vec![1], 1), 1);
    }

    #[test]
    fn test_last_stone_weight_ii() {
        assert_eq!(last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]), 1);
        assert_eq!(last_stone_weight_ii(vec![31, 26, 33, 21, 40]), 5);
    }
}
