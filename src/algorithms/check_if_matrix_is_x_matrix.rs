fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if i == j || i == grid.len() - j - 1 {
                if grid[i][j] == 0 {
                    return false;
                }
            } else {
                if grid[i][j] != 0 {
                    return false;
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_x_matrix() {
        assert_eq!(
            check_x_matrix(vec![vec![5, 7, 0], vec![0, 3, 1], vec![0, 5, 0]]),
            false
        );
        assert_eq!(
            check_x_matrix(vec![
                vec![2, 0, 0, 1],
                vec![0, 3, 1, 0],
                vec![0, 5, 2, 0],
                vec![4, 0, 0, 2]
            ]),
            true
        );
    }
}
