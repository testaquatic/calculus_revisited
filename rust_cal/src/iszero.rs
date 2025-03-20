const EPS: f64 = 1e-6;

pub fn is_zero(x: f64) -> bool {
    -EPS < x && x < EPS
}
