use std::collections::HashMap;

fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    // go through edges & calc diff between
    // remove top or bottom to minimize separation
    let mut diff: Vec<i32> = vec![0; nums.len()];
    let mut ht: HashMap<usize, i32> = HashMap::new();

    for (i, e) in edges.iter().enumerate() {
        let (start, end) = (e[0] as usize, e[1] as usize);
        diff[i] = nums[start] ^ nums[end];

        //find root
        ht.get(&start).get_or_insert(&0);
        *ht.get_mut(&start).unwrap() += 1;
        ht.get(&end).get_or_insert(&0);
        *ht.get_mut(&end).unwrap() += 1;
    }

    let root = *ht.iter().max().map(|(k, _)| k).unwrap();

    //traverse diff to root
    for i in 0..diff.len() {
        if edges[i][0] == root as i32 || edges[i][1] == root as i32 {
            continue;
        }
        //how to find connected???
        //need to build tree
    }

    (nums.len() + edges.len()) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_score() {
        assert_eq!(
            minimum_score(
                vec![1, 5, 5, 4, 11],
                vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]]
            ),
            9
        );
    }
}
