use std::ops;
use std::fmt;

fn gcd(a: i32, b: i32) -> i32 {
    let a = a.abs();
    let b = b.abs();
    let mut x;
    let mut y;
    match (a, b) {
        (a, b) if b > a => {
            x = b;
            y = a;
        },
        _ => {
            x = a;
            y = b;
        }
    }

    let mut t;
    while y != 0 {
        t = y;
        y = x % y;
        x = t;
    }
    x
}

struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    pub fn from(str: String) -> Self {
        let mut multiply_negative= 1;
        let mut numerator = 0;
        let mut denominator = 0;
        let mut current: &mut i32 = &mut numerator;
        for ch in str.chars() {
            if ch == '-' {
                multiply_negative = -1;
                continue
            }
            if ch == '/' {
                current = &mut denominator;
                continue
            }
            *current = *current * 10 + (ch as i32 - '0' as i32);

        }

        if numerator == 0 {
            return Self {
                numerator: 0,
                denominator: 1,
            }
        }

        numerator *= multiply_negative;
        let val = gcd (numerator, denominator);
        numerator /= val;
        denominator /= val;
        return Self {
            numerator,
            denominator
        }
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.numerator == 0 {
            return write!(f, "0/1");
        }
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl ops::Add for Fraction {
    type Output = Fraction;
    fn add(self, rhs: Self) -> Self::Output {
        let mut numerator = self.numerator * rhs.denominator + rhs.numerator * self.denominator;
        let mut denominator = self.denominator * rhs.denominator;
        if numerator == 0 {
            Self::Output { numerator: 0, denominator: 1 }
        } else {
            let val = gcd (numerator, denominator);
            numerator /= val;
            denominator /= val;
            Self::Output { numerator, denominator }
        }
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let mut result = Fraction::from("0/1".to_string());
        let mut current_vec : Vec<char> = vec![];
        for ch in expression.chars() {
            match ch {
                '-' => {
                    if !current_vec.is_empty() {
                        result = result + Fraction::from(String::from_iter(current_vec));
                    }
                    current_vec = vec![];
                    current_vec.push(ch);
                },
                '+' => {
                    result = result + Fraction::from(String::from_iter(current_vec));
                    current_vec = vec![];
                },
                _ => {
                    current_vec.push(ch)
                }
            }

        }
        result = result + Fraction::from(String::from_iter(current_vec));

        format!("{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::fraction_addition("1/3-1/2".to_string()),
            "-1/6"
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::fraction_addition("-1/2+1/2+1/3".to_string()),
            "1/3"
        );
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::fraction_addition("5/3+1/3".to_string()),
            "2/1"
        );
    }
}