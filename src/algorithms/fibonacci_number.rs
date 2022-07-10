fn fib(n: i32) -> i32 {
    if n == 0 { return 0; }
    if n == 1 || n == 1 {
        return 1;
    }
    return fib(n - 2) + fib(n - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib(4), 3);
    }
}
