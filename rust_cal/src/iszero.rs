const EPS: f64 = 1e-6;

pub fn is_zero(x: f64) -> bool {
    -EPS < x && x < EPS
}

#[test]
fn test_is_zero() {
    assert!(is_zero(0.0));
    assert!(is_zero(1e-10));
    assert!(!is_zero(1e-5));
}
