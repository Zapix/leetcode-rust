use std::cell::RefCell;
use std::rc::Rc;

use crate::problems::common_struct::TreeNode;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root1.is_none() && root2.is_none() {
            return true;
        }
        if root1.is_none() ^ root2.is_none() {
            return false;
        }
        if root1.clone().unwrap().borrow().val != root2.clone().unwrap().borrow().val {
            return false;
        }
        (Solution::flip_equiv(
            root1.clone().unwrap().borrow().left.clone(),
            root2.clone().unwrap().borrow().left.clone(),
        ) && Solution::flip_equiv(
            root1.clone().unwrap().borrow().right.clone(),
            root2.clone().unwrap().borrow().right.clone(),
        )) || (Solution::flip_equiv(
            root1.clone().unwrap().borrow().left.clone(),
            root2.clone().unwrap().borrow().right.clone(),
        ) && Solution::flip_equiv(
            root1.clone().unwrap().borrow().right.clone(),
            root2.clone().unwrap().borrow().left.clone(),
        ))
    }
}
