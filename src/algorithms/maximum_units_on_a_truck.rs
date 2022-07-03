fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
    let mut total = 0;
    box_types.sort_by_key(|a| -a[1]);
    for b in box_types.iter() {
        let (n, u) = (b[0], b[1]);
        if n < truck_size {
            total += n * u;
            truck_size -= n;
        } else {
            total += truck_size * u;
            break;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_units() {
        assert_eq!(
            maximum_units(vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4),
            8
        );
    }
}
