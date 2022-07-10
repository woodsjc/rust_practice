fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut dp: Vec<i32> = vec![0; cost.len()];
    dp[0] = cost[0];
    dp[1] = cost[1];
    for i in 2..cost.len() {
        dp[i] = i32::min(dp[i - 2], dp[i - 1]) + cost[i];
    }
    i32::min(dp[cost.len()-2], dp[cost.len()-1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_climbing_stairs() {
        assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }
}
