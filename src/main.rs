mod add_two_numbers;
mod longest_palindrome;
mod longest_substring_without_repeating_characters;
mod maximum_subarray_min_product;
mod minimum_operations_to_reduce_x_to_zero;
mod sum_of_subarray_minimums;
mod sum_of_total_strength_of_wizards;
mod two_sums;

fn main() {
    let s = two_sums::two_sums(vec![1, 2, 3, 4, 5], 7);
    print!("{:?}\n", s);
    let s = two_sums::two_sums(vec![3, 3], 6);
    print!("{:?}\n", s);

    let s = longest_palindrome::longest_palindrome("sssswwwwweeee");
    print!("Longest palindrome: {}\n", s);

    let n1 = Some(Box::new(add_two_numbers::ListNode {
        val: 2,
        next: Some(Box::new(add_two_numbers::ListNode {
            val: 4,
            next: Some(Box::new(add_two_numbers::ListNode { val: 3, next: None })),
        })),
    }));
    let n2 = Some(Box::new(add_two_numbers::ListNode {
        val: 5,
        next: Some(Box::new(add_two_numbers::ListNode {
            val: 6,
            next: Some(Box::new(add_two_numbers::ListNode { val: 4, next: None })),
        })),
    }));
    let s = add_two_numbers::add_two_numbers(n1, n2);
    print!("Add two linked lists: {:?}\n", s);
}
