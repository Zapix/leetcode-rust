use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::problems::common_struct::TreeNode;
struct Solution;

impl Solution {
    fn find_deepest_count(root: Option<Rc<RefCell<TreeNode>>>) -> (usize, usize) {
        let mut max_deep = 0;
        let mut max_count = 0;

        let mut stack = vec![];
        stack.push((root, 0));

        while let Some((node, deep)) = stack.pop() {
            if let Some(n) = node.clone() {
                let n_ref = n.borrow();
                if deep > max_deep {
                    max_deep = deep;
                    max_count = 0;
                }
                if deep == max_deep {
                    max_count += 1;
                }
                stack.push((n_ref.left.clone(), deep + 1));
                stack.push((n_ref.right.clone(), deep + 1));
            }
        }
        (max_deep, max_count)
    }

    pub fn find_lca(
        root: Option<Rc<RefCell<TreeNode>>>,
        max_deep: usize,
        max_count: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut visited = HashSet::new();
        let mut results = HashMap::new();
        let mut lca = None;
        let mut stack = vec![];
        stack.push((root, 0));

        while let Some((node, deep)) = stack.pop() {
            if let Some(n) = node.clone() {
                let n_ref = n.borrow();
                if deep == max_deep && max_count == 1 {
                    lca = node.clone();
                    break;
                }
                if deep == max_deep {
                    let node_val = node?.borrow().val;
                    results.insert(node_val, 1);
                    continue;
                }
                let node_val = n_ref.val;
                let node_left = n_ref.left.clone();
                let node_right = n_ref.right.clone();
                if visited.contains(&node_val) {
                    let left_count = match node_left {
                        Some(left_node) => *results.get(&left_node.borrow().val).unwrap_or(&0),
                        None => 0,
                    };
                    let right_count = match node_right {
                        Some(left_node) => *results.get(&left_node.borrow().val).unwrap_or(&0),
                        None => 0,
                    };
                    if left_count + right_count == max_count {
                        lca = node.clone();
                        break;
                    } else {
                        results.insert(node_val, left_count + right_count);
                    }
                } else {
                    visited.insert(node_val);
                    stack.push((node.clone(), deep));
                    stack.push((node_left.clone(), deep + 1));
                    stack.push((node_right.clone(), deep + 1));
                }
            }
        }

        lca
    }

    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (deep, count) = Solution::find_deepest_count(root.clone());
        Solution::find_lca(root.clone(), deep, count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::problems::common_struct::build_tree_from_array;

    fn compare_trees(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (t1, t2) {
            (Some(n1), Some(n2)) => {
                let n1 = n1.borrow();
                let n2 = n2.borrow();
                n1.val == n2.val
                    && compare_trees(n1.left.clone(), n2.left.clone())
                    && compare_trees(n1.right.clone(), n2.right.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }

    #[test]
    fn test_example_1() {
        let root = build_tree_from_array(&[
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let expected = build_tree_from_array(&[Some(2), Some(7), Some(4)]);
        let result = Solution::lca_deepest_leaves(root);
        assert!(compare_trees(result, expected));
    }

    #[test]
    fn test_example_2() {
        let root = build_tree_from_array(&[Some(1)]);
        let expected = build_tree_from_array(&[Some(1)]);
        let result = Solution::lca_deepest_leaves(root);
        assert!(compare_trees(result, expected));
    }

    #[test]
    fn test_example_3() {
        let root = build_tree_from_array(&[Some(0), Some(1), Some(3), None, Some(2)]);
        let expected = build_tree_from_array(&[Some(2)]);
        let result = Solution::lca_deepest_leaves(root);
        assert!(compare_trees(result, expected));
    }

    #[test]
    fn test_example_4() {
        let root = build_tree_from_array(&[
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            Some(7),
            None,
            None,
            Some(4),
            None,
            None,
            Some(9),
            Some(10),
        ]);
        let expected = build_tree_from_array(&[
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            Some(7),
            None,
            None,
            Some(4),
            None,
            None,
            Some(9),
            Some(10),
        ]);
        let result = Solution::lca_deepest_leaves(root);
        assert!(compare_trees(result, expected));
    }
}
