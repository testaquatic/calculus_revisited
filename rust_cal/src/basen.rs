pub fn basen(mut b: usize, n: usize) -> Vec<usize> {
    let mut d = Vec::new();
    while b > 0 {
        d.push(b % n);
        b /= n;
    }

    d
}

#[test]
fn test_basen() {
    assert_eq!(basen(2836, 3), vec![1, 0, 0, 0, 2, 2, 0, 1]);
}
