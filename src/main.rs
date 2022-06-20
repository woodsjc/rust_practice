#[allow(dead_code)]
mod algorithms;

fn main() {
    let s = algorithms::two_sums::two_sums(vec![1, 2, 3, 4, 5], 7);
    print!("{:?}\n", s);
    let s = algorithms::two_sums::two_sums(vec![3, 3], 6);
    print!("{:?}\n", s);

    let s = algorithms::longest_palindrome::longest_palindrome("sssswwwwweeee");
    print!("Longest palindrome: {}\n", s);

    let n1 = Some(Box::new(algorithms::add_two_numbers::ListNode {
        val: 2,
        next: Some(Box::new(algorithms::add_two_numbers::ListNode {
            val: 4,
            next: Some(Box::new(algorithms::add_two_numbers::ListNode {
                val: 3,
                next: None,
            })),
        })),
    }));
    let n2 = Some(Box::new(algorithms::add_two_numbers::ListNode {
        val: 5,
        next: Some(Box::new(algorithms::add_two_numbers::ListNode {
            val: 6,
            next: Some(Box::new(algorithms::add_two_numbers::ListNode {
                val: 4,
                next: None,
            })),
        })),
    }));
    let s = algorithms::add_two_numbers::add_two_numbers(n1, n2);
    print!("Add two linked lists: {:?}\n", s);
}
