//! Miscellaneous traits.

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
/// let catenated = words.into_iter().fold_map(String::from_str);
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

impl<'a> Reducer<&'a str> for String {
    fn unit(value: &str) -> String {
        String::from_str(value)
    }
    fn combine_right(mut self, value: &str) -> String {
        self.push_str(value);
        self
    }
}

impl<'a, T: Clone> Reducer<&'a [T]> for Vec<T> {
    fn unit(value: &[T]) -> Vec<T> {
        let mut result = Vec::with_capacity(value.len());
        result.push_all(value);
        result
    }
    fn combine_right(mut self, value: &[T]) -> Vec<T> {
        self.push_all(value);
        self
    }
}

/// Trait for wrapper types.
///
/// A wrapper type ("newtype") is a struct with a single field.
pub trait Wrapper {
    type Inner;
    /// Wrap a value.
    fn from_inner(value: Self::Inner) -> Self;
    /// Unwrap a value.
    fn into_inner(self) -> Self::Inner;
}
