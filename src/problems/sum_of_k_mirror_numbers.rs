struct Solution;

struct MirrorNumber {
    is_odd: bool,
    length: usize,
    num: i64,
    base: i64,
}

impl MirrorNumber {
    fn new(base: i64) -> Self {
        MirrorNumber {
            length: 0,
            num: 0,
            is_odd: false,
            base,
        }
    }
}

impl Iterator for MirrorNumber {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            self.length = 1;
            self.num = 1;
            self.is_odd = true;
        } else {
            self.num += 1;
            if self.num >= self.base.pow((self.length as u32)) {
                if !self.is_odd {
                    self.length += 1;
                    self.is_odd = true;
                } else {
                    self.is_odd = false;
                }
                self.num = self.base.pow((self.length as u32) - 1);
            }
        }

        let mut mirrored_num = self.num;
        let mut temp_num = self.num;
        if self.is_odd {
            temp_num /= self.base;
        }

        for _ in 0..(self.length - self.is_odd as usize) {
            mirrored_num = mirrored_num * self.base + (temp_num % self.base);
            temp_num /= self.base;
        }

        Some(mirrored_num)
    }
}

impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        let k = k as i64;
        let mut count = 0;
        let mut sum = 0;
        let mut iter = MirrorNumber::new(k);
        while count < n {
            let num = iter.next().unwrap();
            if Solution::is_k_mirror(num, k) && Solution::is_k_mirror(num, 10) {
                sum += num;
                count += 1;
            }
        }

        sum
    }

    fn is_k_mirror(num: i64, k: i64) -> bool {
        let str_num = Solution::to_base(num, k);
        str_num == str_num.chars().rev().collect::<String>()
    }

    fn to_base(num: i64, base: i64) -> String {
        let mut n = num;
        let mut result = String::new();

        while n > 0 {
            result.push_str(&(n % base).to_string());
            n /= base;
        }

        result.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::k_mirror(2, 5), 25);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::k_mirror(3, 7), 499);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::k_mirror(7, 17), 20379000);
    }
}
