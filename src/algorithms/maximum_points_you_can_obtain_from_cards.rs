fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    // find min array len - k and sum rest
    let mut min = i32::MAX;
    let min_len = card_points.len() - k as usize;
    for i in min_len..card_points.len()+1 {
        min = i32::min(min, card_points[i - min_len..i].iter().sum());
        println!(
            "i:{}, min:{}, card_points[i-min_len..i+1]:{:?}",
            i,
            min,
            card_points[i - min_len..i].to_vec()
        );
    }
    card_points.iter().sum::<i32>() - min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_score() {
        assert_eq!(max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
        assert_eq!(max_score(vec![2, 2, 2], 2), 4);
        assert_eq!(max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
        assert_eq!(max_score(vec![96, 90, 41, 82, 39, 74, 64, 50, 30], 8), 536);
    }
}
