use std::ops::*;
use std::cmp::*;
use std::fmt::*;

//
// Delegations (allows us to verify components can work!)
//

/// Required trait for components!
pub trait MathDelegate {
    fn sqrt_delegate(&self) -> Self;

    fn sin_delegate(&self) -> Self;
    fn cos_delegate(&self) -> Self;
    fn tan_delegate(&self) -> Self;

    fn abs_delegate(&self) -> Self;
}

/// Required trait for operations requiring conversions!
pub trait Constants {
    fn rad_to_deg() -> Self;
    fn deg_to_rad() -> Self;

    fn pi() -> Self;

    fn get_one() -> Self;
}


// https://www.worthe-it.co.za/blog/2017-01-15-aliasing-traits-in-rust.html
/// Strict trait for constraining what types can be used as vector components
///
/// This trait is already implemented for [f32] and [f64]
pub trait Component:
Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> +
AddAssign + SubAssign + MulAssign + DivAssign +
Neg<Output=Self> +
PartialEq +
MathDelegate + Constants +
Clone + Copy + Default + Display
    where Self: Sized {

}

//
// Components for float types
//

// F32
impl Component for f32 {}

impl MathDelegate for f32 {
    fn sqrt_delegate(&self) -> Self {
        self.sqrt()
    }
    
    fn sin_delegate(&self) -> Self {
        self.sin()
    }

    fn cos_delegate(&self) -> Self {
        self.cos()
    }

    fn tan_delegate(&self) -> Self {
        self.tan()
    }

    fn abs_delegate(&self) -> Self {
        self.abs()
    }
}

impl Constants for f32 {
    fn rad_to_deg() -> Self {
        57.2957795131f32
    }

    fn deg_to_rad() -> Self {
        0.01745329251f32
    }

    fn pi() -> Self {
        std::f32::consts::PI
    }

    fn get_one() -> Self {
        1f32
    }
}

// F64
impl Component for f64 {}

impl MathDelegate for f64 {
    fn sqrt_delegate(&self) -> Self {
        self.sqrt()
    }

    fn sin_delegate(&self) -> Self {
        self.sin()
    }

    fn cos_delegate(&self) -> Self {
        self.cos()
    }

    fn tan_delegate(&self) -> Self {
        self.tan()
    }

    fn abs_delegate(&self) -> Self {
        self.abs()
    }
}

impl Constants for f64 {
    fn rad_to_deg() -> Self {
        57.2957795131f64
    }

    fn deg_to_rad() -> Self {
        0.01745329251f64
    }

    fn pi() -> Self {
        std::f64::consts::PI
    }

    fn get_one() -> Self {
        1f64
    }
}