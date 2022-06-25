fn furthest_building(heights: Vec<i32>, mut bricks: i32, ladders: i32) -> i32 {
    let mut queue = std::collections::BinaryHeap::new();
    let mut i = 0;

    while i < heights.len() -1 {
        let diff = heights[i] - heights[i + 1];
        println!(
            "i:{}, diff:{}, heights[i-1]:{}, heights[i]:{}, bricks:{}, ladders:{}, queue:{:?}",
            i,
            diff,
            heights[i],
            heights[i + 1],
            bricks,
            ladders,
            queue
        );

        if diff < 0 {
            queue.push(diff);
            if queue.len() as i32 > ladders {
                bricks += queue.pop().unwrap();
            }

            if bricks < 0 {
                break;
            }
        }
        i += 1;
    }
    i as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_furthest_building() {
        assert_eq!(furthest_building(vec![4, 2, 7, 6, 9, 14, 12], 5, 1), 4);
        assert_eq!(
            furthest_building(vec![4, 12, 2, 7, 3, 18, 20, 3, 19], 10, 2),
            7
        );
    }
}
