use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

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
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack: Vec<(Option<Rc<RefCell<TreeNode>>>, i32)> = vec![]; 
        stack.push((root.clone(), root.clone().unwrap().borrow().val));

        while !stack.is_empty() {
            let mut next_stack: Vec<(Option<Rc<RefCell<TreeNode>>>, i32)> = vec![];
            let mut value_sum = 0;
            let k = stack.len();
            
            for i in 0..k {
                let (node, parent_node) = &stack[i];
                if let Some(node) = node {
                    value_sum += node.borrow().val;
                    let parent_sum = node.borrow().left.clone().map_or(0, |left| left.borrow().val) + 
                        node.borrow().right.clone().map_or(0, |right| right.borrow().val);

                    next_stack.push((node.borrow().left.clone(), parent_sum));
                    next_stack.push((node.borrow().right.clone(), parent_sum));
                }
            }

            for i in 0..k {
                let (node, parent_sum) = &stack[i];
                if let Some(node) = node {
                    node.borrow_mut().val = value_sum - *parent_sum;
                }
            }
            stack = next_stack;
        }
        
        root
    }
}