mod basen;
mod fibonacci;
mod iszero;
mod natural;
mod sqrt;

/// 원시 타입에서 벗어날 것 같지 않다.
trait RustCal: Copy {
    fn is_zero(self) -> bool;
    fn sqrt(self) -> Self;
    fn interative_square(self, n: usize) -> Self;
}

impl RustCal for f64 {
    #[inline]
    fn is_zero(self) -> bool {
        iszero::is_zero(self)
    }

    #[inline]
    fn sqrt(self) -> Self {
        sqrt::sqrt(self)
    }

    #[inline]
    fn interative_square(self, n: usize) -> Self {
        natural::interative_square(self, n)
    }
}
