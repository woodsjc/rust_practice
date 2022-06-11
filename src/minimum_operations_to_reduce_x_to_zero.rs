fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
    let target = nums.iter().sum::<i32>() - x;
    if target < 0 {
        return -1;
    };
    if target == 0 {
        return nums.len() as i32;
    };

    let mut max = -1;
    let mut left: usize = 0;
    let mut current = 0;

    for right in 0..nums.len() {
        if current < target {
            current += nums[right];
        }
        while current >= target {
            if current == target {
                max = i32::max(max, right as i32 - left as i32 + 1);
            }
            current -= nums[left];
            left += 1;
        }
    }

    if max == -1 {
        return max;
    }
    nums.len() as i32 - max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_operations() {
        assert_eq!(min_operations(vec![1, 1, 4, 2, 3], 5), 2);
        assert_eq!(min_operations(vec![5, 6, 7, 8, 9], 4), -1);
        assert_eq!(min_operations(vec![3, 2, 20, 1, 1, 3], 10), 5);
        assert_eq!(min_operations(vec![1, 1], 3), -1);
    }
}
