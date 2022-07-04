fn candy(ratings: Vec<i32>) -> i32 {
    let mut result = vec![1; ratings.len()];

    //left
    for i in 1..ratings.len() {
        if ratings[i - 1] < ratings[i] {
            result[i] = result[i-1] + 1;
        }
    }

    //right
    for i in (0..ratings.len() - 1).rev() {
        if ratings[i] > ratings[i + 1] {
            result[i] = i32::max(result[i], result[i + 1] + 1);
        }
    }

    println!("result:{:?}", result);
    result.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candy() {
        assert_eq!(candy(vec![1, 0, 2]), 5);
        assert_eq!(candy(vec![1, 2, 2]), 4);
        assert_eq!(candy(vec![1, 2, 87, 87, 87, 2, 1]), 13);
    }
}
