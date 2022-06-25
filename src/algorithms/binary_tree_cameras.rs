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

fn dfs(node: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
    if node.is_none() {
        return 0;
    }

    let mut node = node.as_ref().unwrap().borrow_mut();
    let left = &node.left;
    let right = &node.right;

    node.val = dfs(left.as_ref().map(Rc::clone), ans) + dfs(right.as_ref().map(Rc::clone), ans);
    if node.val == 0 {
        return 3;
    } else if node.val < 3 {
        return 0;
    }
    *ans += 1;
    return 1;
}

fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut ans: i32 = 0;
    let result = dfs(root, &mut ans);
    if result > 2 {
        return ans + 1;
    }
    ans
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
