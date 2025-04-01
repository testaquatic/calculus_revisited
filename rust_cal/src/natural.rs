use crate::basen::basen;

pub fn interative_square(a: f64, n: usize) -> f64 {
    (0..n).fold(a, |acc, _| acc * acc)
}

#[test]
fn test_interative_square() {
    assert_eq!(interative_square(2.0, 4), 2.0_f64.powi(2_i32.pow(4)));
    assert_eq!(interative_square(2.0, 0), 2.0);
}

pub fn natural_sequence(n: usize) -> f64 {
    let f = 1.0 + 1.0 / (n as f64);
    let d = basen(n, 2);
    d.iter()
        .enumerate()
        .filter_map(|(i, k)| {
            if *k == 1 {
                Some(interative_square(f, i))
            } else {
                None
            }
        })
        .product()
}

#[test]
fn test_natural_sequence() {
    let output = vec![
        "2.000	2.0000	(n = 1e0)",
        "2.594	2.5937	(n = 1e1)",
        "2.705	2.7048	(n = 1e2)",
        "2.717	2.7169	(n = 1e3)",
        "2.718	2.7181	(n = 1e4)",
        "2.718	2.7183	(n = 1e5)",
        "2.718	2.7183	(n = 1e6)",
        "2.718	2.7183	(n = 1e7)",
    ];

    let mut n = 1;
    for line in output {
        let a = natural_sequence(n);
        let out_string = format!("{:.3}\t{:.4}\t(n = {:e})", a, a, n);
        assert_eq!(out_string, line);
        n *= 10;
    }
}
