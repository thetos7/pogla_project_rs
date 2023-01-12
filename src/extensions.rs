use cgmath::{num_traits::Float, Vector3, BaseNum, InnerSpace};

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

pub trait SafeNormalize: InnerSpace {
    type SafeNormalizeOutput;

    fn safe_normalize(self) -> Self::SafeNormalizeOutput;
}

impl<S> SafeNormalize for Vector3<S> where S: BaseNum + Float{
    type SafeNormalizeOutput = Self;

    fn safe_normalize(self) -> Self::SafeNormalizeOutput {
        let norm = self.magnitude();
        if norm == S::from(0.).unwrap() {
            self
        } else {
            self.normalize()
        }
    }
}
