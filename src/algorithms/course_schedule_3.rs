use std::collections::BinaryHeap;

fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
    let mut pq: BinaryHeap<i32> = BinaryHeap::new();
    let mut time = 0;
    courses.sort_by(|a, b| a[1].partial_cmp(&b[1]).unwrap());

    for c in courses.iter() {
        let (start, end) = (c[0], c[1]);
        time += start;
        pq.push(start);
        if time > end {
            time -= pq.pop().unwrap();
        }
    }
    pq.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedule_course() {
        assert_eq!(schedule_course(vec![vec![1, 2]]), 1);
        assert_eq!(schedule_course(vec![vec![3, 2], vec![4, 3]]), 0);
        assert_eq!(
            schedule_course(vec![vec![2, 5], vec![2, 19], vec![1, 8], vec![1, 3]]),
            4
        );
    }
}
