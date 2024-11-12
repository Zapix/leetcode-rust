use std::collections::HashMap;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut items = items
            .iter()
            .fold(HashMap::new(), |mut acc, x| {
                let (price, beauty) = (x[0], x[1]);
                acc.entry(price)
                    .and_modify(|x: &mut i32| *x = (*x).max(beauty))
                    .or_insert(beauty);
                acc
            })
            .into_iter()
            .collect::<Vec<_>>();
        items.sort_by_key(|x| x.0);

        let mut costs = HashMap::new();
        costs.insert(0, 0);
        let mut max_beauty = 0;
        for (price, beauty) in items {
            if beauty > max_beauty {
                costs.insert(price, beauty);
                max_beauty = beauty;
            }
        }

        let mut prices = costs.keys().map(|x| *x).collect::<Vec<_>>();
        prices.sort();

        queries
            .iter()
            .map(|x| {
                let idx = prices.binary_search(x).unwrap_or_else(|x| x - 1);
                *costs.get(&prices[idx]).unwrap_or(&0)
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let items = vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]];
        let queries = vec![1, 2, 3, 4, 5, 6];
        let expected = vec![2, 4, 5, 5, 6, 6];
        assert_eq!(Solution::maximum_beauty(items, queries), expected);
    }

    #[test]
    fn test_example_2() {
        let items = vec![vec![1, 2], vec![1, 2], vec![1, 3], vec![1, 4]];
        let queries = vec![1];
        let expected = vec![4];
        assert_eq!(Solution::maximum_beauty(items, queries), expected);
    }

    #[test]
    fn test_example_3() {
        let items = vec![vec![10, 1000]];
        let queries = vec![5];
        let expected = vec![0];
        assert_eq!(Solution::maximum_beauty(items, queries), expected);
    }
}
