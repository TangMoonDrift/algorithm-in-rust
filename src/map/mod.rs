pub mod topo_sort;
use crate::topics::dsu::UnionFind;

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
