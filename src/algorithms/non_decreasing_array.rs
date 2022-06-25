fn check_possibility(mut nums: Vec<i32>) -> bool {
    let mut modified = false;

    for i in 1..nums.len() {
        if nums[i - 1] > nums[i] {
            if modified {
                return false;
            } else {
                modified = true;
            }

            if i as i32 - 2 < 0 || nums[i - 2] <= nums[i] {
                nums[i - 1] = nums[i];
            } else {
                nums[i] = nums[i - 1];
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_possibility() {
        assert_eq!(check_possibility(vec![4, 2, 3]), true);
        assert_eq!(check_possibility(vec![4, 2, 1]), false);
        assert_eq!(check_possibility(vec![3, 4, 2, 3]), false);
    }
}
