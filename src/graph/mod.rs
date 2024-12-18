pub mod topo_sort;
use crate::topics::dsu::UnionFind;

pub struct Graph {
    head: Vec<usize>,
    next: Vec<usize>,
    to: Vec<usize>,
    weight: Vec<usize>,
    cnt: usize,
}

impl Graph {
    pub fn new(n: usize, e: usize) -> Self {
        Self {
            head: vec![0; n + 1],
            next: vec![0; e + 1],
            to: vec![0; e + 1],
            weight: vec![0; e * 2 + 1],
            cnt: 1,
        }
    }

    pub fn build(n: usize, edges: &Vec<Vec<usize>>) -> Self {
        let e = edges.len();
        let mut instance = Self::new(n, e);
        for edge in edges {
            instance.add_edge(edge[0], edge[1], edge[2]);
            instance.add_edge(edge[1], edge[0], edge[2]);
        }
        instance
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: usize) {
        let index = self.cnt;
        self.to[index] = to;
        self.weight[index] = weight;
        self.next[index] = self.head[from];
        self.head[from] = index;
        self.cnt += 1;
    }
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
        let is_same_set =
            union_find.find(queries[i][0] as usize) == union_find.find(queries[i][1] as usize);
        reset(index, is_same_set);
    }

    ans
}
