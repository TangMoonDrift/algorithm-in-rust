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
 * 851. 喧闹和富有
 * https://leetcode.cn/problems/loud-and-rich/
*/
pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
    let n = quiet.len();
    let mut ans = vec![0; n];
    let mut in_degree = vec![0; n];
    // let mut graph = vec![vec![]; n];
    let mut graph = HashMap::with_capacity(n);

    richer.iter().for_each(|item| {
        // graph[item[0] as usize].push(item[1]);
        graph.entry(item[0]).or_insert(vec![]).push(item[1]);
        in_degree[item[1] as usize] += 1;
    });

    let mut queue = vec![0; n];
    let (mut l, mut r) = (0, 0);
    for i in 0..n {
        ans[i] = i as i32;
        if in_degree[i] == 0 {
            queue[r] = i;
            r += 1;
        }
    }

    while l < r {
        let cur = queue[l];
        l += 1;

        // let nexts = &graph[cur];
        // nexts.iter().for_each(|next| {
        //     let next = *next as usize;
        //     if quiet[ans[cur] as usize] < quiet[ans[next] as usize] {
        //         ans[next] = ans[cur];
        //     }
        //     in_degree[next] -= 1;
        //     if in_degree[next] == 0 {
        //         queue[r] = next;
        //         r += 1;
        //     }
        // });

        if let Some(nexts) = graph.get(&(cur as i32)) {
            nexts.iter().for_each(|next: &i32| {
                let next = *next as usize;
                if quiet[ans[cur] as usize] < quiet[ans[next] as usize] {
                    ans[next] = ans[cur];
                }
                in_degree[next] -= 1;
                if in_degree[next] == 0 {
                    queue[r] = next;
                    r += 1;
                }
            });
        }
    }

    ans
}
