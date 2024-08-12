use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(dead_code)]
struct KthLargest {
   k: i32,
   heap: BinaryHeap<Reverse<i32>>,
}

#[allow(dead_code)]
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap = nums
            .iter()
            .map(|x| Reverse(*x))
            .collect::<BinaryHeap<Reverse<i32>>>();
        while heap.len() > k as usize {
            heap.pop();
        }
        Self {
            k,
            heap,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        while self.heap.len() > self.k as usize {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        let mut kth_largest = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth_largest.add(3), 4);
        assert_eq!(kth_largest.add(5), 5);
        assert_eq!(kth_largest.add(10), 5);
        assert_eq!(kth_largest.add(9), 8);
        assert_eq!(kth_largest.add(4), 8);
    }
}