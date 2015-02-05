//! Iterator adaptors.

use super::traits::*;

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
    fn fold_monoid(self) -> Self::Item
        where Self::Item: Monoid
    {
        self.fold(Monoid::unit(), Semigroup::combine)
    }

    /// Fold a non-empty iterator using the combining operation.
    ///
    /// Returns `None` on an empty sequence.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use origami::iter::IteratorFoldExt;
    /// # use origami::wrappers::Product;
    /// let nums = vec![Product(1), Product(2), Product(3)];
    /// let product = nums.into_iter().fold_nonempty();
    /// assert_eq!(product, Some(Product(6)));
    /// ```
    fn fold_nonempty(mut self) -> Option<Self::Item>
        where Self::Item: Semigroup
    {
        match self.next() {
            None => None,
            Some(first) => Some(self.fold(first, Semigroup::combine)),
        }
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
    fn fold_map<F, A>(self, f: F) -> A
        where F: FnMut(Self::Item) -> A, A: Monoid
    {
        self.map(f).fold_monoid()
    }

    /// Map each element to a semigroup, then combine the results.
    ///
    /// Returns `None` on an empty sequence.
    fn fold_map_nonempty<F, A>(self, f: F) -> Option<A>
        where F: FnMut(Self::Item) -> A, A: Semigroup
    {
        self.map(f).fold_nonempty()
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
    fn fold_reduce<R>(mut self) -> R
        where R: Reducer<Self::Item> + Monoid
    {
        match self.next() {
            None => Monoid::unit(),
            Some(first) => self.fold(Reducer::unit(first), Reducer::combine_right),
        }
    }

    /// Map each element to a reducer, then combine the results.
    ///
    /// Returns `None` on an empty sequence.
    fn fold_reduce_nonempty<R>(mut self) -> Option<R>
        where R: Reducer<Self::Item>
    {
        match self.next() {
            None => None,
            Some(first) => Some(self.fold(Reducer::unit(first), Reducer::combine_right)),
        }
    }
}

impl<I> IteratorFoldExt for I where I: Iterator + Sized {}
