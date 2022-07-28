use std::cell::RefCell;
use std::rc::Rc;

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

fn dfs(
    node: Option<Rc<RefCell<TreeNode>>>,
    t: Option<Rc<RefCell<TreeNode>>>,
    mut parents: Vec<Rc<RefCell<TreeNode>>>,
) -> Option<Vec<Rc<RefCell<TreeNode>>>> {
    if node.is_none() {
        return None;
    }
    parents.push(node.as_ref().unwrap().clone());

    if node == t {
        return Some(parents.clone());
    }

    if let Some(r) = dfs(
        node.as_ref().unwrap().borrow().left.clone(),
        t.clone(),
        parents.clone(),
    ) {
        return Some(r);
    }

    if let Some(r) = dfs(
        node.as_ref().unwrap().borrow().right.clone(),
        t.clone(),
        parents,
    ) {
        return Some(r);
    }
    None
}

fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let l = dfs(root.clone(), p, vec![]).unwrap();
    let r = dfs(root.clone(), q, vec![]).unwrap();
    println!(
        "l:{:?} ",
        l.iter().map(|a| a.borrow().val).collect::<Vec<i32>>()
    );
    println!(
        "r:{:?} ",
        r.iter().map(|a| a.borrow().val).collect::<Vec<i32>>()
    );

    let mut i = l.len() - 1;
    let mut j = r.len() - 1;
    while i > 0 && j > 0 {
        if l[i] == r[j] {
            return Some(l[i].clone());
        } else if i < j {
            j -= 1;
        } else if j < i {
            i -= 1;
        } else {
            j -= 1;
            i -= 1;
        }
    }
    root
}

#[cfg(test)]
mod tests {
    //use super::*;

    //fn test_lowest_common_ancestor() {
    //    assert_eq!(lowest_common_ancestor(
    //            Rc::new(RefCell::new(TreeNode::new(1, left:None, right: Rc::new(RefCell::new(TreeNode::new(2))))))
    //            ), );
    //}
}
