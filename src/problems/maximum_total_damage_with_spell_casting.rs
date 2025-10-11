use std::collections::{HashMap, VecDeque};

struct Solution;

impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let spells_map = power.iter().fold(HashMap::new(), |mut acc, &p| {
            *acc.entry(p as i64).or_insert(0i64) += 1;
            acc
        });
        let mut spells = spells_map.iter().collect::<Vec<_>>();

        spells.sort_by(|a, b| a.0.cmp(b.0));

        let mut variants = VecDeque::new();

        for (&spell, &count) in spells {
            if variants.is_empty() {
                variants.push_back((spell, count * spell));
                continue;
            }
            let max_spell = variants
                .iter()
                .filter_map(|&a| if a.0 < spell - 2 { Some(a.1) } else { None })
                .max()
                .unwrap_or(0);

            variants.push_back((spell, max_spell + count * spell));
            while variants.len() > 5 {
                variants.pop_front();
            }
        }
        variants.iter().map(|&a| a.1).max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_total_damage() {
        let power = vec![1, 1, 3, 4];
        assert_eq!(Solution::maximum_total_damage(power), 6);
    }
    #[test]
    fn test_maximum_total_damage_1() {
        let power = vec![7, 1, 6, 6];
        assert_eq!(Solution::maximum_total_damage(power), 13);
    }
}
