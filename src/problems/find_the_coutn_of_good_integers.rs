use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    fn generate_palindromes(n: i32) -> Box<dyn Iterator<Item = i64>> {
        if n == 1 {
            return Box::new(1..10);
        }
        if n % 2 == 0 {
            let n = (n / 2) as u32 - 1;
            return Box::new((10i64.pow(n)..10i64.pow(n + 1)).map(|mut x| {
                let mut val = x.clone();
                while x > 0 {
                    val = val * 10 + x % 10;
                    x /= 10;
                }
                val
            }));
        }

        let n = (n / 2) as u32 - 1;
        Box::new(
            (0i64..10)
                .map(move |m| {
                    (10i64.pow(n)..10i64.pow(n + 1)).map(move |mut x| {
                        let mut val = x.clone() * 10 + m;
                        while x > 0 {
                            val = val * 10 + x % 10;
                            x /= 10;
                        }
                        val
                    })
                })
                .flatten(),
        )
    }

    pub fn count_digits(number: i64) -> [usize; 10] {
        let mut number = number;
        let mut digits = [0; 10];
        while number > 0 {
            digits[(number % 10) as usize] += 1;
            number /= 10;
        }
        digits
    }

    pub fn count_variants(digits: [usize; 10], cache: &mut HashMap<[usize; 10], usize>) -> usize {
        if !cache.contains_key(&digits) {
            let mut sum = 0;
            for i in 0..10 {
                if digits[i] > 0 {
                    let mut next_digits = digits.clone();
                    next_digits[i] -= 1;
                    sum += Solution::count_variants(next_digits, cache);
                }
            }
            cache.insert(digits, sum);
        }
        *cache.get(&digits).unwrap()
    }

    pub fn count_variants_no_lead_zero(
        digits: [usize; 10],
        cache: &mut HashMap<[usize; 10], usize>,
    ) -> usize {
        let mut sum = 0;
        for i in 1..10 {
            if digits[i] > 0 {
                let mut next_digits = digits.clone();
                next_digits[i] -= 1;
                sum += Solution::count_variants(next_digits, cache);
            }
        }
        sum
    }

    pub fn count_good_integers(n: i32, k: i32) -> i32 {
        let k = k as i64;
        let result: HashSet<[usize; 10]> = HashSet::from_iter(
            Self::generate_palindromes(n)
                .filter(|x| *x % k == 0)
                .map(|x| Solution::count_digits(x)),
        );

        let mut cache: HashMap<[usize; 10], usize> = HashMap::new();
        cache.insert([0; 10], 1);
        let mut sum = 0;
        for digits in result {
            sum += Solution::count_variants_no_lead_zero(digits, &mut cache);
        }
        sum as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 3;
        let k = 5;
        let result = Solution::count_good_integers(n, k);
        assert_eq!(result, 27);
    }

    #[test]
    fn test_example_2() {
        let n = 1;
        let k = 4;
        let result = Solution::count_good_integers(n, k);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_example_3() {
        let n = 5;
        let k = 6;
        let result = Solution::count_good_integers(n, k);
        assert_eq!(result, 2468);
    }
}
