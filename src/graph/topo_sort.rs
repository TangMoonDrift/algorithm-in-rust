//! 拓扑排序专题
use crate::graph::Graph;
use std::collections::HashMap;

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
        for neighbor in &neighbors {
            let neighbor = neighbor.0;
            in_degree[neighbor] -= 1;
            if in_degree[neighbor] == 0 {
                queue[r] = neighbor as i32;
                r += 1;
            }
        }
    }

    return if cnt == n { queue } else { vec![] };
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
