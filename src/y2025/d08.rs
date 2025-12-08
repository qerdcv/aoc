use std::collections::HashMap;

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let size = vec![1usize; n];
        for i in 0..n {
            parent.push(i);
        }
        let components = n;

        Self { parent, size, components }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }

        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return false;
        }

        if self.size[ra] < self.size[rb] {
            self.parent[ra] = rb;
            self.size[rb] += self.size[ra];
        } else {
            self.parent[rb] = ra;
            self.size[ra] += self.size[rb];
        }

        self.components -= 1;
        true
    }
}


pub fn p1(input: &str) -> i64 {
    let nodes = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let coords = l.split(",").collect::<Vec<_>>();
            (
                coords[0].parse::<i64>().unwrap(),
                coords[1].parse::<i64>().unwrap(),
                coords[2].parse::<i64>().unwrap(),
            )
        }).collect::<Vec<_>>();
    let n = nodes.len();

    let mut edges = Vec::new();
    for i in 0..n {
        let n1 = nodes[i];
        for j in (i + 1)..n {
            let n2 = nodes[j];

            let (dx, dy, dz) = (n1.0 - n2.0, n1.1 - n2.1, n1.2 - n2.2);
            let dist = dx * dx + dy * dy + dz * dz;

            edges.push((dist, i, j))
        }
    }

    edges.sort_by_key(|e| e.0);


    let mut dsu = DSU::new(n);
    for k in 0..1000 {
        let (_, a, b) = edges[k];
        dsu.union(a, b);
    }

    let mut comp_size = HashMap::new();
    for i in 0..n {
        let r = dsu.find(i);
        *comp_size.entry(r).or_insert(0) += 1;
    }


    let mut sizes: Vec<usize> = comp_size.values().cloned().collect();
    sizes.sort_by(|a, b| b.cmp(a));

    (sizes[0] * sizes[1] * sizes[2]) as i64
}

pub fn p2(input: &str) -> i64 {
    let nodes = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let coords = l.split(",").collect::<Vec<_>>();
            (
                coords[0].parse::<i64>().unwrap(),
                coords[1].parse::<i64>().unwrap(),
                coords[2].parse::<i64>().unwrap(),
            )
        }).collect::<Vec<_>>();
    let n = nodes.len();

    let mut edges = Vec::new();
    for i in 0..n {
        let n1 = nodes[i];
        for j in (i + 1)..n {
            let n2 = nodes[j];

            let (dx, dy, dz) = (n1.0 - n2.0, n1.1 - n2.1, n1.2 - n2.2);
            let dist = dx * dx + dy * dy + dz * dz;

            edges.push((dist, i, j))
        }
    }

    edges.sort_by_key(|e| e.0);


    let mut dsu = DSU::new(n);
    for &(_, a, b) in &edges {
        if dsu.union(a, b) && dsu.components == 1 {
            return nodes[a].0 * nodes[b].0;
        }
    }

    0
}
