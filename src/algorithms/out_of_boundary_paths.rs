fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
    if n == 0 || m == 0 {
        return 0;
    }
    let (m, n, max_move) = (m as usize, n as usize, max_move as usize);
    let mut dp: Vec<Vec<Vec<i64>>> = vec![vec![vec![-1; max_move + 1]; n + 1]; m + 1];
    println!(
        "Start row:{}, start column:{}, max_move:{}, m:{}, n:{}",
        start_row, start_column, max_move, m, n
    );

    fn dfs(m: usize, n: usize, moves: usize, i: i32, j: i32, dp: &mut Vec<Vec<Vec<i64>>>) -> i64 {
        println!("i:{}, j:{}, moves:{}", i, j, moves);
        let modulus = 1000000007;
        if i < 0 || j < 0 || i as usize >= m || j as usize >= n {
            return 1;
        } else if moves == 0 {
            return 0;
        }
        let (i, j) = (i as usize, j as usize);
        if dp[i][j][moves] != -1 {
            return dp[i][j][moves];
        }

        dp[i][j][moves] = (dfs(m, n, moves - 1, i as i32 + 1, j as i32, dp)
            + dfs(m, n, moves - 1, i as i32 - 1, j as i32, dp)
            + dfs(m, n, moves - 1, i as i32, j as i32 + 1, dp)
            + dfs(m, n, moves - 1, i as i32, j as i32 - 1, dp))
            % modulus;
        println!("dp[{}][{}][{}]:{}", i, j, moves, dp[i][j][moves]);
        dp[i][j][moves]
    }

    dfs(m, n, max_move, start_row, start_column, &mut dp) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_paths() {
        assert_eq!(find_paths(2, 2, 2, 0, 0), 6);
        assert_eq!(find_paths(1, 3, 3, 0, 1), 12);
    }
}
