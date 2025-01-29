
struct DSU {
    n: usize,
    sizes: Vec<usize>,
    reps: Vec<usize>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        let mut sizes = vec![1; n];
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
        true
    }
}



#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut dsu = DSU::new(n + 1);
        for edge in edges.iter() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            if !dsu.do_union(u, v) {
                return vec![u as i32, v as i32];
            }
        }
        vec![0, 0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_redundant_connection_case1() {
        let edges = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
        assert_eq!(Solution::find_redundant_connection(edges), vec![2, 3]);
    }

    #[test]
    fn test_find_redundant_connection_case2() {
        let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]];
        assert_eq!(Solution::find_redundant_connection(edges), vec![1, 4]);
    }
}
