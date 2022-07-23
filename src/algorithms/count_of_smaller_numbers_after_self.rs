fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for (i, n) in nums.iter().enumerate() {
        let mut count = 0;
        for j in i + 1..nums.len() {
            if n > &nums[j] {
                count += 1;
            }
        }
        result.push(count);
    }
    //result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_smaller() {
        assert_eq!(count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
    }
}
