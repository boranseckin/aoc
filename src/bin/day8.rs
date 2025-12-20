fn main() {
    let input = include_str!("../../input/day8.txt");

    dbg!(part1(input));
    dbg!(part2(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

impl Coordinate {
    fn distance(&self, other: &Self) -> u64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2))
            .try_into()
            .unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
struct Connection {
    u: usize,
    v: usize,
    dist: u64,
}

#[derive(Debug, Clone)]
struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    fn new(count: usize) -> Self {
        let mut new = Self {
            parent: Vec::with_capacity(count),
            size: vec![1; count],
        };

        for i in 0..count {
            new.parent.push(i);
        }

        new
    }

    /// Finds the root of the set containing element i.
    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            // path compression
            self.parent[i] = self.find(self.parent[i]);
            self.parent[i]
        }
    }

    /// Merges two sets.
    /// Returns true if they were merged, false if they were already in the same set.
    fn unite(&mut self, i: usize, j: usize) -> bool {
        let i_root = self.find(i);
        let j_root = self.find(j);

        if i_root == j_root {
            return false;
        }

        // union by size: attach smaller to larger
        if self.size[i_root] < self.size[j_root] {
            self.parent[i_root] = j_root;
            self.size[j_root] += self.size[i_root]
        } else {
            self.parent[j_root] = i_root;
            self.size[i_root] += self.size[j_root]
        }

        true
    }
}

fn part1(input: &str) -> u64 {
    let coords: Vec<_> = input
        .lines()
        .map(|line| {
            let mut line = line.split(',');
            Coordinate {
                x: line.next().unwrap().parse().unwrap(),
                y: line.next().unwrap().parse().unwrap(),
                z: line.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let mut conns = vec![];

    for u in 0..coords.len() {
        // start from index u + 1 to avoid duplicate
        for v in (u + 1)..coords.len() {
            if coords[u] == coords[v] {
                continue;
            }

            let dist = coords[u].distance(&coords[v]);
            conns.push(Connection { u, v, dist });
        }
    }

    conns.sort_unstable_by_key(|c| c.dist);

    let mut circuits = DisjointSet::new(conns.len());

    let mut conns = conns.iter();

    for _ in 0..1000 {
        let conn = conns.next().unwrap();
        circuits.unite(conn.u, conn.v);
    }

    circuits.size.sort_by(|a, b| b.cmp(a));
    circuits
        .size
        .iter()
        .take(3)
        .fold(1, |acc, c| acc * *c as u64)
}

fn part2(input: &str) -> u64 {
    let coords: Vec<_> = input
        .lines()
        .map(|line| {
            let mut line = line.split(',');
            Coordinate {
                x: line.next().unwrap().parse().unwrap(),
                y: line.next().unwrap().parse().unwrap(),
                z: line.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let mut conns = vec![];

    for u in 0..coords.len() {
        // start from index u + 1 to avoid duplicate
        for v in (u + 1)..coords.len() {
            if coords[u] == coords[v] {
                continue;
            }

            let dist = coords[u].distance(&coords[v]);
            conns.push(Connection { u, v, dist });
        }
    }

    conns.sort_unstable_by_key(|c| c.dist);

    let mut circuits = DisjointSet::new(conns.len());
    let mut n_circuits = coords.len();

    let mut conns = conns.iter();

    let mut last = None;

    while n_circuits > 1
        && let Some(conn) = conns.next()
    {
        if circuits.unite(conn.u, conn.v) {
            n_circuits -= 1;

            if n_circuits == 1 {
                last = Some(conn);
            }
        }
    }

    let last = last.unwrap();
    (coords[last.u].x * coords[last.v].x).try_into().unwrap()
}
