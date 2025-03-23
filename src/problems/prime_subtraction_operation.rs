#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn primes() -> Vec<i32> {
        let mut primes = [true; 1001];
        primes[0] = false;
        primes[1] = false;
        for i in 2..1001 {
            if primes[i] {
                for j in (i..1001).step_by(i).skip(1) {
                    primes[j] = false
                }
            }
        }

        primes
            .iter()
            .enumerate()
            .filter_map(|x| if *x.1 { Some(x.0 as i32) } else { None })
            .collect::<Vec<_>>()
    }
    pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
        let primes = Solution::primes();
        let mut prev = 0;
        for val in nums {
            let mut val = val;
            if val <= prev {
                return false;
            }
            let i = match primes.binary_search(&val) {
                Ok(x) => x,
                Err(x) => x,
            };
            for j in (0..(i)).rev() {
                if val - primes[j] > prev {
                    val -= primes[j];
                    break;
                }
            }
            prev = val;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::prime_sub_operation(vec![4, 9, 6, 10]), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::prime_sub_operation(vec![6, 8, 11, 12]), true);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::prime_sub_operation(vec![5, 8, 3]), false);
    }
}
