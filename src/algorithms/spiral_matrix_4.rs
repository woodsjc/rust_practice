#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
    let mut result = vec![vec![-1; n as usize]; m as usize];
    let mut i = 0;
    let mut j = 0;
    let mut head = head;
    let mut current = Direction::Right;

    println!("result:{:?}", result);
    while let Some(ref node) = head {
        println!("i:{}, j:{}, current:{:?}", i, j, current);
        //println!("i:{}, j:{}, v:{:?}, current:{:?}", i, j, node, current);
        let v = node.val;
        result[i as usize][j as usize] = v;

        match current {
            Direction::Right => {
                if j + 1 < n && result[i as usize][(j + 1) as usize] == -1 {
                    j += 1;
                } else {
                    current = Direction::Down;
                    i += 1;
                }
            }
            Direction::Down => {
                if i + 1 < m && result[(i + 1) as usize][j as usize] == -1 {
                    i += 1;
                } else {
                    current = Direction::Left;
                    j -= 1;
                }
            }
            Direction::Left => {
                if j > 0 && result[i as usize][(j - 1) as usize] == -1 {
                    j -= 1;
                } else {
                    current = Direction::Up;
                    i -= 1;
                }
            }
            Direction::Up => {
                if i > 0 && result[(i - 1) as usize][j as usize] == -1 {
                    i -= 1;
                } else {
                    current = Direction::Right;
                    j += 1;
                }
            }
        }
        head = head.unwrap().next;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_matrix() {
        assert_eq!(
            spiral_matrix(
                1,
                4,
                Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: Some(Box::new(ListNode { val: 2, next: None }))
                    }))
                }))
            ),
            vec![vec![0, 1, 2, -1]]
        );
        assert_eq!(
            spiral_matrix(
                3,
                5,
                Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 0,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode {
                                val: 6,
                                next: Some(Box::new(ListNode {
                                    val: 8,
                                    next: Some(Box::new(ListNode {
                                        val: 1,
                                        next: Some(Box::new(ListNode {
                                            val: 7,
                                            next: Some(Box::new(ListNode {
                                                val: 9,
                                                next: Some(Box::new(ListNode {
                                                    val: 4,
                                                    next: Some(Box::new(ListNode {
                                                        val: 2,
                                                        next: Some(Box::new(ListNode {
                                                            val: 5,
                                                            next: Some(Box::new(ListNode {
                                                                val: 5,
                                                                next: Some(Box::new(ListNode {
                                                                    val: 0,
                                                                    next: None
                                                                }))
                                                            }))
                                                        }))
                                                    }))
                                                }))
                                            }))
                                        }))
                                    }))
                                }))
                            }))
                        }))
                    }))
                })),
            ),
            vec![vec![3,0,2,6,8],vec![5,0,-1,-1,1],vec![5,2,4,9,7]]
        );
    }
}
