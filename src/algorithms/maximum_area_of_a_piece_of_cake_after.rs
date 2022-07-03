fn max_area(h: i32, w: i32, mut horizontal_cuts: Vec<i32>, mut vertical_cuts: Vec<i32>) -> i32 {
    horizontal_cuts.sort();
    vertical_cuts.sort();
    let modulus = 1000000007;

    let mut h_max: i64 = (h - horizontal_cuts.last().unwrap_or(&0)) as i64;
    let mut h_prior = 0;
    for h in horizontal_cuts.iter() {
        h_max = i64::max(h_max, (h - h_prior) as i64);
        h_prior = *h;
    }

    let mut v_max: i64 = (w - vertical_cuts.last().unwrap_or(&0)) as i64;
    let mut v_prior = 0;
    for v in vertical_cuts.iter() {
        v_max = i64::max(v_max, (v - v_prior) as i64);
        v_prior = *v;
    }
    ((h_max * v_max) % modulus) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(max_area(5, 4, vec![1, 2, 4], vec![1, 3]), 4);
        assert_eq!(max_area(5, 4, vec![3, 1], vec![1]), 6);
        assert_eq!(max_area(1000000000, 1000000000, vec![2], vec![2]), 81);
    }
}
