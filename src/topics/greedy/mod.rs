use std::collections::HashMap;

/**
 * 题目描述：https://leetcode.cn/problems/minimum-number-of-days-to-eat-n-oranges/description/
 * 厨房里总共有 n 个橘子，你决定每一天选择如下方式之一吃这些橘子：
 *
 * 吃掉一个橘子。
 * 如果剩余橘子数 n 能被 2 整除，那么你可以吃掉 n/2 个橘子。
 * 如果剩余橘子数 n 能被 3 整除，那么你可以吃掉 2*(n/3) 个橘子。
 * 每天你只能从以上 3 种方案中选择一种方案。

 * 请你返回吃掉所有 n 个橘子的最少天数。
*/
pub fn min_days(n: i32) -> i32 {
    let mut dp = HashMap::new();
    dp.insert(0, 0);

    fn help(n: i32, dp: &mut HashMap<i32, i32>) -> i32 {
        if let Some(&ans) = dp.get(&n) {
            return ans;
        }
        let ans = n
            .min(help(n / 2, dp) + 1 + n % 2)
            .min(help(n / 3, dp) + 1 + n % 3);
        dp.insert(n, ans);
        ans
    }

    help(n, &mut dp)
}
