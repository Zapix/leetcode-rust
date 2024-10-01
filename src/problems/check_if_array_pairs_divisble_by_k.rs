use std::collections::HashMap;

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut values: HashMap<i32, usize> = HashMap::new();
        for value in arr {
            let rest = ((value % k) + k) % k;
            let need = (k - rest) % k;
            if values.contains_key(&need) {
                let x = values.get_mut(&need).unwrap();
                *x -= 1;
                if *x == 0 {
                    values.remove(&need);
                }
            } else {
                match values.get_mut(&rest) {
                    Some(x) => *x += 1,
                    None => {
                        values.insert(rest, 1);
                    }
                }
            }
        }

        values.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::can_arrange(vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5),
            true
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 7), true);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 10), false);
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::can_arrange(vec![-1, 1, -2, 2, -3, 3, -4, 4], 3),
            true
        );
    }
}
