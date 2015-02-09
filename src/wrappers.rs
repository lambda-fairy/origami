//! Various wrapper types.

use std::cmp;
use num::{Bounded, One, Zero};
use std::ops::{Add, Mul};

use super::traits::*;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Product<T>(pub T);

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

impl Semigroup for Any {
    fn combine(self, other: Any) -> Any {
        Any(self.0 || other.0)
    }
}

impl Monoid for Any {
    fn unit() -> Any { Any(false) }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Min<T>(pub T);

impl<T: Ord> Semigroup for Min<T> {
    fn combine(self, other: Min<T>) -> Min<T> {
        Min(cmp::min(self.0, other.0))
    }
}

impl<T: Bounded + Ord> Monoid for Min<T> {
    fn unit() -> Min<T> {
        Min(Bounded::max_value())
    }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Max<T>(pub T);

impl<T: Ord> Semigroup for Max<T> {
    fn combine(self, other: Max<T>) -> Max<T> {
        Max(cmp::max(self.0, other.0))
    }
}

impl<T: Bounded + Ord> Monoid for Max<T> {
    fn unit() -> Max<T> {
        Max(Bounded::min_value())
    }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct First<T>(pub T);

impl<T: Ord> Semigroup for First<T> {
    fn combine(self, _other: First<T>) -> First<T> {
        self
    }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Last<T>(pub T);

impl<T: Ord> Semigroup for Last<T> {
    fn combine(self, other: Last<T>) -> Last<T> {
        other
    }
}
