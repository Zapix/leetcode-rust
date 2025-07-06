use std::collections::HashMap;

struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    count_map: HashMap<i32, i32>,
}

impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let count_map = nums2.iter().fold(HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });
        FindSumPairs {
            nums1,
            nums2,
            count_map,
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        let index = index as usize;
        *self.count_map.entry(self.nums2[index]).or_insert(0) -= 1;
        self.nums2[index] += val;
        *self.count_map.entry(self.nums2[index]).or_insert(0) += 1;
    }

    fn count(&self, tot: i32) -> i32 {
        self.nums1
            .iter()
            .filter_map(|&x| self.count_map.get(&(tot - x)))
            .sum::<i32>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut obj = FindSumPairs::new(vec![1, 1, 2, 2, 2, 3], vec![1, 4, 5, 2, 5, 4]);
        // ["FindSumPairs", "count", "add", "count", "count", "add", "add", "count"]
        // [[[1, 1, 2, 2, 2, 3], [1, 4, 5, 2, 5, 4]], [7], [3, 2], [8], [4], [0, 1], [1, 1], [7]]
        // Output: [null, 8, null, 2, 1, null, null, 11]
        assert_eq!(obj.count(7), 8);
        obj.add(3, 2);
        assert_eq!(obj.count(8), 2);
        assert_eq!(obj.count(4), 1);
        obj.add(0, 1);
        obj.add(1, 1);
        assert_eq!(obj.count(7), 11);
    }
}
