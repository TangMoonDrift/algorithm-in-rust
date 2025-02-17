//! 拓扑排序专题
use std::vec;

use crate::graph::{DynamicGraph, Graph};

/**
 * 210. 课程表 II
 * https://leetcode.cn/problems/course-schedule-ii/description/
 */
pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let edges: Vec<Vec<usize>> = prerequisites
        .iter()
        .map(|edge| edge.iter().map(|&e| e as usize).rev().collect())
        .collect();
    let n = num_courses as usize;
    let graph = Graph::build(n, &edges, true);
    let mut in_degree = vec![0; n];
    let mut queue = vec![0; n];
    let (mut l, mut r) = (0, 0);

    for edge in &edges {
        in_degree[edge[1]] += 1;
    }

    for i in 0..n {
        if in_degree[i] == 0 {
            queue[r] = i as i32;
            r += 1;
        }
    }

    let mut cnt = 0;
    while l < r {
        let curr = queue[l];
        l += 1;
        cnt += 1;
        let neighbors = graph.collect_neighbors(curr as usize);
        for (node, ..) in neighbors {
            in_degree[node] -= 1;
            if in_degree[node] == 0 {
                queue[r] = node as i32;
                r += 1;
            }
        }
    }

    return if cnt == n { queue } else { vec![] };
}

/**
 * LCR 114. 火星词典
 * https://leetcode.cn/problems/Jf1JuT/description/
 */
pub fn alien_order(words: Vec<String>) -> String {
    const A: usize = 'a' as usize;
    let mut in_degree = [-1; 26];
    words.iter().for_each(|word| {
        word.bytes().for_each(|code| {
            in_degree[code as usize - A] = 0;
        })
    });

    let mut graph = DynamicGraph::new(26);
    for i in 0..(words.len() - 1) {
        let current = words[i].chars().collect::<Vec<char>>();
        let next = words[i + 1].chars().collect::<Vec<char>>();
        let mut j = 0;
        while j < current.len().min(next.len()) {
            if current[j] != next[j] {
                graph.add_edge(current[j] as usize - A, next[j] as usize - A, 0);
                in_degree[next[j] as usize - A] += 1;
                break;
            }
            j += 1;
        }

        if j < current.len() && j == next.len() {
            return String::from("");
        }
    }

    let mut queue = vec![0; 26];
    let (mut l, mut r) = (0, 0);
    let mut kinds = 0;
    for i in 0..26 {
        if in_degree[i] != -1 {
            kinds += 1;
        }
        if in_degree[i] == 0 {
            queue[r] = i;
            r += 1;
        }
    }

    let mut ans = Vec::new();
    while l < r {
        let current = queue[l];
        l += 1;
        ans.push(current as u8 + A as u8);
        let neighbors = graph.neighbors(current);
        for (node, _) in neighbors {
            in_degree[node] -= 1;
            if in_degree[node] == 0 {
                queue[r] = node;
                r += 1;
            }
        }
    }

    return if ans.len() == kinds {
        String::from_utf8(ans).unwrap()
    } else {
        String::from("")
    };
}

/**
 * 936. 戳印序列
 * https://leetcode.cn/problems/stamping-the-sequence/description/
 */
pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
    let m = stamp.len();
    let n = target.len();
    let s = stamp.chars().collect::<Vec<char>>();
    let t = target.chars().collect::<Vec<char>>();
    let mut graph = Graph::new(1000, 1000 * m);
    let mut in_degree = vec![m; n - m + 1];
    let mut queue = vec![0; n - m + 1];
    let (mut l, mut r) = (0, 0);

    for i in 0..=(n - m) {
        for j in 0..m {
            if t[i + j] == s[j] {
                in_degree[i] -= 1;
                if in_degree[i] == 0 {
                    queue[r] = i;
                    r += 1;
                }
            } else {
                graph.add_edge(i + j, i, 0);
            }
        }
    }

    let mut visited = vec![false; n];
    let mut path = vec![0; n - m + 1];
    let mut size = 0;
    while l < r {
        let curr = queue[l];
        l += 1;
        path[size] = curr as i32;
        size += 1;

        for i in 0..m {
            if !visited[curr + i] {
                visited[curr + i] = true;
                let neighbors = graph.collect_neighbors(curr + i);
                for neighbor in neighbors {
                    let neighbor = neighbor.0;
                    in_degree[neighbor] -= 1;
                    if in_degree[neighbor] == 0 {
                        queue[r] = neighbor;
                        r += 1;
                    }
                }
            }
        }
    }

    if size != (n - m + 1) {
        return vec![];
    }

    path.reverse();
    path
}

