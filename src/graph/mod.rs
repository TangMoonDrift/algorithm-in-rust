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
            instance.add_edge(edge[0], edge[1], edge.get(2).copied().unwrap_or(0));
            if !has_direction {
                instance.add_edge(edge[1], edge[0], edge.get(2).copied().unwrap_or(0));
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

    pub fn collect_neighbors(&self, node: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        let mut index = self.head[node];
        while index != 0 {
            neighbors.push((self.to[index], self.weight[index]));
            index = self.next[index];
        }
        neighbors
    }
}
