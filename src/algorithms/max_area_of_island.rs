use std::cell::RefCell;
use std::rc::Rc;

fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let visited: Rc<RefCell<Vec<Vec<bool>>>> =
        Rc::new(RefCell::new(vec![vec![false; grid[0].len()]; grid.len()]));
    let mut max = 0;

    fn search(
        i: usize,
        j: usize,
        grid: &Vec<Vec<i32>>,
        visited: Rc<RefCell<Vec<Vec<bool>>>>,
    ) -> i32 {
        let mut total = 0;
        if i >= grid.len() || j >= grid[i].len() || grid[i][j] == 0 || visited.borrow()[i][j] {
            return 0;
        }
        visited.borrow_mut()[i][j] = true;
        total += 1;

        if i as i32 - 1 >= 0 && grid[i - 1][j] == 1 {
            total += search(i - 1, j, grid, visited.clone());
        }
        if i + 1 < grid.len() && grid[i + 1][j] == 1 {
            total += search(i + 1, j, grid, visited.clone());
        }
        if j as i32 - 1 >= 0 && grid[i][j - 1] == 1 {
            total += search(i, j - 1, grid, visited.clone());
        }
        if j + 1 < grid[i].len() && grid[i][j + 1] == 1 {
            total += search(i, j + 1, grid, visited.clone());
        }

        total
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if visited.borrow()[i][j] || grid[i][j] == 0 {
                continue;
            }
            max = max.max(search(i, j, &grid, visited.clone()));
            println!("i:{}, j:{}, max:{}", i, j, max);
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area_of_island() {
        assert_eq!(
            max_area_of_island(vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ]),
            6
        );
    }
}
