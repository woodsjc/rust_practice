fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;
    let mut total = 0;
    let mut sum = 0;
    let mut hm: HashMap<i32, i32> = HashMap::new();
    hm.insert(0, 1);

    for n in nums {
        sum += n;
        if let Some(s) = hm.get(&(sum - k)) {
            total += s;
        }
        hm.insert(sum, hm.get(&sum).unwrap_or(&0) + 1);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subarray_sums() {
        assert_eq!(subarray_sum(vec![1, 1, 1], 2), 2);
    }
}
