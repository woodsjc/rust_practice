// NOT FINISHED
use std::cell::RefCell;
use std::rc::Rc;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
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

fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, prior_covered: bool, current_covered: bool) -> i32 {
    if node.is_none() {
        return 0;
    }

    let node = node.unwrap().borrow();
    let l = &node.left;
    let r = &node.right;

    if !prior_covered {
        if l.is_some() && r.is_some() {
            return 1 + dfs(l.as_ref(), true, true) + dfs(r.as_ref(), true, true);
        } else if l.is_none() && r.is_none() {
            return 1;
        } else if l.is_some() {
            return 1 + dfs(l.as_ref(), true, true);
        } else {
            return 1 + dfs(r.as_ref(), true, true);
        }
    }

    if l.is_some() && r.is_some() {
        return i32::min(
            1 + dfs(l.as_ref(), true, true) + dfs(r.as_ref(), true, true),
            dfs(l.as_ref(), current_covered, false) + dfs(r.as_ref(), current_covered, false),
        );
    } else if l.is_none() && r.is_none() {
        if current_covered {
            return 0;
        }
        return 1;
    } else if l.is_some() {
        return i32::min(
            dfs(l.as_ref(), current_covered, false),
            1 + dfs(l.as_ref(), true, true),
        );
    } else {
        return i32::min(
            dfs(r.as_ref(), current_covered, false),
            1 + dfs(r.as_ref(), true, true),
        );
    }
}

fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    i32::min(
        dfs(root.as_ref(), true, false),
        1 + dfs(root.as_ref(), true, true),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_camera_cover() {
        //[0,0,null,0,0]
        assert_eq!(
            min_camera_cover(Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: None,
            })))),
            1
        );

        //[0,0,null,0,null,0,null,null,0]
        assert_eq!(
            min_camera_cover(Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 0,
                                left: None,
                                right: None,
                            }))),
                        }))),
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            })))),
            2
        );

        //[0,0,0,null,null,null,0]
        assert_eq!(
            min_camera_cover(Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None,
                    }))),
                }))),
            })))),
            2
        );

        //[0,0,0,null,0,null,0]
        assert_eq!(
            min_camera_cover(Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None,
                    }))),
                }))),
            })))),
            2
        );

        //[0,0,null,null,0,0,null,null,0,0]
        //                          0
        //              c                       null
        //      null           0
        //                 0      null
        //            null    c
        //                  0
        assert_eq!(
            min_camera_cover(Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 0,
                                left: Some(Rc::new(RefCell::new(TreeNode {
                                    val: 0,
                                    left: None,
                                    right: None,
                                }))),
                                right: None,
                            }))),
                        }))),
                        right: None,
                    }))),
                }))),
                right: None,
            })))),
            2
        );
    }
}
