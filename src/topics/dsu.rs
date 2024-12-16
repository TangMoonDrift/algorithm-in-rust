//! 并查集专题

pub struct UnionFind {
    pub father: Vec<usize>,
    pub size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            father: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.father[x] != x {
            self.father[x] = self.find(self.father[x]);
        }
        self.father[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let fx = self.find(x);
        let fy = self.find(y);
        if fx != fy {
            if self.size[fx] < self.size[fy] {
                self.father[fx] = fy;
                self.size[fy] += self.size[fx];
            } else {
                self.father[fy] = fx;
                self.size[fx] += self.size[fy];
            }
        }
    }
}

/**
 * 765. 情侣牵手
 * https://leetcode.cn/problems/couples-holding-hands/
 */
pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
    let n = row.len();
    let mut sets = n / 2;
    let mut union_find = UnionFind::new(sets);

    for i in (0..n).step_by(2) {
        if union_find.find((row[i] / 2) as usize) != union_find.find((row[i + 1] / 2) as usize) {
            union_find.union((row[i] / 2) as usize, (row[i + 1] / 2) as usize);
            sets -= 1;
        }
    }

    (n / 2 - sets) as i32
}

/**
 * 839. 相似字符串组
 * https://leetcode.cn/problems/similar-string-groups/
 */
pub fn num_similar_groups(strs: Vec<String>) -> i32 {
    let n = strs.len();
    let m = strs[0].len();
    let mut sets = n;
    let mut father = vec![0; sets];
    build(&mut father, n);

    fn build(father: &mut Vec<usize>, sets: usize) {
        for i in 0..sets {
            father[i] = i;
        }
    }

    fn find(father: &mut Vec<usize>, i: usize) -> usize {
        if father[i] != i {
            father[i] = find(father, father[i]);
        }
        father[i]
    }

    fn union(father: &mut Vec<usize>, x: usize, y: usize) {
        let fx = find(father, x);
        let fy = find(father, y);
        father[fx] = fy;
    }

    fn ok(a: &str, b: &str, k: usize) -> bool {
        let mut diff = 0;
        for (char_in_a, char_in_b) in a.bytes().zip(b.bytes()) {
            if char_in_a != char_in_b {
                diff += 1;
                if diff > 2 {
                    return false;
                }
            }
        }
        true
    }

    for i in 0..n {
        for j in (i + 1)..n {
            if find(&mut father, i) != find(&mut father, j) {
                if ok(&strs[i], &strs[j], m) {
                    union(&mut father, i, j);
                    sets -= 1;
                }
            }
        }
    }

    sets as i32
}

/**
 * 200. 岛屿数量
 * https://leetcode.cn/problems/number-of-islands/
 */
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut father = vec![0; 100001];
    let mut sets = build(&grid, &mut father, n, m);

    fn build(grid: &Vec<Vec<char>>, father: &mut Vec<usize>, n: usize, m: usize) -> usize {
        let mut sets = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '1' {
                    let index = serialize(m, i, j);
                    father[index] = index;
                    sets += 1;
                }
            }
        }

        sets
    }

    fn serialize(cols: usize, i: usize, j: usize) -> usize {
        i * cols + j
    }

    fn find(father: &mut Vec<usize>, i: usize) -> usize {
        if father[i] != i {
            father[i] = find(father, father[i]);
        }
        father[i]
    }

    fn union(father: &mut Vec<usize>, x: usize, y: usize) {
        let fx = find(father, x);
        let fy = find(father, y);
        father[fx] = fy;
    }

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '1' {
                let index_1 = serialize(m, i, j);
                let index_2 = serialize(m, i - 1, j);
                let index_3 = serialize(m, i, j - 1);
                if i > 0 && grid[i - 1][j] == '1' {
                    if find(&mut father, index_1) != find(&mut father, index_2) {
                        union(&mut father, index_1, index_2);
                        sets -= 1;
                    }
                }

                if j > 0 && grid[i][j - 1] == '1' {
                    if find(&mut father, index_1) != find(&mut father, index_3) {
                        union(&mut father, index_1, index_3);
                        sets -= 1;
                    }
                }
            }
        }
    }

    sets as i32
}

mod tests {
    use super::*;
    #[test]
    fn test_min_swaps_couples() {
        assert_eq!(min_swaps_couples(vec![0, 2, 1, 3]), 1);
        assert_eq!(min_swaps_couples(vec![3, 2, 0, 1]), 0);
    }
}
