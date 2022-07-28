#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut list = vec![];

    fn post_order(node: Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<Rc<RefCell<TreeNode>>>) {
        if let Some(n) = node {
            list.push(n.clone());

            post_order(n.borrow_mut().left.take(), list);
            post_order(n.borrow_mut().right.take(), list);
        }
    }

    post_order(root.clone(), &mut list);

    for i in 1..list.len() {
        let mut prior = list[i - 1].clone();
        let mut current = list[i].clone();

        prior.borrow_mut().left.take();
        prior.borrow_mut().right = Some(current);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_flatten() {
        assert_eq!(flatten(None), vec![]);
    }
}
