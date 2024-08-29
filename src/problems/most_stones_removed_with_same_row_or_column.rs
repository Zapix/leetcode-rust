use std::collections::HashSet;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let n = stones.len();
        let mut count = 0;
        let mut visited = HashSet::new();
        for i in 0..n {
            let x = stones[i][0];
            let y = stones[i][1];
            if !visited.contains(&(x, y)) {
                count += 1;
                let mut queue = vec![(x, y)];
                visited.insert((x, y));
                while !queue.is_empty() {
                    let (x, y) = queue.pop().unwrap();
                    for j in 0..n {
                        if stones[j][0] == x || stones[j][1] == y {
                            let nx = stones[j][0];
                            let ny = stones[j][1];
                            if !visited.contains(&(nx, ny)) {
                                queue.push((nx, ny));
                                visited.insert((nx, ny));
                            }
                        }
                    }
                }
            }
        }
        (n as i32) - count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        let stones = vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![2, 2],
        ];
        let result = Solution::remove_stones(stones);
        assert_eq!(5, result);
    }

    #[test]
    fn leetcode_2() {
        let stones = vec![vec![0, 0], vec![0, 2], vec![1, 1], vec![2, 0], vec![2, 2]];
        let result = Solution::remove_stones(stones);
        assert_eq!(3, result);
    }
}
