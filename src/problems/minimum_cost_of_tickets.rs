#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut dp = vec![0; 366];
        let days = days
            .into_iter()
            .map(|x| usize::try_from(x).unwrap())
            .collect::<std::collections::HashSet<usize>>();
        for i in 1usize..366 {
            if !days.contains(&i) {
                dp[i] = dp[i - 1];
                continue;
            }

            dp[i] = *[
                dp[i - 1] + costs[0],
                match usize::try_from((i as i32) - 7) {
                    Ok(x) => dp[x] + costs[1],
                    Err(_) => dp[0] + costs[1],
                },
                match usize::try_from((i as i32) - 30) {
                    Ok(x) => dp[x] + costs[2],
                    Err(_) => dp[0] + costs[2],
                }
            ]
            .iter()
            .min()
            .unwrap();
        }

        dp[365]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]), 11);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]), 17);
    }
}