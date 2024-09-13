#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sub_queries = vec![];
        let mut current = 0i32;
        for x in arr {
            current ^= x;
            sub_queries.push(current);
        }

        queries
            .iter()
            .map(|x| {
                let i = x[0];
                let j = x[1];
                let st = if i == 0 {
                    0
                } else {
                    sub_queries[(i - 1) as usize]
                };
                let end = sub_queries[j as usize];
                st ^ end
            })
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn leetcode() {
        let arr = vec![1, 3, 4, 8];
        let queries = vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]];
        let result = Solution::xor_queries(arr, queries);
        assert_eq!(vec![2, 7, 14, 8], result);
    }

    #[test]
    fn leetcode_2() {
        let arr = vec![4, 8, 2, 10];
        let queries = vec![vec![2, 3], vec![1, 3], vec![0, 0], vec![0, 3]];
        let result = Solution::xor_queries(arr, queries);
        assert_eq!(vec![8, 0, 4, 4], result);
    }
}
