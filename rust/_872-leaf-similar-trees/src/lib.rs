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
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let v1 = get_leaf_node_values(&root1);
        let v2 = get_leaf_node_values(&root2);

        v1 == v2
    }
}

fn get_leaf_node_values(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    match root {
        Some(r_in_cell) => {
            let r = r_in_cell.borrow();

            if r.left.is_none() && r.right.is_none() {
                vec![r.val]
            } else {
                let left = get_leaf_node_values(&r.left);
                let right = get_leaf_node_values(&r.right);

                left.into_iter().chain(right.into_iter()).collect()
            }
        },
        None => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
