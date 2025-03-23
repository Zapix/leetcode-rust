struct DSU {
    n: usize,
    sizes: Vec<usize>,
    reps: Vec<usize>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        let sizes = vec![1; n];
        let reps = (0..n).collect();
        Self { n, sizes, reps }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.reps[x] != x {
            self.reps[x] = self.find(self.reps[x]);
        }
        self.reps[x]
    }

    pub fn do_union(&mut self, x: usize, y: usize) -> bool {
        let x_rep = self.find(x);
        let y_rep = self.find(y);
        if x_rep == y_rep {
            return false;
        }
        if self.sizes[x_rep] < self.sizes[y_rep] {
            self.reps[x_rep] = y_rep;
            self.sizes[y_rep] += self.sizes[x_rep];
        } else {
            self.reps[y_rep] = x_rep;
            self.sizes[x_rep] += self.sizes[y_rep];
        }
        self.n -= 1;
        true
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut dsu = DSU::new(n);
        for edge in edges.iter() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            if !dsu.do_union(u, v) {
                return false;
            }
        }
        dsu.n == 1
    }
}