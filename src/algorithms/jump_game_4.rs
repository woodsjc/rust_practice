use std::collections::BinaryHeap;

fn max_result(nums: Vec<i32>, k: i32) -> i32 {
    let mut pq: BinaryHeap<(i32, usize)> = BinaryHeap::new();
    let mut total = nums[0];

    pq.push((nums[0], 0));
    for i in 1..nums.len() {
        while let Some((v, j)) = pq.peek() {
            println!("i:{}, j:{}, v:{}, total:{}, pq:{:?}", i, j, v, total, pq);
            if (j + k as usize) < i {
                pq.pop();
                continue;
            }
            total = v + nums[i];
            pq.push((total, i));
            break;
        }
    }
    total
}

fn max_result_too_slow(nums: Vec<i32>, k: i32) -> i32 {
    let mut dp: Vec<i32> = vec![i32::MIN; nums.len()];
    dp[0] = nums[0];

    for i in 1..nums.len() {
        for j in i32::max(i as i32 - k, 0) as usize..i {
            //println!("i:{}, j:{}, nums[i]:{}", i, j, nums[i]);
            dp[i] = dp[i].max(nums[i] + dp[j]);
        }
    }
    //println!("dp:{:?}", dp);
    dp[nums.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_result() {
        assert_eq!(max_result(vec![1, -1, -2, 4, -7, 3], 2), 7);
        assert_eq!(max_result(vec![10, -5, -2, 4, 0, 3], 3), 17);
        assert_eq!(max_result(vec![1, -5, -20, 4, -1, 3, -6, -3], 2), 0);
        assert_eq!(max_result(vec![-11, -5, -20, 4, -1, 3, -6, -3], 2), -12);
    }

    #[test]
    fn test_max_result_too_slow() {
        assert_eq!(max_result_too_slow(vec![1, -1, -2, 4, -7, 3], 2), 7);
        assert_eq!(max_result_too_slow(vec![10, -5, -2, 4, 0, 3], 3), 17);
        assert_eq!(
            max_result_too_slow(vec![1, -5, -20, 4, -1, 3, -6, -3], 2),
            0
        );
        assert_eq!(
            max_result_too_slow(vec![-11, -5, -20, 4, -1, 3, -6, -3], 2),
            -12
        );
    }
}
