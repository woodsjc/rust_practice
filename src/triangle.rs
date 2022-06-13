fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
    if triangle.len() == 0 {
        return 0;
    }

    for layer in (0..triangle.len() - 1).rev() {
        for position in 0..triangle[layer].len() {
            let prior: usize = layer + 1;
            triangle[layer][position] +=
                i32::min(triangle[prior][position], triangle[prior][position + 1]);
        }
    }
    triangle[0][0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_minimum_total() {
        assert_eq!(
            minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
        assert_eq!(minimum_total(vec![vec![-10]]), -10);
        assert_eq!(
            minimum_total(vec![vec![-1], vec![2, 3], vec![1, -1, -3]]),
            -1
        );
    }
}
