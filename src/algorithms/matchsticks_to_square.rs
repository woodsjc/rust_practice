fn dfs(edges: &mut [i32; 4], edge: i32, matchsticks: &[i32]) -> bool {
    if matchsticks.len() == 0 {
        return true;
    }

    let s = matchsticks[0];
    for i in 0..4 {
        if edges[i] >= s {
            edges[i] -= s;
            if dfs(edges, edge, &matchsticks[1..]) {
                return true;
            }
            edges[i] += s;
        }
    }
    false
}

fn makesquare(matchsticks: Vec<i32>) -> bool {
    let perimeter: i32 = matchsticks.iter().sum();
    if perimeter % 4 != 0 || matchsticks.len() < 4 {
        return false;
    }
    let edge = perimeter / 4;
    let mut edges = [edge; 4];

    println!("edge:{}, perimeter:{}", edge, perimeter);

    for s in matchsticks.iter() {
        if *s > edge {
            return false;
        }
    }

    dfs(&mut edges, edge, &matchsticks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_makesquare() {
        assert_eq!(makesquare(vec![1, 1, 2, 2, 2]), true);
        assert_eq!(makesquare(vec![5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3]), true);
    }
}
