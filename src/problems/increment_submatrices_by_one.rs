use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut diff_map = HashMap::new();

        for query in queries {
            let r1 = query[0] as usize;
            let c1 = query[1] as usize;
            let r2 = query[2] as usize;
            let c2 = query[3] as usize;
            for i in r1..=r2 {
                *diff_map.entry((i, c1)).or_insert(0) += 1;
                *diff_map.entry((i, (c2 + 1))).or_insert(0) -= 1;
            }
        }

        let mut result = vec![vec![0; n]; n];
        for i in 0..n {
            let mut value = 0;
            for j in 0..n {
                value += *diff_map.get(&(i, j)).unwrap_or(&0);
                result[i][j] = value;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_range_add_queries_1() {
        let n = 3;
        let queries = vec![vec![1, 1, 2, 2], vec![0, 0, 1, 1]];

        let expected = vec![vec![1, 1, 0], vec![1, 2, 1], vec![0, 1, 1]];

        assert_eq!(Solution::range_add_queries(n, queries), expected);
    }

    #[test]
    fn test_range_add_queries_2() {
        let n = 2;
        let queries = vec![vec![0, 0, 1, 1]];
        let expected = vec![vec![1, 1], vec![1, 1]];

        assert_eq!(Solution::range_add_queries(n, queries), expected);
    }
}
