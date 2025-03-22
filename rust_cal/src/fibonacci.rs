pub fn fibonacci(n: u64) -> f64 {
    if n == 0 || n == 1 {
        return 1.0;
    }
    (2..=n).fold((1.0, 1.0), |f, _| (f.1, f.0 + f.1)).1
}

#[test]
fn test_fibonacci() {
    assert_eq!(fibonacci(10), 89.0, "fibonacci(10) != 89.0");
}
