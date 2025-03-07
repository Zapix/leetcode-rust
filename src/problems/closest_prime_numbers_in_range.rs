const ARR_SIZE: usize = 1_000_001;
#[allow(long_running_const_eval)]
const PRIME_MAP: [bool; ARR_SIZE] = {
    let mut prime_map = [true; ARR_SIZE];
    prime_map[0] = false;
    prime_map[1] = false;
    let mut i = 2;
    while i < ARR_SIZE {
        if prime_map[i] {
            let mut j = i * 2;
            while j < ARR_SIZE {
                prime_map[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    prime_map
};

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let primes = (left as usize..right as usize + 1)
            .filter(|x| PRIME_MAP[*x])
            .collect::<Vec<usize>>();
        primes
            .iter()
            .zip(primes.iter().skip(1))
            .min_by_key(|x| x.1 - x.0)
            .map_or(vec![-1, -1], |x| vec![*x.0 as i32, *x.1 as i32])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let left = 10;
        let right = 19;
        assert_eq!(Solution::closest_primes(left, right), vec![11, 13]);
    }

    #[test]
    fn test_example_2() {
        let left = 4;
        let right = 6;
        assert_eq!(Solution::closest_primes(left, right), vec![-1, -1]);
    }

    #[test]
    fn test_example_3() {
        let left = 19;
        let right = 31;
        assert_eq!(Solution::closest_primes(left, right), vec![29, 31]);
    }
}
