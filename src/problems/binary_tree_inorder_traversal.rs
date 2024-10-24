use crate::problems::common_struct::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = vec![];
        let mut result: Vec<i32> = vec![];
        let mut current = root;
        while current.is_some() || !stack.is_empty() {
            while let Some(current_node) = current.clone() {
                stack.push(current);
                current = current_node.borrow().left.clone();
            }
            if let Some(Some(rc_node)) = stack.pop() {
                result.push(rc_node.borrow().val);
                current = rc_node.borrow().right.clone();
            }
        }
        return result;
    }
}
