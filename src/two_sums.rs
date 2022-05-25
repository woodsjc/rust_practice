use std::collections::HashMap;

pub fn two_sums(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ht: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

    for (i, n) in nums.iter().enumerate() {
        match ht.get(&(target - n)) {
            Some(x) => return vec![i as i32, *x],
            None => {
                ht.insert(*n, i as i32);
            }
        }
    }
    vec![]
}
