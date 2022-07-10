use std::collections::HashMap;

fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
    let dp: HashMap<(i32, i32, i32), i32> = HashMap::new();

    fn dfs(i: i32, t: i32, p: i32, dp: HashMap<(i32, i32, i32), i32>) -> i32 {
        let key = (i,t,p);

        if i == houses.len() || t < 0 || m - i < t {
            return -1
        }

        match dp.get_mut(&key) {
            Some(v) => v,
            None => {
                if houses[i] == 0 {
                    dp.insert(key, i32::min(
                            dfs(i+1, t - 


        0
    }

    let result = dfs(0, target, -1, dp);
    if result > 0 {
        result
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost() {
        assert_eq!(
            min_cost(
                vec![0, 0, 0, 0, 0],
                vec![
                    vec![1, 10],
                    vec![10, 1],
                    vec![10, 1],
                    vec![1, 10],
                    vec![5, 1]
                ],
                5,
                2,
                3
            ),
            9
        );
    }
}
