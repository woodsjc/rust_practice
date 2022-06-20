fn one_off(s1: &str, s2: &str) -> bool {
    for i in 0..s2.len() {
        let new: String = s2
            .chars()
            .enumerate()
            .into_iter()
            .filter(|(j, _)| *j != i)
            .map(|(_, c)| c)
            .collect::<String>();
        if s1.as_bytes() == new.as_bytes() {
            return true;
        }
    }
    return false;
}

fn longest_str_chain(words: Vec<String>) -> i32 {
    // organize into vec of vec with each index corresponding to length of strings inside
    // then work from longest strings backwards recording the max values for chains
    // in bottom up approach

    let mut w: Vec<Vec<&str>> = vec![vec![]];
    let mut chains: Vec<Vec<i32>> = vec![vec![]];
    let mut max = 0;

    for s in words.iter() {
        while w.len() <= s.len() {
            w.push(vec![]);
            chains.push(vec![]);
        }

        w[s.len()].push(&s);
        chains[s.len()].push(1);
    }

    for layer in (1..w.len() - 1).rev() {
        for i in 0..w[layer].len() {
            // 1 off from prior row
            for j in 0..w[layer + 1].len() {
                if one_off(w[layer][i], w[layer + 1][j]) {
                    println!(
                        "layer:{}, i:{}, j:{}, w:{:?}, chains:{:?}",
                        layer, i, j, w, chains
                    );
                    chains[layer][i] = i32::max(chains[layer][i], chains[layer + 1][j] + 1);
                }
            }
        }
    }

    for c in chains.iter() {
        if c.len() > 0 {
            max = i32::max(*c.iter().max().unwrap_or(&0), max);
        }
    }
    max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_str_chain() {
        assert_eq!(
            longest_str_chain(vec![
                String::from("a"),
                String::from("b"),
                String::from("ba"),
                String::from("bca"),
                String::from("bda"),
                String::from("bdca")
            ]),
            4
        );
        assert_eq!(
            longest_str_chain(vec![
                String::from("xbc"),
                String::from("pcxbcf"),
                String::from("xb"),
                String::from("cxbc"),
                String::from("pcxbc"),
            ]),
            5
        );
    }
}
