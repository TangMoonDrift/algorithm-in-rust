//! 并查集专题

pub struct UnionFind {
    father: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            father: (0..n).collect(),
            rank: vec![1; n],
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
            if self.rank[fx] < self.rank[fy] {
                self.father[fx] = fy;
                self.rank[fy] += self.rank[fx];
            } else {
                self.father[fy] = fx;
                self.rank[fx] += self.rank[fy];
            }
        }
    }

    pub fn is_connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
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
        let x = row[i] as usize / 2;
        let y = row[i + 1] as usize / 2;
        if union_find.find(x) != union_find.find(y) {
            union_find.union(x, y);
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
    let mut sets = n;
    let mut union_find = UnionFind::new(n);
    fn ok(a: &str, b: &str) -> bool {
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
            if union_find.find(i) != union_find.find(j) {
                if ok(&strs[i], &strs[j]) {
                    union_find.union(i, j);
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
    trait Extra {
        fn set_father_index(&mut self, index: usize);
    }

    impl Extra for UnionFind {
        fn set_father_index(&mut self, index: usize) {
            self.father[index] = index;
        }
    }

    let n = grid.len();
    let m = grid[0].len();
    let mut sets = 0;
    let mut union_find = UnionFind::new(100_001);

    fn serialize(cols: usize, i: usize, j: usize) -> usize {
        i * cols + j
    }

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '1' {
                sets += 1;
                let curr = serialize(m, i, j);
                union_find.set_father_index(curr);
                let top = serialize(m, i - 1, j);
                let left = serialize(m, i, j - 1);
                if i > 0 && grid[i - 1][j] == '1' {
                    if union_find.find(curr) != union_find.find(top) {
                        union_find.union(curr, top);
                        sets -= 1;
                    }
                }

                if j > 0 && grid[i][j - 1] == '1' {
                    if union_find.find(curr) != union_find.find(left) {
                        union_find.union(curr, left);
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

    #[test]
    fn test_num_similar_groups() {
        assert_eq!(
            num_similar_groups(vec![
                "tars".to_owned(),
                "rats".to_owned(),
                "arts".to_owned(),
                "star".to_owned()
            ]),
            2
        );
        assert_eq!(
            num_similar_groups(vec!["omv".to_owned(), "ovm".to_owned()]),
            1
        );
    }

    #[test]
    fn test_num_islands() {
        assert_eq!(
            num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
    }
}
