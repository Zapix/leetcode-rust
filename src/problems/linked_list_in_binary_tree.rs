use std::boxed::Box;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

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

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_sub(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if head.is_none() {
            return true;
        }
        if head.is_some() && root.is_none() {
            return false;
        }

        let head_node = *head.unwrap();
        let root_rfc = root.unwrap();
        let root_node = root_rfc.borrow();

        if head_node.val != root_node.val {
            return false;
        }

        Solution::is_sub(head_node.next.clone(), root_node.left.clone())
            || Solution::is_sub(head_node.next.clone(), root_node.right.clone())
    }

    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = vec![root];

        while let Some(node) = stack.pop() {
            if node.is_none() {
                continue;
            }
            if Solution::is_sub(head.clone(), node.clone()) {
                return true;
            }
            let node = node.unwrap();
            let node_rfc = Rc::try_unwrap(node).unwrap();
            let node = node_rfc.borrow();
            stack.push(node.left.clone());
            stack.push(node.right.clone());
        }
        false
    }
}
