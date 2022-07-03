fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    for n in nums.iter_mut() {
        *n += sum;
        sum = *n;
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_sum() {
        assert_eq!(running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    }
}
