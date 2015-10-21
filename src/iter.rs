//! Iterator adaptors.

use traits::*;

/// Extends the `Iterator` trait with origami superpowers.
pub trait IteratorFoldExt: Iterator + Sized {
    /// Fold the iterator using the combining operation.
    ///
    /// Returns `Monoid::unit()` on an empty sequence.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use origami::iter::IteratorFoldExt;
    /// # use origami::wrappers::Sum;
    /// let nums = vec![Sum(1), Sum(2), Sum(3)];
    /// let sum = nums.into_iter().fold_monoid();
    /// assert_eq!(sum, Sum(6));
    /// ```
    fn fold_monoid(self) -> Self::Item where
        Self::Item: Monoid
    {
        self.fold(Monoid::unit(), Semigroup::combine)
    }

    /// Map each element to a monoid, then combine the results.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use origami::iter::IteratorFoldExt;
    /// # use origami::wrappers::All;
    /// let nums = vec![true, false, true];
    /// let all_true = nums.into_iter().fold_map(All).0;
    /// assert_eq!(all_true, false);
    /// ```
    fn fold_map<F, A>(self, f: F) -> A where
        F: FnMut(Self::Item) -> A, A: Monoid
    {
        self.map(f).fold_monoid()
    }

    /// Map each element to a reducer, then combine the results.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use origami::iter::IteratorFoldExt;
    /// let names = vec!["Applejack", "Fluttershy", "Rarity"];
    /// let result = names.into_iter().fold_reduce::<String>();
    /// assert_eq!(result, "ApplejackFluttershyRarity");
    /// ```
    fn fold_reduce<R>(mut self) -> R where
        R: Reducer<Self::Item> + Monoid
    {
        match self.next() {
            None => Monoid::unit(),
            Some(first) => self.fold(Reducer::start(first), Reducer::combine_right),
        }
    }
}

impl<I> IteratorFoldExt for I where I: Iterator + Sized {}
