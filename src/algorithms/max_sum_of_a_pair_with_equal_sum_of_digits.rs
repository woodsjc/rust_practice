use std::collections::HashMap;

fn maximum_sum(nums: Vec<i32>) -> i32 {
    let mut ht: HashMap<i32, i32> = HashMap::new();
    let mut max = -1;
    for n in nums.iter() {
        let mut digits = 0;
        let mut num = *n;
        while num >= 1 {
            digits += num % 10;
            num /= 10;
        }
        if let Some(r) = ht.get(&digits) {
            max = max.max(*n + r);
            if r < n {
                ht.insert(digits, *n);
            }
        } else {
            ht.insert(digits, *n);
        }
        println!("n:{}, digits:{}, max:{}, ht:{:?}", n, digits, max, ht);
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_sum() {
        assert_eq!(maximum_sum(vec![10, 12, 19, 14]), -1);
        assert_eq!(maximum_sum(vec![18, 43, 36, 13, 7]), 54);
    }
}
