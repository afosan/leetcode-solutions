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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (_, _, res) = minmax(&root);

        res
    }
}

fn minmax(root: &Option<Rc<RefCell<TreeNode>>>) -> (Option<i32>, Option<i32>, i32) {
    if let Some(sr) = root {
        let r = sr.borrow();

        let (lmin, lmax, lres) = minmax(&r.left);
        let (rmin, rmax, rres) = minmax(&r.right);

        let minn = vec![lmin, rmin, Some(r.val)].iter().filter(|v| v.is_some()).map(|v| v.unwrap()).min();
        let maxx = vec![lmax, rmax, Some(r.val)].iter().filter(|v| v.is_some()).map(|v| v.unwrap()).max();
        let ress = vec![lmin, lmax, rmin, rmax].iter().filter(|v| v.is_some()).map(|v| (v.unwrap() - r.val).abs()).max().unwrap_or(0);
        let res = ress.max(lres).max(rres);

        (minn, maxx, res)
    } else {
        (None, None, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
