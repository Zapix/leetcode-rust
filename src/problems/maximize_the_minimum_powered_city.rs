struct Solution;

impl Solution {
    pub fn max_powert(stations: Vec<i32>, r: i32, k: i32) -> i32 {
        let mut cnt = vec![0i64; stations.len() + 1];
        for (i, &power) in stations.iter().enumerate() {
            let left = (i as i32 - r).max(0) as usize;
            let right = (i as i32 + r + 1).min(stations.len() as i32) as usize;
            cnt[left] += power as i64;
            cnt[right] -= power as i64;
        }

        let min_value = *stations.iter().min().unwrap() as i64;
        let max_value = stations.iter().fold(0i64, |acc, &x| acc + x as i64) + k as i64;
        let mut res = 0i64;
        let mut lo = min_value;
        let mut hi = max_value;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if Solution::check(&cnt, mid, k as i64, r as usize) {
                res = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        res as i32
    }

    fn check(cnt: &Vec<i64>, val: i64, k: i64, r: usize) -> bool {
        let mut diff = cnt.clone();
        let mut sum = 0;
        let mut remain = k;
        for i in 0..cnt.len() - 1 {
            sum += diff[i];
            if sum < val {
                let need = val - sum;
                if need > remain {
                    return false;
                }
                remain -= need;
                sum += need;
                let right = (i + 2 * r as usize + 1).min(cnt.len() - 1);
                diff[right] -= need;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let stations = vec![1, 2, 4, 5, 0];
        let r = 1;
        let k = 2;
        assert_eq!(Solution::max_powert(stations, r, k), 5);
    }

    #[test]
    fn test_example_2() {
        let stations = vec![4, 4, 4, 4];
        let r = 0;
        let k = 3;
        assert_eq!(Solution::max_powert(stations, r, k), 4);
    }
}
