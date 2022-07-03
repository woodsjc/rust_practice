fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    people.sort_by_key(|a| vec![-a[0], a[1]]);
    println!("people:{:?}, len:{}", people, people.len());
    for i in 0..people.len() {
        if i as i32 != people[i][1] {
            let p = people.remove(i);
            people.insert(p[1] as usize, p);
        }
    }

    people
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reconstruct_queue() {
        assert_eq!(
            reconstruct_queue(vec![
                vec![7, 0],
                vec![4, 4],
                vec![7, 1],
                vec![5, 0],
                vec![6, 1],
                vec![5, 2]
            ]),
            vec![
                vec![5, 0],
                vec![7, 0],
                vec![5, 2],
                vec![6, 1],
                vec![4, 4],
                vec![7, 1]
            ]
        );
    }
}
