fn maximum_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let sum1 = nums1.iter().sum::<i32>();
    let sum2 = nums2.iter().sum::<i32>();
    let (mut first, mut second, mut max1, mut max2) = (0, 0, 0, 0);
    let mut max = i32::max(sum1, sum2);

    for i in 0..nums1.len() {
        first += nums2[i] - nums1[i];
        second += nums1[i] - nums2[i];

        max1 = i32::max(max1, first);
        max2 = i32::max(max2, second);

        if first < 0 {
            first = 0;
        }
        if second < 0 {
            second = 0;
        }
    }

    max = i32::max(max, sum1 + max1);
    i32::max(max, sum2 + max2)
}

fn maximum_spliced_array_brute_force(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut t1 = nums1.iter().sum::<i32>();
    let t2 = nums2.iter().sum::<i32>();

    if t2 > t1 {
        (nums1, nums2) = (nums2, nums1);
        t1 = t2;
    }

    for left in 0..nums1.len() {
        for right in left..nums1.len() {
            let diff = nums2[left..right + 1].iter().sum::<i32>()
                - nums1[left..right + 1].iter().sum::<i32>();
            max = i32::max(max, diff);
        }
    }
    t1 + max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_spliced_array() {
        assert_eq!(maximum_spliced_array(vec![7, 11, 13], vec![1, 1, 1]), 31);
        assert_eq!(
            maximum_spliced_array(vec![20, 40, 20, 70, 30], vec![50, 20, 50, 40, 20]),
            220
        );
        assert_eq!(
            maximum_spliced_array(vec![10, 20, 50, 15, 30, 10], vec![40, 20, 10, 100, 10, 10]),
            230
        );
    }

    #[test]
    fn test_maximum_spliced_array_brute_force() {
        assert_eq!(
            maximum_spliced_array_brute_force(vec![7, 11, 13], vec![1, 1, 1]),
            31
        );
        assert_eq!(
            maximum_spliced_array_brute_force(vec![20, 40, 20, 70, 30], vec![50, 20, 50, 40, 20]),
            220
        );
        assert_eq!(
            maximum_spliced_array_brute_force(
                vec![10, 20, 50, 15, 30, 10],
                vec![40, 20, 10, 100, 10, 10]
            ),
            230
        );
    }
}
