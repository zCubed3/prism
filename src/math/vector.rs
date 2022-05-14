use std::ops::*;
use std::fmt::{Debug, Display, Formatter};

// Trait for acceptable VectorElements
// https://www.worthe-it.co.za/blog/2017-01-15-aliasing-traits-in-rust.html
pub trait VectorComponent:
    Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> +
    AddAssign + SubAssign + MulAssign + DivAssign +
    Clone + Copy + Default + Display
    where Self: Sized {

}

// By default VectorComponent isn't implemented! We've implemented it for floating point types inside of component_impls.rs!
#[derive(Copy, Clone)]
pub struct Vector<T: VectorComponent, const COUNT: usize> {
    pub data: [T; COUNT],
}

// Subtypes can declare a better "new" constructor
impl<T: VectorComponent, const COUNT: usize> Vector<T, COUNT> {
    pub fn from_arr(d: [T; COUNT]) -> Vector<T, COUNT> {
        Vector { data: d }
    }

    pub fn sum(&self) -> T {
        let mut sum = T::default();

        self.data.iter().for_each(|x| {
            sum += *x
        });

        return sum;
    }

    pub fn normalize(&self) -> Self {
        let s = self.sum();
        return self / s;
    }

    pub fn cross(&self, rhs : Self) -> Self {
        // Unless 4D or 2D cross products are a thing, we only support 3D cross products!
        // Sorry for such a weird hard coded quirk!
        return if COUNT == 3 {
            let mut p = Self::default();

            p[0] = self[1] * rhs[2] - self[2] * rhs[1];
            p[1] = self[2] * rhs[0] - self[0] * rhs[2];
            p[2] = self[0] * rhs[1] - self[1] * rhs[0];

            p
        } else {
            panic!("Cross products are only supported for 3 dimensional vectors! (Your vector has {} components!)", COUNT);
        }
    }
}

//
// Traits
//

//
// Default Trait
//
impl<T: VectorComponent, const COUNT: usize> Default for Vector<T, COUNT> {
    fn default() -> Self {
        let d: [T; COUNT] = [T::default(); COUNT];
        Self { data: d }
    }
}

//
// Indexing Traits
//
impl<T: VectorComponent, const COUNT: usize> Index<usize> for Vector<T, COUNT> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        return if index < self.data.len() {
            &self.data[index]
        } else {
            panic!("Index out of range!")
        }
    }
}

impl<T: VectorComponent, const COUNT: usize> IndexMut<usize> for Vector<T, COUNT> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return if index < self.data.len() {
            &mut self.data[index]
        } else {
            panic!("Index out of range!")
        }
    }
}

//
// Formatting Traits
//
impl<T: VectorComponent, const COUNT: usize> Display for Vector<T, COUNT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<");

        let mut first = true;
        for c in self.data {
            if !first {
                write!(f, ", ");
            }

            write!(f, "{c}");
            first = false;
        }
        write!(f, ">");

        Ok(())
    }
}

//
// Component Math Traits
//
impl<T: VectorComponent, const COUNT: usize> Div for Vector<T, COUNT> {