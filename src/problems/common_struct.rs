use std::cell::RefCell;
use std::rc::Rc;

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
            right: None,
        }
    }
}

pub fn build_tree_from_array(arr: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(arr: &[Option<i32>], index: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if index >= arr.len() || arr[index].is_none() {
            return None;
        }
        let node = Rc::new(RefCell::new(TreeNode::new(arr[index].unwrap())));
        node.borrow_mut().left = helper(arr, 2 * index + 1);
        node.borrow_mut().right = helper(arr, 2 * index + 2);
        Some(node)
    }
    helper(arr, 0)
}
