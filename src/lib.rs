//! # Compute the factorial
//!
//! This crate provides some convenient and safe methods to compute the factorial in O(n) time.
//!
//! They are not necessarily the fastest versions: there are prime sieve methods that
//! compute the factorial in `O(n (log n loglog n)^2)`. Patches are welcome.

#[cfg(test)]
extern crate num_bigint;
extern crate num_traits;

use num_traits::{CheckedMul, Signed, Unsigned};
use std::ops::RangeInclusive;

/// Unary operator for computing the factorial of a number
///
/// Implements checked and unchecked versions of the formula
pub trait Factorial<Target = Self> {
    /// Returns `self!`, i.e. the factorial of `self`,
    /// if it doesn't overflow the type `T`.
    ///
    /// # Examples
    /// ```
    /// use factorial::Factorial;
    /// assert_eq!(10u32.checked_factorial(), Some(3628800));
    /// ```
    fn checked_factorial(&self) -> Option<Target>;

    /// Returns `self!`, i.e. the factorial of `self`.
    ///
    /// # Examples
    /// ```
    /// use factorial::Factorial;
    /// assert_eq!(10u32.factorial(), 3628800);
    /// ```
    fn factorial(&self) -> Target {
        self.checked_factorial()
            .expect("Overflow computing factorial")
    }
}

impl<T: PartialOrd + Unsigned + CheckedMul> Factorial<T> for T {
    #[inline(always)]
    fn checked_factorial(&self) -> Option<T> {
        let mut acc = T::one();
        let mut i = T::one() + T::one();
        while i <= *self {
            if let Some(acc_i) = acc.checked_mul(&i) {
                acc = acc_i;
                i = i + T::one();
            } else {
                return None;
            }
        }
        Some(acc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::*;

    #[test]
    fn zero_fact_is_one() {
        assert_eq!(0u32.factorial(), 1u32);
    }

    #[test]
    fn one_fact_is_one() {
        assert_eq!(1.factorial(), 1u32);
    }

    #[test]
    fn two_fact_is_two() {
        assert_eq!(2.factorial(), 2u32);
    }

    #[test]
    fn ten_fact() {
        assert_eq!(10u32.factorial(), 3628800);
    }

    #[test]
    #[should_panic(expected = "Overflow computing factorial")]
    fn too_large() {
        100u32.factorial();
    }

    #[test]
    fn too_large_safe() {
        assert_eq!(100u32.checked_factorial(), None)
    }

    #[test]
    fn biguint_support() {
        assert_eq!(
            2u32.to_biguint().unwrap().factorial(),
            2u32.to_biguint().unwrap()
        );
    }
}