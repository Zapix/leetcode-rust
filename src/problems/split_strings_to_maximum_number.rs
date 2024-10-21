use std::collections::HashSet;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        let mut res = 0;
        let mut set = HashSet::new();
        let mut s = s.chars().collect::<Vec<_>>();
        Self::dfs(&mut s, &mut set, &mut res);
        res
    }

    fn dfs(s: &mut Vec<char>, set: &mut HashSet<String>, res: &mut i32) {
        if s.is_empty() {
            *res = std::cmp::max(*res, set.len() as i32);
            return;
        }

        for i in 1..=s.len() {
            let sub = s[..i].iter().collect::<String>();
            if set.contains(&sub) {
                continue;
            }
            set.insert(sub.clone());
            Self::dfs(&mut s[i..].to_vec(), set, res);
            set.remove(&sub);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let s = "ababccc".to_string();
        let res = Solution::max_unique_split(s);
        assert_eq!(res, 5);
    }

    #[test]
    fn test2() {
        let s = "aba".to_string();
        let res = Solution::max_unique_split(s);
        assert_eq!(res, 2);
    }
}
