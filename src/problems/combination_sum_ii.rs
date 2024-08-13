use std::collections::HashSet;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut current: Vec<i32> = Vec::new();
        let mut candidates = candidates;
        candidates.sort();
        Solution::backtrack(&candidates, target, 0, &mut current, &mut result);
        result
    }

    fn backtrack(candidates: &Vec<i32>, target: i32, start: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(current.clone());
            return;
        }
        let mut visited: HashSet<i32> = HashSet::new();
        for i in start..candidates.len() {
            if visited.contains(&candidates[i]) {
                continue;
            }
            if target < candidates[i] {
                break;
            }
            visited.insert(candidates[i]);
            current.push(candidates[i]);
            Solution::backtrack(candidates, target - candidates[i], i + 1, current, result);
            current.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::combination_sum2(
                vec![10,1,2,7,6,1,5],
                8
            ),
            vec![
                vec![1,1,6],
                vec![1,2,5],
                vec![1,7],
                vec![2,6],
            ]
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::combination_sum2(
                vec![2,5,2,1,2],
                5
            ),
            vec![
                vec![1,2,2],
                vec![5],
            ]
        );
    }
}