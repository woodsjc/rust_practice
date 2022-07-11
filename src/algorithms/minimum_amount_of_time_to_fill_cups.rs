fn fill_cups(amount: Vec<i32>) -> i32 {
    let mut amount = amount;
    let mut total: i32 = amount.iter().sum();
    let mut count = 0;
    amount.sort();

    while total > 0 {
        amount.sort();
        amount[2] -= 1;
        total -= 1;
        if amount[1] > 0 {
            amount[1] -= 1;
            total -= 1;
        }
        count += 1
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_cups() {
        assert_eq!(fill_cups(vec![5, 0, 0]), 5);
        assert_eq!(fill_cups(vec![5, 4, 4]), 7);
    }
}
