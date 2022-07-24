fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix[0].len();
    let mut i = matrix.len() - 1;
    let mut j = 0;

    while matrix[i][j] != target {
        if matrix[i][j] > target && i != 0 {
            i -= 1;
        } else if matrix[i][j] < target && j < m - 1 {
            j += 1;
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_matrix() {
        assert_eq!(
            search_matrix(
                vec![
                    vec![1, 4, 7, 11, 15],
                    vec![2, 5, 8, 12, 19],
                    vec![3, 6, 9, 16, 22],
                    vec![10, 13, 14, 17, 24],
                    vec![18, 21, 23, 26, 30]
                ],
                5
            ),
            true
        );
    }
}
