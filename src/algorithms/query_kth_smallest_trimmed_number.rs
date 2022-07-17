fn get_kth(nums: &Vec<String>, k: usize, t: usize) -> i32 {
    let mut current: Vec<(&str, usize)> = vec![];
    for (i, n) in nums.iter().enumerate() {
        current.push((&n[n.len() - t..], i));
    }
    current.sort_by_key(|&a| a.0);
    current[k - 1].1 as i32
}

fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![];

    for q in queries {
        let (k, t) = (q[0], q[1]);
        let r = get_kth(&nums, k as usize, t as usize);
        result.push(r);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_trimmed_numbers() {
        assert_eq!(
            smallest_trimmed_numbers(
                vec![
                    "24".to_string(),
                    "37".to_string(),
                    "96".to_string(),
                    "04".to_string()
                ],
                vec![vec![2, 1], vec![2, 2]]
            ),
            vec![3, 0]
        );
    }
}
