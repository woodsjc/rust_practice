use std::collections::HashMap;

fn num_submatrix_sum_target(mut matrix: Vec<Vec<i32>>, target: i32) -> i32 {
    let mut total = 0;

    for i in 0..matrix.len() {
        for j in 1..matrix[i].len() {
            matrix[i][j] += matrix[i][j - 1];
        }
    }

    for i in 0..matrix[0].len() {
        for j in i..matrix[0].len() {
            let mut hm: HashMap<i32, i32> = HashMap::from([(0, 1)]);
            let mut sum = 0;
            for k in 0..matrix.len() {
                sum += matrix[k][j];
                if i > 0 {
                    sum += -matrix[k][i - 1];
                }
                if let Some(s) = hm.get(&(sum - target)) {
                    total += s;
                }
                hm.insert(sum, hm.get(&sum).unwrap_or(&0) + 1);
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_submatrix_sum_target() {
        assert_eq!(
            num_submatrix_sum_target(vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]], 0),
            4
        );
        assert_eq!(
            num_submatrix_sum_target(vec![vec![1, -1], vec![-1, 1]], 0),
            5
        );
        assert_eq!(
            num_submatrix_sum_target(
                vec![
                    vec![0, 1, 1, 1, 0, 1],
                    vec![0, 0, 0, 0, 0, 1],
                    vec![0, 0, 1, 0, 0, 1],
                    vec![1, 1, 0, 1, 1, 0],
                    vec![1, 0, 0, 1, 0, 0]
                ],
                0
            ),
            43
        );
    }
}
