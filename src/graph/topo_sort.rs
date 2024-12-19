//! 拓扑排序专题
use crate::graph::Graph;
use std::collections::HashMap;

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
