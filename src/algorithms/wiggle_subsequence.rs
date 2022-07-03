fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    let mut positive = 1;
    let mut negative = 1;

    for i in 1..nums.len() {
        let p = nums[i - 1] - nums[i];
        if p < 0 {
            negative = positive + 1;
        } else if p > 0 {
            positive = negative + 1;
        }
    }
    i32::max(positive, negative)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wiggle_max_length() {
        assert_eq!(wiggle_max_length(vec![1, 7, 4, 9, 2, 5]), 6);
        assert_eq!(
            wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]),
            7
        );
    }
}
