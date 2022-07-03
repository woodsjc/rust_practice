fn min_partitions(n: String) -> i32 {
    let mut max = 0;

    for c in n.chars() {
        match c.to_digit(10) {
            Some(d) => {
                if d == 0 {
                    continue;
                }
                max = i32::max(max, d as i32);
            }
            None => println!("Invalid digit: {}", c),
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_partitions() {
        assert_eq!(min_partitions("32".to_string()), 3);
        assert_eq!(min_partitions("1".to_string()), 1);
    }
}
