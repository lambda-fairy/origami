# Origami [![Build Status](https://img.shields.io/travis/lfairy/origami.svg)](http://travis-ci.org/lfairy/origami)

Folding utilities.

A [fold][] is when you reduce a collection down to a summary value. Many iterator methods, like `.count()`, `.sum()`, `.max()` etc., are folds.

However the folds in the standard library, while easy to use, are not very flexible. They work strictly from left-to-right, and cannot run in parallel.

This library provides an alternative formulation of these folds, in terms of [semigroups][] and [monoids][]. These structures decouple the core operation ("what") from the execution strategy ("how"), giving you more control over how your folds run.

[fold]: https://en.wikipedia.org/wiki/Fold_%28higher-order_function%29
[monoids]: https://en.wikipedia.org/wiki/Monoid
[semigroups]: https://en.wikipedia.org/wiki/Semigroup
