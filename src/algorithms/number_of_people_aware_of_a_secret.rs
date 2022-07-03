fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
    let modulus = 1000000007;
    let mut d: Vec<i64> = vec![0; n as usize];
    let mut current: i64 = 0;
    let mut result: i64 = 0;
    d[0] = 1;
    if n <= forget {
        result = 1;
    }

    for i in delay as usize..n as usize {
        d[i] = current;
        if i as i32 - delay >= 0 {
            d[i] = (d[i] + d[i - delay as usize]) % modulus;
        }

        if i as i32 - forget >= 0 {
            d[i] = (d[i] + modulus - d[i - forget as usize]) % modulus;
        }
        current = d[i];
        if i as i32 + forget >= n {
            result = (result + current) % modulus;
        }
        //total = (total + d[i]) % modulus;
        //println!("i:{}, current:{}, total:{}", i, current, total);
    }

    println!("n:{}, delay:{}, forget:{}, d:{:?}", n, delay, forget, d);
    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_people_aware_of_secret() {
        assert_eq!(people_aware_of_secret(4, 1, 3), 6);
        assert_eq!(people_aware_of_secret(6, 2, 4), 5);
        assert_eq!(people_aware_of_secret(4, 1, 4), 8);
        assert_eq!(people_aware_of_secret(684, 18, 496), 653668527);
        assert_eq!(people_aware_of_secret(289, 7, 23), 790409951);
    }
}
