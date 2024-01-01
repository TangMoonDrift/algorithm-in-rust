pub mod utils;

use std::collections::{BinaryHeap, HashMap};
use utils::power;

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

/**
 * 题目描述：https://leetcode.cn/problems/course-schedule-iii/description/
 * 这里有 n 门不同的在线课程，按从 1 到 n 编号。给你一个数组 courses ，
 * 其中 courses[i] = [durationi, lastDayi] 表示第 i 门课将会 持续 上 durationi 天课，
 * 并且必须在不晚于 lastDayi 的时候完成。
 *
 * 你的学期从第 1 天开始。且不能同时修读两门及两门以上的课程。
 * 返回你最多可以修读的课程数目。
 */
pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
    let mut courses: Vec<_> = courses
        .into_iter()
        .filter(|course| course[0] <= course[1])
        .collect();
    courses.sort_by_key(|course| course[1]);

    if courses.is_empty() {
        return 0;
    }

    let mut cur_time = 0;
    let mut heap = BinaryHeap::new();

    for course in courses {
        let (duration, deadline) = (course[0], course[1]);
        if cur_time + duration <= deadline {
            cur_time += duration;
            heap.push(duration);
        } else {
            if let Some(&d) = heap.peek() {
                if duration < d {
                    cur_time = cur_time - d + duration;
                    heap.pop();
                    heap.push(duration);
                }
            }
        }
    }

    heap.len() as i32
}

/**
 * 将一个数字拆分成 k 个非负整数的和，要求这 k 个整数的乘积。
 */
pub fn max_value(n: i32, k: usize) -> i32 {
    let model = 1000000007;
    let avg = n / k as i32;
    let rest = n % k as i32;

    let avgs_group = power(avg, k as i32 - rest, model);
    let non_avgs_group = power(avg + 1, rest, model);

    avgs_group * non_avgs_group % model
}
