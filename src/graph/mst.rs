//! 最小生成树专题
use crate::topics::dsu::UnionFind;
use std::io::{self, BufRead};

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

fn read_input() -> io::Result<(usize, Vec<Edge>)> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap()?;
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut edges = Vec::with_capacity(m);
    for _ in 0..m {
        let line = lines.next().unwrap()?;
        let mut parts = line.split_whitespace();
        let source: usize = parts.next().unwrap().parse().unwrap();
        let destination: usize = parts.next().unwrap().parse().unwrap();
        let weight: i32 = parts.next().unwrap().parse().unwrap();
        edges.push(Edge::new(source, destination, weight));
    }

    Ok((n, edges))
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
