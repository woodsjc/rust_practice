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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sums() {
        assert_eq!(two_sums(vec![1, 2, 3, 4, 5], 7), vec![3, 2]);
    }
}
