pub mod utils;

use std::cmp::Ordering;
use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashMap},
};
use utils::power;

#[derive(Eq, PartialEq)]
struct Node(i32, usize, usize);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 == other.0 {
            self.1.cmp(&other.1)
        } else {
            self.0.cmp(&other.0)
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Into<Node> for (i32, usize, usize) {
    fn into(self) -> Node {
        Node(self.0, self.1, self.2)
    }
}

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
 * 630. 课程表 III
 * https://leetcode.cn/problems/course-schedule-iii/description/
 */
pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
    let mut courses = courses;
    let mut duration = 0;
    let mut heap = BinaryHeap::new();

    courses.sort_by_key(|course| course[1]);
    courses.iter().for_each(|course| {
        let (d, deadline) = (course[0], course[1]);
        match d + duration <= deadline {
            true => {
                heap.push(d);
                duration += d;
            }
            false => {
                if let Some(&top) = heap.peek() {
                    if top > d {
                        duration = duration + d - heap.pop().unwrap();
                        heap.push(d);
                    }
                }
            }
        }
    });

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
 * 502. IPO
 * https://leetcode.cn/problems/ipo/description/
 */
pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let mut pairs = capital
        .iter()
        .zip(profits.iter())
        .map(|(c, p)| (Reverse(c), p))
        .collect::<BinaryHeap<_>>();
    let mut heap = BinaryHeap::new();
    let mut ans = w;

    for _ in 0..k {
        loop {
            match pairs.peek() {
                Some(&(Reverse(&c), p)) if c <= ans => {
                    heap.push(p);
                    pairs.pop();
                }
                _ => break,
            }
        }

        if let Some(p) = heap.pop() {
            ans += p;
        } else {
            break;
        }
    }

    ans
}

/**
 * 581. 最短无序连续子数组
 * https://leetcode.cn/problems/shortest-unsorted-continuous-subarray/description/
 */
pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let (mut max, mut min, mut l, mut r) = (i32::MIN, i32::MAX, len, 0);

    for i in 0..len {
        match max > nums[i] {
            true => {
                r = i;
            }
            false => {
                max = nums[i];
            }
        }

        match min < nums[len - 1 - i] {
            true => {
                l = len - 1 - i;
            }
            false => {
                min = nums[len - 1 - i];
            }
        }
    }

    match r == 0 {
        true => 0,
        false => (r - l + 1) as i32,
    }
}

/**
 * 632. 最小区间
 * https://leetcode.cn/problems/smallest-range-covering-elements-from-k-lists/description/
 */
pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
    if nums.len() == 1 {
        return vec![nums[0][0], nums[0][0]];
    }

    let n = nums.len();
    let mut set: BTreeSet<Node> = BTreeSet::new();

    nums.iter().enumerate().for_each(|(index, num)| {
        set.insert((num[0], index, 0).into());
    });

    let mut distance = i32::MAX;
    let (mut l, mut r) = (0, 0);

    while set.len() == n {
        let min = set.pop_first().unwrap();
        if let Some(max) = set.last() {
            if max.0 - min.0 < distance {
                distance = max.0 - min.0;
                l = min.0;
                r = max.0;
            }
            if min.2 + 1 < nums[min.1].len() {
                set.insert((nums[min.1][min.2 + 1], min.1, min.2 + 1).into());
            }
        }
    }

    vec![l, r]
}

/**
 * 1675. 数组的最小偏移量
 * https://leetcode.cn/problems/minimize-deviation-in-array/description/
 */
pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
    let mut set = BTreeSet::new();
    for &num in &nums {
        if num % 2 == 0 {
            set.insert(num);
        } else {
            set.insert(num * 2);
        }
    }

    let mut ans = set.last().unwrap() - set.first().unwrap();
    while ans > 0 && set.last().unwrap() % 2 == 0 {
        let max = set.pop_last().unwrap();
        set.insert(max / 2);
        ans = ans.min(set.last().unwrap() - set.first().unwrap());
    }

    ans
}

/**
 * 781. 森林中的兔子
 * https://leetcode.cn/problems/rabbits-in-forest/description/
 */
pub fn num_rabbits(answers: Vec<i32>) -> i32 {
    answers
        .iter()
        .fold(HashMap::new(), |mut map, &x| {
            *map.entry(x).or_insert(0) += 1;
            map
        })
        .iter()
        .fold(0, |i, (&k, &v)| i + (k + v) / (k + 1) * (k + 1))

    // answers.sort_unstable();
    // let n = answers.len() as i32;

    // let (mut ans, mut i, mut j) = (0, 0, 1);
    // while j < n {
    //     let x = answers[i as usize];
    //     while j < n && answers[j as usize] == x {
    //         j += 1;
    //     }
    //     ans += (j - i + x) / (x + 1) * (x + 1);
    //     i = j;
    // }

    // ans
}

/**
 * 2449. 使数组相似的最少操作次数
 * https://leetcode.cn/problems/minimum-number-of-operations-to-make-arrays-similar/description/
 */
pub fn make_similar(nums: Vec<i32>, target: Vec<i32>) -> i64 {
    let vecs = vec![nums, target]
        .iter()
        .map(|v| {
            let (mut odds, mut evens): (Vec<i64>, Vec<i64>) =
                v.iter().map(|&x| x as i64).partition(|&x| x % 2 != 0);
            let mut sorted = Vec::new();
            odds.sort();
            evens.sort();
            sorted.extend(odds);
            sorted.extend(evens);
            sorted
        })
        .collect::<Vec<Vec<i64>>>();

    let mut ans = 0;
    for (i, j) in vecs[0].iter().zip(vecs[1].iter()) {
        ans += (i - j).abs();
    }

    ans / 4
}

/**
 * 871. 最低加油次数
 * https://leetcode.cn/problems/minimum-number-of-refueling-stops/description/
 */
pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
    if start_fuel >= target {
        return 0;
    }

    let mut to = start_fuel;
    let mut ans = 0;

    let mut heap = BinaryHeap::new();
    for station in &stations {
        let distance = station[0];
        let fuel = station[1];
        if to < distance {
            while !heap.is_empty() && to < distance {
                to += heap.pop().unwrap();
                ans += 1;
                if to >= target {
                    return ans;
                }
            }
            if to < distance {
                return -1;
            }
        }
        heap.push(fuel);
    }

    while !heap.is_empty() {
        to += heap.pop().unwrap();
        ans += 1;
        if to >= target {
            return ans;
        }
    }

    -1
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
