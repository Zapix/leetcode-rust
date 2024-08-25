use std::rc::Rc;
use std::cell::RefCell;

#[allow(dead_code)]
struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut stack= vec![root];
        while let Some(current) = stack.pop() {
            match current {
                Some(current) => {
                    let mut current = current.clone();
                    let mut current_node_ref= current.borrow_mut();
                    if current_node_ref.left.is_none() && current_node_ref.right.is_none() {
                        result.push(current_node_ref.val);
                        continue
                    }
                    let right = current_node_ref.right.clone();
                    let left = current_node_ref.left.clone();
                    current_node_ref.left = None;
                    current_node_ref.right = None;
                    stack.push(Some(current.clone()));
                    stack.push(right);
                    stack.push(left);
                },
                None => {},
            }
        }
        result
    }
}