fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut left: usize = 0;
    let mut max = 0;

    for right in left + 1..nums.len() + 1 {
        let current = &nums[left..right];
        if right < nums.len() {
            match current.into_iter().position(|&x| x == nums[right]) {
                Some(p) => left += p + 1,
                None => {}
            }
        }
        max = i32::max(max, current.iter().sum());
    }
    max
}

fn maximum_unique_subarray_hashset(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut left: usize = 0;
    let mut right: usize = 0;
    let mut max = 0;
    let mut current = 0;
    let mut set = std::collections::HashSet::new();

    while left < nums.len() && right < nums.len() {
        if set.contains(&nums[right]) {
            set.remove(&nums[left]);
            current -= nums[left];
            left += 1;
        } else {
            set.insert(&nums[right]);
            current += &nums[right];
            right += 1;
            max = i32::max(max, current);
        }
    }
    max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_maximum_unique_subarray() {
        assert_eq!(maximum_unique_subarray(vec![4, 2, 4, 5, 6]), 17);
        assert_eq!(maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5]), 8);
    }

    #[test]
    fn test_maximum_unique_subarray_hashset() {
        assert_eq!(maximum_unique_subarray_hashset(vec![4, 2, 4, 5, 6]), 17);
        assert_eq!(
            maximum_unique_subarray_hashset(vec![5, 2, 1, 2, 5, 2, 1, 2, 5]),
            8
        );
    }
}
