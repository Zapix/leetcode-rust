use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::problems::common_struct::TreeNode;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn calc_left_to_right_length(
        node: Option<Rc<RefCell<TreeNode>>>,
        current_length: i32,
        max_length: &mut i32,
        tree_length: &mut HashMap<i32, i32>,
    ) {
        if node.is_none() {
            return;
        }
        let val = node.clone().unwrap().borrow().val;
        tree_length
            .entry(val)
            .and_modify(|height| {
                *height = (*height).max(*max_length);
            })
            .or_insert(*max_length);
        *max_length = (*max_length).max(current_length);
        Self::calc_left_to_right_length(
            node.clone().unwrap().borrow().left.clone(),
            current_length + 1,
            max_length,
            tree_length,
        );
        Self::calc_left_to_right_length(
            node.clone().unwrap().borrow().right.clone(),
            current_length + 1,
            max_length,
            tree_length,
        );
    }

    fn calc_right_to_left_length(
        node: Option<Rc<RefCell<TreeNode>>>,
        current_length: i32,
        max_length: &mut i32,
        tree_length: &mut HashMap<i32, i32>,
    ) {
        if node.is_none() {
            return;
        }
        let val = node.clone().unwrap().borrow().val;
        tree_length
            .entry(val)
            .and_modify(|height| {
                *height = (*height).max(*max_length);
            })
            .or_insert(*max_length);
        *max_length = (*max_length).max(current_length);
        Self::calc_right_to_left_length(
            node.clone().unwrap().borrow().right.clone(),
            current_length + 1,
            max_length,
            tree_length,
        );
        Self::calc_right_to_left_length(
            node.clone().unwrap().borrow().left.clone(),
            current_length + 1,
            max_length,
            tree_length,
        );
    }
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let mut tree_length: HashMap<i32, i32> = HashMap::new();
        let mut max_length = 0;

        Self::calc_left_to_right_length(root.clone(), 0, &mut max_length, &mut tree_length);
        max_length = 0;
        Self::calc_right_to_left_length(root.clone(), 0, &mut max_length, &mut tree_length);

        queries
            .iter()
            .map(|x| *tree_length.get(x).unwrap())
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::problems::common_struct::TreeNode;

    // Helper function to build a binary tree from a level order traversal array
    fn build_tree(level_order: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if level_order.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(level_order[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while i < level_order.len() {
            if let Some(Some(val)) = level_order.get(i) {
                let node = Rc::new(RefCell::new(TreeNode::new(*val)));
                queue.front().unwrap().borrow_mut().left = Some(Rc::clone(&node));
                queue.push_back(node);
            }
            i += 1;

            if let Some(Some(val)) = level_order.get(i) {
                let node = Rc::new(RefCell::new(TreeNode::new(*val)));
                queue.front().unwrap().borrow_mut().right = Some(Rc::clone(&node));
                queue.push_back(node);
            }
            i += 1;

            queue.pop_front();
        }

        Some(root)
    }

    #[test]
    fn test_tree_queries() {
        // Test case 1
        let tree1 = build_tree(vec![
            Some(1),
            Some(3),
            Some(4),
            Some(2),
            None,
            Some(6),
            Some(5),
            None,
            None,
            None,
            None,
            None,
            Some(7),
        ]);
        let queries1 = vec![4];
        let expected1 = vec![2];
        assert_eq!(Solution::tree_queries(tree1, queries1), expected1);
    }

    #[test]
    fn test_tree_queries_2() {
        // Test case 2
        let tree2 = build_tree(vec![
            Some(5),
            Some(8),
            Some(9),
            Some(2),
            Some(1),
            Some(3),
            Some(7),
            Some(4),
            Some(6),
        ]);
        let queries2 = vec![3, 2, 4, 8];
        let expected2 = vec![3, 2, 3, 2];
        assert_eq!(Solution::tree_queries(tree2, queries2), expected2);
    }
}
