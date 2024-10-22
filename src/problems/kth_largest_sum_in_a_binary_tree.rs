use std::cell::RefCell;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::rc::Rc;

struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn kth_largest_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut level_map = HashMap::new();
        let mut que = VecDeque::new();
        que.push_back((0, root));

        while let Some((level, node)) = que.pop_front() {
            if node.is_none() {
                continue;
            }
            let val = node.clone().unwrap().borrow().val;
            if let Some(count) = level_map.get_mut(&level) {
                *count += val;
            } else {
                level_map.insert(level, val);
            }
            que.push_back((level + 1, node.clone().unwrap().borrow().left.clone()));
            que.push_back((level + 1, node.clone().unwrap().borrow().right.clone()));
        }
        let mut heap = level_map.values().map(|x| *x).collect::<BinaryHeap<i32>>();
        if heap.len() < k as usize {
            -1
        } else {
            let mut result = 0;
            for _ in 0..k {
                result = heap.pop().unwrap();
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.as_mut().unwrap().borrow_mut().left = left;
        root.as_mut().unwrap().borrow_mut().right = right;
        let k = 1;
        let res = Solution::kth_largest_sum(root, k);
        assert_eq!(res, 5);
    }

    #[test]
    fn test2() {
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(10))));
        let mut left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let left_left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        left.as_mut().unwrap().borrow_mut().left = left_left;
        root.as_mut().unwrap().borrow_mut().left = left;
        let k = 2;
        let res = Solution::kth_largest_sum(root, k);
        assert_eq!(res, 10);
    }
}
