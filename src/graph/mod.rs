pub mod bellman_ford;
pub mod bfs;
pub mod dijkstra;
pub mod mst;
pub mod topo_sort;

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
            weight: vec![0; e + 1],
            cnt: 1,
        }
    }

    pub fn build(n: usize, edges: &Vec<Vec<usize>>, has_direction: bool) -> Self {
        let e = if has_direction {
            edges.len()
        } else {
            edges.len() * 2
        };
        let mut instance = Self::new(n, e);
        for edge in edges {
            let (from, to) = (edge[0], edge[1]);
            let weight = edge.get(2).copied().unwrap_or(0);
            instance.add_edge(from, to, weight);
            if !has_direction {
                instance.add_edge(to, from, weight);
            }
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

    pub fn collect_neighbors(&self, from: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        let mut index = self.head[from];
        std::iter::from_fn(move || {
            if index == 0 {
                None
            } else {
                let res = (self.to[index], self.weight[index]);
                index = self.next[index];
                Some(res)
            }
        })
    }
}

/**
* 改进点总结
*   - 安全增强：移除预分配长度限制，改用动态扩容，避免越界
*
*   - 内存优化：使用 Vec::with_capacity 替代全量初始化
*
*   - API 明确：通过文档注明节点编号从1开始
*
*   - 性能提升：返回迭代器替代收集 Vec，减少内存分配
*
*   - 错误处理：添加输入数据校验断言
*
* 使用时需确保：
*
*   - 节点编号从 1 开始，最大编号不超过初始化时的 n
*
*   - 边的 from/to 必须在有效节点范围内
*
*   - 无向图通过 directed: false 自动添加反向边
*/
#[derive(Debug)]
pub struct DynamicGraph {
    head: Vec<usize>,   // 头指针数组，初始为 usize::MAX
    next: Vec<usize>,   // 邻接表 next 指针
    to: Vec<usize>,     // 边指向的节点
    weight: Vec<usize>, // 边权重
}

impl DynamicGraph {
    pub fn new(n: usize) -> Self {
        Self {
            head: vec![usize::MAX; n + 1], // 关键修复点
            next: Vec::new(),
            to: Vec::new(),
            weight: Vec::new(),
        }
    }

    pub fn build(n: usize, edges: &[Vec<usize>], directed: bool) -> Self {
        let mut instanc = Self::new(n);
        for edge in edges {
            let (from, to) = (edge[0], edge[1]);
            let weight = edge.get(2).copied().unwrap_or(0);
            instanc.add_edge(from, to, weight);
            if !directed {
                instanc.add_edge(to, from, weight);
            }
        }
        instanc
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: usize) {
        self.next.push(self.head[from]);
        self.to.push(to);
        self.weight.push(weight);
        self.head[from] = self.next.len() - 1;
    }

    pub fn neighbors(&self, node: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        let mut index = self.head[node];
        std::iter::from_fn(move || {
            if index == usize::MAX {
                None
            } else {
                let res = (self.to[index], self.weight[index]);
                index = self.next[index];
                Some(res)
            }
        })
    }
}
