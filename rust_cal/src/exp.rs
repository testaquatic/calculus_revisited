use crate::{basen::basen, iszero::is_zero, natural::interative_square};

pub fn exp_sequence(x: f64, n: isize) -> f64 {
    if n < 0 {
        return 0.0;
    }

    let d = basen(n as usize, 2);
    d.iter()
        .enumerate()
        .fold((1.0, 1.0 + x / n as f64), |(mut e, f), (i, k)| {
            if *k == 1 {
                e *= interative_square(f, i as usize);
            }

            (e, f)
        })
        .0
}

#[test]
fn test_exp_sequence() {
    let expected = vec![
        "1.00000", "-4.00000", "2.25000", "-0.29630", "0.00391", "0.00000", "0.00002", "0.00016",
        "0.00039", "0.00068",
    ];
    (0..10).zip(expected).for_each(|(i, want)| {
        let result = exp_sequence(-5.0, i);
        assert_eq!(format!("{:0.5}", result), want);
    });
}

pub fn exp(x: f64) -> f64 {
    let mut f = 1.0;
    let mut p = 1.0;
    for i in 0.. {
        p *= x / (i + 1) as f64;
        f += p;
        if is_zero(p) {
            break;
        }
    }

    f
}
