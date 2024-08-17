#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let m = points[0].len();
        let mut result: Vec<i64> = (0..m).map(|_| 0i64).collect();
        for row in points {
            let mut l2r= result.clone();
            for i in 1..m {
                l2r[i] = l2r[i].max(l2r[i - 1] - 1)
            }

            let mut r2l= result.clone();
            for i in (0..(m-1)).rev() {
                r2l[i] = r2l[i].max(r2l[i + 1] - 1)
            }
            for i in 0..m {
                result[i] = l2r[i].max(r2l[i]) as i64 + row[i] as i64;
            }
        }
        *result.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::max_points(
                vec![
                    vec![1, 2, 3],
                    vec![1, 5, 1],
                    vec![3, 1, 1]
                ]
            ),
            9
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::max_points(
                vec![
                    vec![1, 5],
                    vec![2, 3],
                    vec![4, 2]
                ]
            ),
            11
        );
    }
}