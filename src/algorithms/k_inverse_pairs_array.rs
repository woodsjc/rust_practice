fn k_inverse_pairs(n: i32, k: i32) -> i32 {
    let (n, k) = (n as usize, k as usize);
    let mut dp = vec![vec![0; k + 1]; n + 1];
    let modulus = 1000000007;

    dp[0][0] = 1;
    for i in 1..=n {
        if i == 0 {
            dp[i][0] = 1;
        }
        for j in 0..=k {
            if j == 0 {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = (dp[i - 1][j] + dp[i][j - 1]) % modulus;
            }

            if j >= i {
                dp[i][j] = (modulus + dp[i][j] - dp[i - 1][j - i]) % modulus;
            }
        }
    }
    println!("{:?}", dp);
    dp[n][k]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_inverse_pairs() {
        assert_eq!(k_inverse_pairs(3, 0), 1);
        assert_eq!(k_inverse_pairs(3, 1), 2);
    }
}
