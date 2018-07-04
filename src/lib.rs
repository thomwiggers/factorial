//! # Compute the factorial
//! 
//! This crate provides some convenient and safe methods to compute the factorial in O(n) time.
//!
//! They are not necessarily the fastest versions: there are prime sieve methods that
//! compute the factorial in `O(n (log n loglog n)^2)`. Patches are welcome.

extern crate num_traits;

use num_traits::{CheckedMul, Unsigned};
use std::ops::RangeInclusive;

/// Unary operator for computing the factorial of a number
///
/// Implements checked and unchecked versions of the formula
pub trait Factorial: Sized {

    /// Returns `self!`, i.e. the factorial of `self`,
    /// if it doesn't overflow the type `T`.
    ///
    /// # Examples
    /// ```
    /// use factorial::Factorial;
    /// assert_eq!(10u32.checked_factorial(), Some(3628800));
    /// ```
    fn checked_factorial(self) -> Option<Self>;

    /// Returns `self!`, i.e. the factorial of `self`.
    ///
    /// # Examples
    /// ```
    /// use factorial::Factorial;
    /// assert_eq!(10u32.factorial(), 3628800);
    /// ```
    fn factorial(self) -> Self {
        self.checked_factorial().expect("Overflow computing factorial")
    }
}

impl<T: Unsigned + CheckedMul> Factorial for T 
    where RangeInclusive<T>: IntoIterator<Item=T> {

    #[inline(always)]
    fn checked_factorial(self) -> Option<T> 
    {
        (T::one()..=self).into_iter().try_fold(T::one(), |acc, i| acc.checked_mul(&i))
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

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
}
