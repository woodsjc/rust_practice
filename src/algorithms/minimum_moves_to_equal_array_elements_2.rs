fn min_moves(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let target = nums[nums.len()/2];
    //let target: i32 = nums.iter().sum::<i32>() / nums.len() as i32;
    let mut total = 0;

    for n in nums.iter() {
        total += i32::abs(target - n);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_moves() {
        assert_eq!(min_moves(vec![1, 2, 3]), 2);
        assert_eq!(min_moves(vec![1, 10, 2, 9]), 16);
        assert_eq!(min_moves(vec![1, 0, 0, 8, 6]), 14);
    }
}
