use std::os::unix::raw::off_t;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut res = vec![];
        let k = a + b + c;
        let mut a = a;
        let mut b = b;
        let mut c = c;
        let mut curra = 0;
        let mut currb = 0;
        let mut currc = 0;

        for _ in 0..k {
            if (a >= b && a >= c && curra != 2) || (a > 0 && (currb == 2 || currc == 2)) {
                res.push('a');
                curra += 1;
                currb = 0;
                currc = 0;
                a -= 1;
            } else if (b >= a && b >= c && currb != 2) || (b > 0 && (curra == 2 || currc == 2)) {
                res.push('b');
                currb += 1;
                curra = 0;
                currc = 0;
                b -= 1;
            } else if (c >= a && c >= b && currc != 2) || (c > 0 && (curra == 2 || currb == 2)) {
                res.push('c');
                currc += 1;
                curra = 0;
                currb = 0;
                c -= 1;
            }
        }

        String::from_iter(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = 1;
        let b = 1;
        let c = 7;
        let res = Solution::longest_diverse_string(a, b, c);
        assert_eq!(res, "ccaccbcc");
    }

    #[test]
    fn test2() {
        let a = 2;
        let b = 2;
        let c = 1;
        let res = Solution::longest_diverse_string(a, b, c);
        assert_eq!(res, "ababc");
    }
}
