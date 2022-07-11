use std::collections::BinaryHeap;

#[derive(Debug)]
struct SmallestInfiniteSet {
    set: BinaryHeap<i32>,
    pops: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        return SmallestInfiniteSet {
            set: BinaryHeap::new(),
            pops: 0,
        };
    }

    fn pop_smallest(&mut self) -> i32 {
        if self.set.len() == 0 {
            self.pops += 1;
            return self.pops;
        }

        let result = self.set.pop().unwrap();
        while let Some(r) = self.set.peek() {
            if *r == result {
                self.set.pop();
            } else {
                break;
            }
        }
        -result
    }

    fn add_back(&mut self, num: i32) {
        if num > self.pops {
            return;
        }
        self.set.push(-num);
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_infinite_set() {
        let mut obj = SmallestInfiniteSet::new();
        let ret_1: i32 = obj.pop_smallest();
        assert_eq!(ret_1, 1);
        obj.pop_smallest();
        obj.pop_smallest();
        obj.add_back(2);
        assert_eq!(*obj.set.peek().unwrap_or(&0), -2);

        //adds dupe items
        //obj.add_back(2);
        //println!("{:?}", obj);
        //panic!();
    }
}
