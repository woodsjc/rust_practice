fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();

    *nums.iter().rev().take(k as usize).last().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_largest() {
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }
}
