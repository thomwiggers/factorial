//! # Compute the factorial
//!
//! This crate provides some convenient and safe methods to compute the
//! factorial and related functions the most naive way possible.
//!
//! They are not necessarily the fastest versions: there are prime sieve methods that
//! compute the factorial in `O(n (log n loglog n)^2)`. Patches are welcome.

#[cfg(test)]
extern crate num_bigint;
extern crate num_traits;

use num_traits::{CheckedMul, FromPrimitive, ToPrimitive, Unsigned};
use primal_sieve::Sieve;

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

    fn prime_swing(n: Target, sieve: &Sieve) -> Target;

    /// Returns `self!`, i.e. the factorial of `self` using the prime swing algorithm.
    ///
    /// # Examples
    /// ```
    /// use factorial::Factorial;
    /// use primal_sieve::Sieve;
    /// let number = 10_usize;
    /// let sieve = Sieve::new(number);
    /// assert_eq!(10_usize.factorial(), 3628800);
    /// ```
    fn psw_factorial(&self, sieve: &Sieve) -> Target;
}

/// Unary operator for computing the double factorial of a number
///
/// Implements checked and unchecked versions of the formula
pub trait DoubleFactorial<Target = Self> {
    fn checked_double_factorial(&self) -> Option<Target>;

    fn double_factorial(&self) -> Target {
        self.checked_double_factorial()
            .expect("Overflow computing double factorial")
    }
}

impl<
        T: PartialOrd
            + Unsigned
            + CheckedMul
            + std::iter::Product
            + Clone
            + FromPrimitive
            + ToPrimitive,
    > Factorial<T> for T
{
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

    fn prime_swing(n: T, sieve: &Sieve) -> T {
        let mut vec = vec![];
        for prime in sieve
            .primes_from(2)
            .take_while(|x| *x <= n.clone().to_usize().unwrap())
        {
            let mut p = T::one();
            let mut q = n.clone();
            while q != T::zero() {
                q = q / T::from_usize(prime).unwrap();
                // q&1 == 1 if q is odd
                if q.clone() % T::from_usize(2).unwrap() == T::one() {
                    p = p * T::from_usize(prime).unwrap();
                }
            }
            if p > T::one() {
                vec.push(p);
            }
        }
        vec.into_iter().product()
    }

    fn psw_factorial(&self, sieve: &Sieve) -> T {
        if *self < T::from_usize(2).unwrap() {
            return T::one();
        }
        let first_term = Self::psw_factorial(&(self.clone() / T::from_usize(2).unwrap()), sieve);
        first_term.clone() * first_term.clone() * Self::prime_swing(self.clone(), sieve)
    }
}

impl<T: PartialOrd + Unsigned + CheckedMul + Copy> DoubleFactorial<T> for T {
    #[inline(always)]
    fn checked_double_factorial(&self) -> Option<T> {
        let one = T::one();
        let two = one + one;
        let mut acc = one;
        let mut i = if *self % two == T::zero() { two } else { one };
        while i <= *self {
            if let Some(acc_i) = acc.checked_mul(&i) {
                acc = acc_i;
                i = i + two;
            } else {
                return None;
            }
        }
        Some(acc)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

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
        assert_eq!(10u32.factorial(), 3_628_800);
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
        assert_eq!(
            2u32.to_biguint().unwrap().checked_factorial(),
            Some(2u32.to_biguint().unwrap())
        );
    }

    #[test]
    fn zero_psw_fact_is_one() {
        let sieve = Sieve::new(1);
        assert_eq!(0u32.psw_factorial(&sieve), 1u32);
    }

    #[test]
    fn one_psw_fact_is_one() {
        let sieve = Sieve::new(2);
        assert_eq!(1.psw_factorial(&sieve), 1u32);
    }

    #[test]
    fn two_psw_fact_is_two() {
        let sieve = Sieve::new(3);
        assert_eq!(2.psw_factorial(&sieve), 2u32);
    }

    #[test]
    fn ten_psw_fact() {
        let sieve = Sieve::new(10);
        assert_eq!(10u32.psw_factorial(&sieve), 3_628_800);
    }

    #[test]
    fn psw_biguint_support() {
        let sieve = Sieve::new(2);
        assert_eq!(
            2u32.to_biguint().unwrap().psw_factorial(&sieve),
            2u32.to_biguint().unwrap()
        );
    }

    #[test]
    fn psw_speed_test() {
        let time_fac = Instant::now();
        let fac = 1000_usize.to_biguint().unwrap().factorial();
        let time_fac = time_fac.elapsed().as_micros();

        let time_psw_fac = Instant::now();
        let sieve = Sieve::new(1000);
        let psw_fac = 1000_usize.to_biguint().unwrap().psw_factorial(&sieve);
        let time_psw_fac = time_psw_fac.elapsed().as_micros();

        assert_eq!(fac, psw_fac);
        println!("{} > {}", time_fac, time_psw_fac);
        assert!(time_fac > time_psw_fac);
    }

    #[test]
    fn zero_double_fact_is_one() {
        assert_eq!(0.double_factorial(), 1u32)
    }

    #[test]
    fn one_double_fact_is_two() {
        assert_eq!(1.double_factorial(), 1u32)
    }

    #[test]
    fn two_double_fact_is_two() {
        assert_eq!(2.double_factorial(), 2u32)
    }

    #[test]
    fn ten_double_fact() {
        assert_eq!(10u32.double_factorial(), 3840u32);
    }

    #[test]
    fn seven_double_fact() {
        assert_eq!(7u32.double_factorial(), 105u32);
    }

    #[test]
    #[should_panic(expected = "Overflow computing double factorial")]
    fn too_large_double_fact() {
        100u32.double_factorial();
    }

    #[test]
    fn too_large_safe_double_fact() {
        assert_eq!(100u32.checked_double_factorial(), None)
    }
}
