fn number_of_pairs(mut nums: Vec<i32>) -> Vec<i32> {
    let mut pairs = 0;
    let mut skip = false;
    nums.sort();
    for i in 1..nums.len() {
        if skip {
            skip = false;
            continue;
        }
        if nums[i - 1] == nums[i] {
            pairs += 1;
            skip = true;
        }
    }
    vec![pairs, nums.len() as i32 - 2 * pairs]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_pairs() {
        assert_eq!(number_of_pairs(vec![1, 1]), vec![1, 0]);
    }
}
