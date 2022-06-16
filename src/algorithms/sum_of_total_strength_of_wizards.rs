pub fn total_strength_brute_force(strength: Vec<i32>) -> i32 {
    let mut total: i64 = 0;
    let modulus: i64 = 1000000007;

    for i in 0..strength.len() {
        for j in i..strength.len() {
            total = (total
                + *strength[i..j + 1].iter().min().unwrap() as i64
                    * strength[i..j + 1].iter().sum::<i32>() as i64)
                % modulus;
        }
    }

    total as i32
}

pub fn total_strength(strength: Vec<i32>) -> i32 {
    let mut total: i64 = 0;
    let mut stack: Vec<usize> = vec![];
    let mut pre_left_sum: Vec<i64> = vec![0; strength.len() + 1];
    let mut pre_left_scalar: Vec<i64> = vec![0; strength.len() + 1];
    let mut pre_right_sum: Vec<i64> = vec![0; strength.len() + 1];
    let mut pre_right_scalar: Vec<i64> = vec![0; strength.len() + 1];
    let modulus: i64 = 1000000007;

    //sum and scalar precompute
    for i in 0..strength.len() {
        pre_left_sum[i + 1] = (pre_left_sum[i] + strength[i] as i64) % modulus;
        pre_left_scalar[i + 1] =
            (pre_left_scalar[i] + (i + 1) as i64 * strength[i] as i64) % modulus;
    }
    for i in (0..strength.len()).rev() {
        pre_right_sum[i] = (pre_right_sum[i + 1] + strength[i] as i64) % modulus;
        pre_right_scalar[i] =
            (pre_right_scalar[i + 1] + (strength.len() - i) as i64 * strength[i] as i64) % modulus;
    }

    for right in 0..strength.len() + 1 {
        while stack.len() > 0
            && (right == strength.len() || strength[stack[stack.len() - 1]] >= strength[right])
        {
            let pivot = stack.pop().unwrap();
            let mut left = 0;
            if stack.len() > 0 {
                left = stack[stack.len() - 1] + 1;
            }
            let left_sum = (pre_left_scalar[pivot + 1]
                - pre_left_scalar[left]
                - left as i64 * (pre_left_sum[pivot + 1] - pre_left_sum[left]))
                % modulus;
            let right_sum = (pre_right_scalar[pivot + 1]
                - pre_right_scalar[right]
                - (strength.len() as i64 - right as i64)
                    * (pre_right_sum[pivot + 1] - pre_right_sum[right]))
                % modulus;
            let all_sum = (right_sum * (pivot - left + 1) as i64
                + left_sum * (right - pivot) as i64)
                % modulus;
            total = (total + all_sum * strength[pivot] as i64) % modulus;
        }
        stack.push(right);
    }

    total as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_strength_brute_force() {
        assert_eq!(total_strength_brute_force(vec![1, 3, 1, 2]), 44);
    }

    #[test]
    fn test_total_strength() {
        assert_eq!(total_strength(vec![1, 3, 1, 2]), 44);
    }
}
