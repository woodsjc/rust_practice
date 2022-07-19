fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];

    for i in 0..num_rows as usize {
        result.push(vec![]);
        for j in 0..=i {
            if j != 0 && j != i {
                let tmp = result[i - 1][j - 1] + result[i - 1][j];
                result[i].push(tmp);
            } else {
                result[i].push(1);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        assert_eq!(generate(1), [[1]]);
    }
}
