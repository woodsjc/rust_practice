fn count_house_placement(n: i32) -> i32 {
    let n = n as usize;
    let mut dp: Vec<i64> = vec![0; n + 1];
    let modulus = 1000000007;
    dp[0] = 1;
    dp[1] = 2;
    for i in 2..n + 1 {
        dp[i] = (dp[i - 2] + dp[i - 1]) % modulus;
    }
    ((dp[n] * dp[n]) % modulus) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_house_placement() {
        assert_eq!(count_house_placement(1), 4);
        assert_eq!(count_house_placement(2), 9);
        assert_eq!(count_house_placement(3), 25);
        assert_eq!(count_house_placement(5), 169);
        assert_eq!(count_house_placement(1000), 500478595);
    }
}
