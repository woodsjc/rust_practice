pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    let mut stack_prev: Vec<(i32, usize)> = vec![];
    let mut stack_next: Vec<(i32, usize)> = vec![];
    let mut left: Vec<usize> = (0..arr.len()).collect();
    let mut right: Vec<usize> = (0..arr.len()).rev().collect();
    let mut total: i64 = 0;
    let modulus = 1000000007;

    //previous
    for i in 0..arr.len() {
        while stack_prev.len() > 0 && stack_prev[stack_prev.len() - 1].0 >= arr[i] {
            stack_prev.pop();
        }
        left[i] = i + 1;
        if stack_prev.len() > 0 {
            left[i] = i - stack_prev[stack_prev.len() - 1].1;
        }
        stack_prev.push((arr[i], i));
    }

    //next
    for i in (0..arr.len()).rev() {
        while stack_next.len() > 0 && stack_next[stack_next.len() - 1].0 > arr[i] {
            stack_next.pop();
        }
        right[i] = arr.len() - i;
        if stack_next.len() > 0 {
            right[i] = stack_next[stack_next.len() - 1].1 - i;
        }
        stack_next.push((arr[i], i));
    }

    for i in 0..arr.len() {
        total = (total + arr[i] as i64 * left[i] as i64 * right[i] as i64) % modulus;
        print!(
            "total:{}, arr[i]:{}, left[i]:{}, right[i]:{}\n",
            total, arr[i], left[i], right[i]
        );
    }
    total as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_subarray_mins() {
        assert_eq!(sum_subarray_mins(vec![3, 1, 2, 4]), 17);
        assert_eq!(sum_subarray_mins(vec![11, 81, 94, 43, 3]), 444);
    }
}