/**
 * 851. 喧闹和富有
 * https://leetcode.cn/problems/loud-and-rich/
*/
pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
    let n = quiet.len();
    let m = richer.len();
    let mut ans = (0..n as i32).collect::<Vec<i32>>();
    let mut queue = vec![0; n];
    let (mut l, mut r) = (0, 0);
    let mut in_degree = vec![0; n];

    let mut graph = Graph::new(n, m);

    richer.iter().for_each(|r| {
        graph.add_edge(r[0] as usize, r[1] as usize, 0);
        in_degree[r[1] as usize] += 1;
    });

    for i in 0..n {
        if in_degree[i] == 0 {
            queue[r] = i;
            r += 1;
        }
    }

    while l < r {
        let curr = queue[l];
        l += 1;
        let neighbors = graph.collect_neighbors(curr);
        for neighbor in neighbors {
            let neighbor = neighbor.0;
            if quiet[ans[curr] as usize] < quiet[ans[neighbor] as usize] {
                ans[neighbor] = ans[curr];
            }
            in_degree[neighbor] -= 1;
            if in_degree[neighbor] == 0 {
                queue[r] = neighbor;
                r += 1;
            }
        }
    }

    ans
}

/**
 * 2050. 并行课程 III
 * https://leetcode.cn/problems/parallel-courses-iii/description/
 */
pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut ans = 0;
    let mut in_degree = vec![0; n + 1];
    let mut queue = vec![0; n];
    let (mut l, mut r) = (0, 0);
    let mut graph = Graph::new(n, relations.len());
    let mut costs = vec![0; n + 1];

    relations.iter().for_each(|relation| {
        graph.add_edge(relation[0] as usize, relation[1] as usize, 0);
        in_degree[relation[1] as usize] += 1;
    });

    for i in 1..=n {
        if in_degree[i] == 0 {
            queue[r] = i;
            r += 1;
        }
    }

    while l < r {
        let curr = queue[l];
        l += 1;
        let neighbors = graph.collect_neighbors(curr);
        costs[curr] += time[curr - 1];
        ans = ans.max(costs[curr]);
        for neighbor in neighbors {
            let neighbor = neighbor.0;
            costs[neighbor] = costs[neighbor].max(costs[curr]);
            in_degree[neighbor] -= 1;
            if in_degree[neighbor] == 0 {
                queue[r] = neighbor;
                r += 1;
            }
        }
    }

    ans
}

/**
 * 2127. 参加会议的最多员工数
 * https://leetcode.cn/problems/maximum-employees-to-be-invited-to-a-meeting/description/
 */
pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
    let n = favorite.len();
    let mut in_degree = vec![0; n];
    let mut queue = vec![0; n];
    let (mut l, mut r) = (0, 0);
    let mut deep = vec![0; n];

    for i in 0..n {
        in_degree[favorite[i] as usize] += 1;
    }
    for i in 0..n {
        if in_degree[i] == 0 {
            queue[r] = i;
            r += 1;
        }
    }

    while l < r {
        let curr = queue[l];
        l += 1;
        let next = favorite[curr] as usize;
        deep[next] = deep[next].max(deep[curr] + 1);
        in_degree[next] -= 1;
        if in_degree[next] == 0 {
            queue[r] = next;
            r += 1;
        }
    }

    let mut sum_of_small_ring = 0;
    let mut sum_of_big_ring = 0;

    for i in 0..n {
        if in_degree[i] > 0 {
            let mut ring_size = 1;
            in_degree[i] = 0;
            let mut j = favorite[i] as usize;
            while j != i {
                ring_size += 1;
                in_degree[j] = 0;
                j = favorite[j] as usize;
            }
            if ring_size == 2 {
                sum_of_small_ring += 2 + deep[i] + deep[favorite[i] as usize];
            } else {
                sum_of_big_ring = sum_of_big_ring.max(ring_size);
            }
        }
    }

    sum_of_big_ring.max(sum_of_small_ring)
}
