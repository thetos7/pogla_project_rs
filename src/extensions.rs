pub trait CeilDiv<Rhs> {
    type Output;

    /// Performs a ceiled division.
    /// Returns rounded up quotient.
    fn ceil_div(self, rhs: Rhs) -> Self::Output;
}

impl CeilDiv<usize> for usize {
    type Output = usize;

    fn ceil_div(self, rhs: usize) -> Self::Output {
        self / rhs + ((self % rhs) != 0) as usize
    }
}
