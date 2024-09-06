use std::boxed::Box;
use std::collections::HashSet;
#[allow(dead_code)]
struct Solution;

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let exclude_nums: HashSet<i32> = HashSet::from_iter(nums);
        let mut dummy = ListNode::new(-1);
        dummy.next = head;
        let mut curr = &mut dummy;
        while let Some(next_box) = curr.next.as_mut() {
            if exclude_nums.contains(&next_box.val) {
                curr.next = next_box.next.take();
            } else {
                curr = curr.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}
