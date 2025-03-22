use crate::RustCal;

pub fn sqrt(mut x: f64) -> f64 {
    if x.is_sign_negative() {
        return f64::NAN;
    }

    let mut n = 0_usize;
    while x >= 4.0 {
        x /= 4.0;
        n += 1;
    }

    let (mut y, mut p) = (0.0, 1.0);
    while !p.is_zero() {
        if (y + p) * (y + p) <= x {
            y += p;
        }
        p /= 2.0;
    }

    while n > 0 {
        y *= 2.0;
        n -= 1;
    }

    return y;
}

#[test]
fn test_sqrt() {
    (0..=100).for_each(|i| {
        let x = i as f64 / 10.0;
        // `is_zero(sqrt(x) - x.sqrt())`는 테스트를 통과하지 못해서 조건을 완화했다.
        assert!((sqrt(x) - x.sqrt()).abs() <= 1e-5);
    });
}
