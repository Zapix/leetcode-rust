use std::collections::HashMap;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn query_results(_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut balls: HashMap<i32, i32> = HashMap::new();
        let mut colors: HashMap<i32, i32> = HashMap::new();

        queries
            .iter()
            .map(|x| {
                let ball = x[0];
                match balls.get(&ball) {
                    Some(color) => {
                        let entry = colors.entry(*color).or_insert(1);
                        *entry -= 1;
                        if *entry == 0 {
                            colors.remove(color);
                        }
                    }
                    _ => {}
                }
                let color = x[1];
                let entry = colors.entry(color).or_insert(0);
                *entry += 1;
                balls.insert(ball, color);
                colors.len() as i32
            })
            .collect::<Vec<_>>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let limit = 4;
        let queries = vec![vec![1, 4], vec![2, 5], vec![1, 3], vec![3, 4]];
        assert_eq!(Solution::query_results(limit, queries), vec![1, 2, 2, 3]);
    }

    #[test]
    fn test_example_2() {
        let limit = 4;
        let queries = vec![vec![0, 1], vec![1, 2], vec![2, 2], vec![3, 4], vec![4, 5]];
        assert_eq!(Solution::query_results(limit, queries), vec![1, 2, 2, 3, 4]);
    }

    #[test]
    fn test_example_3() {
        let limit = 1000000000;
        let queries = vec![vec![0, 1], vec![1, 2], vec![2, 2], vec![3, 4], vec![4, 5]];
        assert_eq!(Solution::query_results(limit, queries), vec![1, 2, 2, 3, 4]);
    }
}
