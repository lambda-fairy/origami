//! Miscellaneous traits.

use std::borrow::ToOwned;

/// A [semigroup](https://en.wikipedia.org/wiki/Semigroup) is a type
/// with a combining operation.
///
/// There is one catch—the combining operation has to be *associative*.
/// In other words, for *all* values `a`, `b` and `c`, this equality
/// must hold:
///
/// ```ignore
/// combine(combine(a, b), c) == combine(a, combine(b, c))
/// ```
///
/// Many types form semigroups, often in more than one way. For example,
/// integers form a semigroup under either addition or multiplication.
/// When this happens you can fix the implementation using a wrapper
/// type: `Product` or `Sum`.
pub trait Semigroup {
    fn combine(self, other: Self) -> Self;
}

impl Semigroup for () {
    fn combine(self, _other: ()) -> () {
        ()
    }
}

impl<A: Semigroup> Semigroup for (A,) {
    fn combine(self, other: (A,)) -> (A,) {
        (self.0.combine(other.0),)
    }
}

impl<A: Semigroup, B: Semigroup> Semigroup for (A, B) {
    fn combine(self, other: (A, B)) -> (A, B) {
        (self.0.combine(other.0), self.1.combine(other.1))
    }
}

impl<A: Semigroup, B: Semigroup, C: Semigroup> Semigroup for (A, B, C) {
    fn combine(self, other: (A, B, C)) -> (A, B, C) {
        (self.0.combine(other.0), self.1.combine(other.1), self.2.combine(other.2))
    }
}

/// Combine the underlying `Some` values, ignoring `None`s.
impl<T: Semigroup> Semigroup for Option<T> {
    fn combine(self, other: Option<T>) -> Option<T> {
        match self {
            None => other,
            Some(x) => match other {
                None => Some(x),
                Some(y) => Some(x.combine(y)),
            },
        }
    }
}

impl Semigroup for String {
    fn combine(mut self, other: String) -> String {
        self.push_str(&other);
        self
    }
}

impl<T> Semigroup for Vec<T> {
    fn combine(mut self, other: Vec<T>) -> Vec<T> {
        self.extend(other.into_iter());
        self
    }
}

/// A [monoid](https://en.wikipedia.org/wiki/Monoid) is a semigroup with
/// an empty value, or *unit*.
///
/// Combining any value with this unit leaves the value unchanged.
///
/// In pseudo-Rust:
///
/// ```ignore
/// combine(unit(), x) == x
/// combine(x, unit()) == x
/// ```
pub trait Monoid: Semigroup {
    /// Return the unit value.
    fn unit() -> Self;
}

impl Monoid for () {
    fn unit() -> () {
        ()
    }
}

impl<A: Monoid> Monoid for (A,) {
    fn unit() -> (A,) {
        (Monoid::unit(),)
    }
}

impl<A: Monoid, B: Monoid> Monoid for (A, B) {
    fn unit() -> (A, B) {
        (Monoid::unit(), Monoid::unit())
    }
}

impl<A: Monoid, B: Monoid, C: Monoid> Monoid for (A, B, C) {
    fn unit() -> (A, B, C) {
        (Monoid::unit(), Monoid::unit(), Monoid::unit())
    }
}

/// The unit of `Option<T>` is `None`.
impl<T: Semigroup> Monoid for Option<T> {
    fn unit() -> Option<T> {
        None
    }
}

impl Monoid for String {
    fn unit() -> String {
        String::new()
    }
}

impl<T> Monoid for Vec<T> {
    fn unit() -> Vec<T> {
        Vec::new()
    }
}

/// A `Reducer<T>` is a `Semigroup` which has a canonical mapping from
/// `T`.
///
/// For example, say you have a list of strings:
///
/// ```ignore
/// let words = vec!["pickle", "barrel", "kumquat"];
/// ```
///
/// If you wanted to concatenate them, you could map each element to a
/// `String` then fold them up:
///
/// ```ignore
/// let catenated = words.into_iter().fold_map(ToString::to_string);
/// ```
///
/// but that allocates too much and runs in quadratic time.
///
/// The equivalent using `Reducer` runs much faster:
///
/// ```ignore
/// let catenated = words.into_iter().fold_reduce::<String>();
/// ```
///
/// Instead of repeatedly allocating `String`s, the `Reducer` creates a
/// single `String` and pushes to it using `.combine_right()`.
pub trait Reducer<T>: Semigroup + Sized {
    fn unit(value: T) -> Self;
    fn combine_left(self, value: T) -> Self {
        <Self as Reducer<T>>::unit(value).combine(self)
    }
    fn combine_right(self, value: T) -> Self {
        self.combine(Reducer::unit(value))
    }
}

impl<T, R: Reducer<T>> Reducer<T> for Option<R> {
    fn unit(value: T) -> Option<R> {
        Some(Reducer::unit(value))
    }
    fn combine_left(self, value: T) -> Option<R> {
        match self {
            None => Some(Reducer::unit(value)),
            Some(r) => Some(r.combine_left(value)),
        }
    }
    fn combine_right(self, value: T) -> Option<R> {
        match self {
            None => Some(Reducer::unit(value)),
            Some(r) => Some(r.combine_right(value)),
        }
    }
}

impl<'a> Reducer<&'a str> for String {
    fn unit(value: &str) -> String {
        value.to_owned()
    }
    fn combine_right(mut self, value: &str) -> String {
        self.push_str(value);
        self
    }
}

impl<'a, T: Clone> Reducer<&'a [T]> for Vec<T> {
    fn unit(value: &[T]) -> Vec<T> {
        value.to_owned()
    }
    fn combine_right(mut self, value: &[T]) -> Vec<T> {
        self.extend(value.iter().cloned());
        self
    }
}
