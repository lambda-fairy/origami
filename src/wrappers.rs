//! Various wrapper types.

use num::{One, Zero};
use std::ops::{Add, Mul};

use super::traits::*;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Product<T>(pub T);

impl<T> Wrapper for Product<T> {
    type Inner = T;
    fn from_inner(value: T) -> Product<T> { Product(value) }
    fn into_inner(self) -> T { self.0 }
}

impl<T: Mul<T, Output=T>> Semigroup for Product<T> {
    fn combine(self, other: Product<T>) -> Product<T> {
        Product(self.0 * other.0)
    }
}

impl<T: One> Monoid for Product<T> {
    fn unit() -> Product<T> { Product(One::one()) }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Sum<T>(pub T);

impl<T> Wrapper for Sum<T> {
    type Inner = T;
    fn from_inner(value: T) -> Sum<T> { Sum(value) }
    fn into_inner(self) -> T { self.0 }
}

impl<T: Add<T, Output=T>> Semigroup for Sum<T> {
    fn combine(self, other: Sum<T>) -> Sum<T> {
        Sum(self.0 + other.0)
    }
}

impl<T: Zero> Monoid for Sum<T> {
    fn unit() -> Sum<T> { Sum(Zero::zero()) }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct All(pub bool);

impl Wrapper for All {
    type Inner = bool;
    fn from_inner(value: bool) -> All { All(value) }
    fn into_inner(self) -> bool { self.0 }
}

impl Semigroup for All {
    fn combine(self, other: All) -> All {
        All(self.0 && other.0)
    }
}

impl Monoid for All {
    fn unit() -> All { All(true) }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Any(pub bool);

impl Wrapper for Any {
    type Inner = bool;
    fn from_inner(value: bool) -> Any { Any(value) }
    fn into_inner(self) -> bool { self.0 }
}

impl Semigroup for Any {
    fn combine(self, other: Any) -> Any {
        Any(self.0 || other.0)
    }
}

impl Monoid for Any {
    fn unit() -> Any { Any(false) }
}
