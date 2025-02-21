//! 最小生成树（Minimum Spanning Tree）专题

//! 最小生成树的英文是 Minimum Spanning Tree（简称 MST）。
//! 它是图论中的一个概念，指在一个带权无向图中，边的权重之和最小的生成树（即连接所有顶点的无环子图）。
//! 常见的算法包括 Prim 算法和 Kruskal 算法。

use crate::topics::dsu::UnionFind;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Edge {
    source: usize,
    destination: usize,
    weight: i32,
}

impl Edge {
    fn new(source: usize, destination: usize, weight: i32) -> Self {
        Edge {
            source,
            destination,
            weight,
        }
    }
}

fn kruskal(mut edges: Vec<Edge>, number_of_vertices: usize) -> (i32, Vec<Edge>) {
    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    let mut parent = (0..number_of_vertices).collect::<Vec<_>>();
    let mut total_cost = 0;
    let mut final_edges = Vec::new();

    edges.sort_unstable_by(|a, b| a.weight.cmp(&b.weight));

    for edge in edges {
        let x = find(&mut parent, edge.source);
        let y = find(&mut parent, edge.destination);

        if x != y {
            parent[x] = y;
            final_edges.push(edge.clone());
            total_cost += edge.weight;
        }
    }

    (total_cost, final_edges)
}

/**
 * 1697. 检查边长度限制的路径是否存在
 * https://leetcode.cn/problems/checking-existence-of-edge-length-limited-paths/
 */
pub fn distance_limited_paths_exist(
    n: i32,
    mut edges: Vec<Vec<i32>>,
    mut queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    edges.sort_by(|a, b| a[2].cmp(&b[2]));

    queries.iter_mut().enumerate().for_each(|(index, query)| {
        query.push(index as i32);
    });
    queries.sort_by(|a, b| a[2].cmp(&b[2]));
    let k = queries.len();
    let mut ans = vec![false; k];

    let mut union_find = UnionFind::new(n as usize);

    let mut reset = |index: usize, flag: bool| {
        ans[index] = flag;
    };

    let mut j = 0;
    let m = edges.len();
    for i in 0..k {
        while j < m && edges[j][2] < queries[i][2] {
            union_find.union(edges[j][0] as usize, edges[j][1] as usize);
            j += 1;
        }
        let index = queries[i][3] as usize;
        let is_connected = union_find.is_connected(queries[i][0] as usize, queries[i][1] as usize);
        reset(index, is_connected);
    }

    ans
}
