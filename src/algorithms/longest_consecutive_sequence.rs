fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    nums.sort();
    let mut max = 1;
    let mut current = 1;

    for i in 1..nums.len() {
        if nums[i - 1] + 1 == nums[i] {
            current += 1;
            max = i32::max(max, current);
        } else if nums[i - 1] == nums[i] {
            {}
        } else {
            current = 1;
        }
    }
    max
}

fn longest_consecutive_o_n(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for n in nums.into_iter() {
        set.insert(n);
    }

    let mut max = 1;
    for n in set.iter() {
        if !set.contains(&(*n - 1)) {
            let mut current = 1;
            while set.contains(&(*n + current)) {
                current += 1;
            }
            max = i32::max(max, current);
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn test_longest_consecutive_o_n() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }
}
