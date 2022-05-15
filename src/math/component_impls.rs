use super::vector::*;

impl VectorComponent for f32 {}
impl SqrtDelegate for f32 {
    fn sqrt_delegate(&self) -> Self {
        self.sqrt()
    }
}

impl VectorComponent for f64 {}
impl SqrtDelegate for f64 {
    fn sqrt_delegate(&self) -> Self {
        self.sqrt()
    }
}