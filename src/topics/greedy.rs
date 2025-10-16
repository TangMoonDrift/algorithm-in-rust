pub mod utils;

use crate::number_theoretic::power::power;
use std::cmp::Ordering;
use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet},
};
#[derive(Eq, PartialEq)]
struct Node(i32, usize, usize);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).then(self.1.cmp(&other.1))
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
        .fold(0, |i, (&k, &v)| i + (v + k + 1 - 1) / (k + 1) * (k + 1))

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
    // 定义公共处理函数避免重复代码
    fn process(v: &[i32]) -> Vec<i64> {
        let (mut odds, mut evens): (Vec<i64>, Vec<i64>) =
            v.iter().map(|&x| x as i64).partition(|&x| x % 2 != 0);

        odds.sort_unstable(); // 使用更快的非稳定排序
        evens.sort_unstable();
        odds.extend(evens); // 直接复用已分配的vec
        odds // 返回组合后的结果
    }

    // 并行处理两个输入数组
    let nums_sorted = process(&nums);
    let target_sorted = process(&target);

    // 使用迭代器优化差值计算
    nums_sorted
        .iter()
        .zip(target_sorted.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i64>()
        / 4
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

/**
 * 45. 跳跃游戏 II
 * https://leetcode.cn/problems/jump-game-ii/description/
 */
pub fn jump(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let (mut curr, mut next, mut ans) = (0, 0, 0);

    for i in 0..n {
        if curr < i as i32 {
            ans += 1;
            curr = next;
        }
        next = next.max(i as i32 + nums[i]);
    }

    ans
}

/**
 * 1792. 最大平均通过率
 * https://leetcode.cn/problems/maximum-average-pass-ratio/description/
 */
pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
    #[derive(PartialEq)]
    struct Item(f64, f64, f64);

    impl Ord for Item {
        fn cmp(&self, other: &Self) -> Ordering {
            other.2.partial_cmp(&self.2).unwrap_or(Ordering::Equal)
        }
    }

    impl PartialOrd for Item {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(other.cmp(self))
        }
    }

    impl Eq for Item {}

    impl Into<Item> for (f64, f64, f64) {
        fn into(self) -> Item {
            Item(self.0, self.1, self.2)
        }
    }
    let mut heap: BinaryHeap<Item> = BinaryHeap::new();
    let n = classes.len();
    for class in &classes {
        let a = class[0] as f64;
        let b = class[1] as f64;
        heap.push((a, b, (a + 1.0) / (b + 1.0) - a / b).into());
    }

    for _ in 0..extra_students {
        let Item(a, b, _) = heap.pop().unwrap();
        heap.push(
            (
                a + 1.0,
                b + 1.0,
                (a + 2.0) / (b + 2.0) - (a + 1.0) / (b + 1.0),
            )
                .into(),
        );
    }

    let mut ans = 0.0;
    while !heap.is_empty() {
        let Item(a, b, _) = heap.pop().unwrap();
        ans += a / b;
    }

    ans / n as f64
}

/**
 * 857. 雇佣 K 名工人的最低成本
 * https://leetcode.cn/problems/minimum-cost-to-hire-k-workers/description/
 */
pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
    let n = quality.len();
    let k = k as usize;
    let mut id = (0..n).collect::<Vec<_>>();
    // 按照 r 值排序
    id.sort_unstable_by(|&i, &j| (wage[i] * quality[j]).cmp(&(wage[j] * quality[i])));

    let mut h = BinaryHeap::new();
    let mut sum_q = 0;
    for i in 0..k {
        h.push(quality[id[i]]);
        sum_q += quality[id[i]];
    }

    // 选 r 值最小的 k 名工人
    let mut ans = sum_q as f64 * wage[id[k - 1]] as f64 / quality[id[k - 1]] as f64;

    // 后面的工人 r 值更大
    // 但是 sum_q 可以变小，从而可能得到更优的答案
    for i in k..n {
        let q = quality[id[i]];
        if q < *h.peek().unwrap() {
            sum_q -= h.pop().unwrap() - q;
            h.push(q);
            ans = ans.min(sum_q as f64 * wage[id[i]] as f64 / q as f64);
        }
    }
    ans
}

/**
 * 计算点组成的正方形数量
 */
pub fn count_squares(points: Vec<(i32, i32)>) -> usize {
    let point_set: HashSet<(i32, i32)> = points.iter().cloned().collect();
    let mut count = 0;
    let n = points.len();

    for i in 0..n {
        let (x1, y1) = points[i];
        for j in (i + 1)..n {
            let (x2, y2) = points[j];
            let dx = x2 - x1;
            let dy = y2 - y1;

            // 计算逆时针旋转90度后的两个点
            let (x3, y3) = (x1 - dy, y1 + dx);
            let (x4, y4) = (x2 - dy, y2 + dx);
            if point_set.contains(&(x3, y3)) && point_set.contains(&(x4, y4)) {
                count += 1;
            }

            // 计算顺时针旋转90度后的两个点
            let (x5, y5) = (x1 + dy, y1 - dx);
            let (x6, y6) = (x2 + dy, y2 - dx);
            if point_set.contains(&(x5, y5)) && point_set.contains(&(x6, y6)) {
                count += 1;
            }
        }
    }

    count / 4
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

    #[test]
    fn test_squares() {
        // 标准正方形
        let points = vec![(0, 0), (0, 1), (1, 0), (1, 1)];
        assert_eq!(count_squares(points), 1);

        // 两个独立的正方形
        let points = vec![
            (0, 0),
            (0, 1),
            (1, 0),
            (1, 1),
            (2, 0),
            (2, 1),
            (3, 0),
            (3, 1),
        ];
        assert_eq!(count_squares(points), 2);

        // 无正方形
        let points = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
        assert_eq!(count_squares(points), 0);
    }
}
