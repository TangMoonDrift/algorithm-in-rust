//! 并查集专题
use std::collections::HashMap;

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

/**
 * 947. 移除最多的同行或同列石头
 * https://leetcode.cn/problems/most-stones-removed-with-same-row-or-column/description/
 */
pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    struct EnhancedUnionFind {
        father: Vec<usize>,
        sets: usize,
    }

    impl EnhancedUnionFind {
        pub fn new(size: usize) -> Self {
            Self {
                father: (0..size).collect(),
                sets: size,
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
                self.father[fx] = fy;
                self.sets -= 1;
            }
        }

        pub fn sets(&self) -> usize {
            self.sets
        }
    }

    let n = stones.len();
    let mut union_find = EnhancedUnionFind::new(n);
    let mut row_first: HashMap<usize, usize> = HashMap::new();
    let mut col_first: HashMap<usize, usize> = HashMap::new();

    for (i, stone) in stones.into_iter().enumerate() {
        let row = stone[0] as usize;
        let col = stone[1] as usize;

        if let Some(&r) = row_first.get(&row) {
            union_find.union(i, r);
        } else {
            row_first.insert(row, i);
        }

        if let Some(&c) = col_first.get(&col) {
            union_find.union(i, c);
        } else {
            col_first.insert(col, i);
        }
    }

    (n - union_find.sets()) as i32
}

/**
 * 2092. 找出知晓秘密的所有专家
 * https://leetcode.cn/problems/find-all-people-with-secret/description/
 */
pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
    struct EnhancedUnionFind {
        father: Vec<usize>,
        know: Vec<bool>,
    }

    impl EnhancedUnionFind {
        pub fn new(size: usize, first: usize) -> Self {
            let mut instance = Self {
                father: (0..size).collect(),
                know: vec![false; size],
            };
            instance.father[first] = 0;
            instance.know[0] = true;
            instance
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
                self.father[fx] = fy;
                self.know[fy] |= self.know[fx];
            }
        }

        pub fn is_known(&mut self, x: usize) -> bool {
            self.know[x]
        }

        pub fn reset_father(&mut self, x: usize) {
            self.father[x] = x;
        }
    }

    let m = meetings.len();
    let mut meetings = meetings.clone();
    meetings.sort_unstable_by(|a, b| a[2].cmp(&b[2]));
    let mut union_find = EnhancedUnionFind::new(n as usize, first_person as usize);
    let mut ans = vec![];

    let mut l = 0;
    while l < m {
        let mut r = l;
        while r + 1 < m && meetings[l][2] == meetings[r + 1][2] {
            r += 1;
        }

        for i in l..=r {
            union_find.union(meetings[i][0] as usize, meetings[i][1] as usize);
        }

        for i in l..=r {
            let x = meetings[i][0] as usize;
            let y = meetings[i][1] as usize;
            let fx = union_find.find(x);
            let fy = union_find.find(y);
            if !union_find.is_known(fx) {
                union_find.reset_father(x);
            }

            if !union_find.is_known(fy) {
                union_find.reset_father(y);
            }
        }
        l = r + 1;
    }

    for i in 0..(n as usize) {
        let fi = union_find.find(i);
        if union_find.is_known(fi) {
            ans.push(i as i32);
        }
    }

    ans
}

/**
 * 2421. 好路径的数目
 * https://leetcode.cn/problems/number-of-good-paths/description/
 */
pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    struct EnhancedUnionFind {
        father: Vec<usize>,
        max_counts: Vec<usize>,
    }

    impl EnhancedUnionFind {
        pub fn new(size: usize) -> Self {
            Self {
                father: (0..size).collect(),
                max_counts: vec![1; size],
            }
        }

        pub fn find(&mut self, x: usize) -> usize {
            if self.father[x] != x {
                self.father[x] = self.find(self.father[x]);
            }
            self.father[x]
        }

        pub fn union(&mut self, x: usize, y: usize, vals: &Vec<i32>) -> usize {
            let fx = self.find(x);
            let fy = self.find(y);
            let mut path = 0;
            if vals[fx] > vals[fy] {
                self.father[fy] = fx;
            } else if vals[fx] < vals[fy] {
                self.father[fx] = fy;
            } else {
                self.father[fx] = fy;
                path = self.max_counts[fx] * self.max_counts[fy];
                self.max_counts[fy] += self.max_counts[fx];
            }
            path
        }
    }
    let n = vals.len();
    let mut edges = edges.clone();
    let mut union_find = EnhancedUnionFind::new(n);
    let mut ans = n;

    edges.sort_unstable_by(|a, b| {
        let pos_a_start = a[0] as usize;
        let pos_a_end = a[1] as usize;
        let pos_b_start = b[0] as usize;
        let pos_b_end = b[1] as usize;

        let max_a = vals[pos_a_start].max(vals[pos_a_end]);
        let max_b = vals[pos_b_start].max(vals[pos_b_end]);

        max_a.cmp(&max_b)
    });

    for edge in edges.iter() {
        let pos_start = edge[0] as usize;
        let pos_end = edge[1] as usize;
        ans += union_find.union(pos_start, pos_end, &vals);
    }

    ans as i32
}

#[cfg(test)]
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
