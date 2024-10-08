pub mod utils;

use std::collections::{BinaryHeap, HashMap};
use utils::power;

/**
 * 179. 最大数
 * https://leetcode.cn/problems/largest-number/description/
 */
pub fn largest_number(mut nums: Vec<i32>) -> String {
    nums.sort_unstable_by(|&a, &b| format!("{}{}", b, a).cmp(&format!("{}{}", a, b)));

    if nums[0] == 0 {
        return "0".to_string();
    }

    nums.iter().map(|&num| num.to_string()).collect::<String>()
}

/**
 * 1029. 两地调度
 * https://leetcode.cn/problems/two-city-scheduling/description/
 */
pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
    let n = costs.len();
    let m = n >> 1;

    let mut diff = costs
        .iter()
        .map(|cost| cost[1] - cost[0])
        .collect::<Vec<i32>>();
    diff.sort();

    costs.iter().map(|cost| cost[0]).sum::<i32>() + diff.iter().take(m).sum::<i32>()
}

/**
 * 1553. 吃掉 N 个橘子的最少天数
 * https://leetcode.cn/problems/minimum-number-of-days-to-eat-n-oranges/description/
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
 * https://leetcode.cn/problems/course-schedule-iii/description/
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

/**
 * 581. 最短无序连续子数组
 * https://leetcode.cn/problems/shortest-unsorted-continuous-subarray/description/
 */
pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut max = i32::MIN;
    let mut min = i32::MAX;
    let mut begin = -1;
    let mut end = -1;

    for i in 0..len {
        if max > nums[i] {
            end = i as i32;
        } else {
            max = nums[i];
        }

        if min < nums[len - i - 1] {
            begin = len as i32 - i as i32 - 1;
        } else {
            min = nums[len - i - 1];
        }
    }

    if end == -1 {
        0
    } else {
        end - begin + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_number() {
        assert_eq!(largest_number(vec![10, 2]), "210");
        assert_eq!(largest_number(vec![3, 30, 34, 5, 9]), "9534330");
        assert_eq!(largest_number(vec![1]), "1");
    }

    #[test]
    fn test_two_city_sched_cost() {
        assert_eq!(
            two_city_sched_cost(vec![
                vec![10, 20],
                vec![30, 200],
                vec![400, 50],
                vec![30, 20]
            ]),
            410
        );
    }

    #[test]
    fn test_min_days() {
        assert_eq!(min_days(10), 4);
        assert_eq!(min_days(6), 3);
    }
}
