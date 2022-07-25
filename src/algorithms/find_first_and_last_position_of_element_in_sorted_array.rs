fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    fn search(l: i32, r: i32, t: i32, nums: &Vec<i32>) -> Vec<i32> {
        if nums.len() == 0 || l > r {
            return vec![-1, -1];
        }
        let median = (r - l) / 2 + l;
        if nums[median as usize] > t {
            return search(l, median - 1, t, nums);
        } else if nums[median as usize] < t {
            return search(median + 1, r, t, nums);
        } else {
            // hit target
            let l = search(l, median - 1, t, nums);
            let r = search(median + 1, r, t, nums);
            let l = if l[0] == -1 { median as i32 } else { l[0] };
            let r = if r[1] == -1 { median as i32 } else { r[1] };
            return vec![l, r];
        }
    }

    search(0, nums.len() as i32 - 1, target, &nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_range() {
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
    }
}
