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

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        range_sum_bst_helper(&root, low, high)
    }
}

fn range_sum_bst_helper(root: &Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    if let Some(val) = root {
        let r = val.borrow();
        if r.val < low {
            return range_sum_bst_helper(&r.right, low, high);
        } else if r.val > high {
            return range_sum_bst_helper(&r.left, low, high);
        } else {
            return r.val + range_sum_bst_helper(&r.right, low, high) + range_sum_bst_helper(&r.left, low, high);
        }
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
