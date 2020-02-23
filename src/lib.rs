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
extern crate rgsl;

use num_traits::{CheckedMul, Float, FloatConst, FromPrimitive, Unsigned};

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

pub trait SignedDoubleFactorial<Target = Self> {
    fn checked_double_factorial(&self) -> Option<Target>;

    fn double_factorial(&self) -> Target {
        self.checked_double_factorial()
            .expect("Overflow computing double factorial")
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

impl<T: Float + FloatConst + FromPrimitive> SignedDoubleFactorial<T> for T
where
    f64: From<T>,
{
    #[inline(always)]
    fn checked_double_factorial(&self) -> Option<T> {
        let two = T::one() + T::one();
        if *self < T::zero() {
            if *self % two == T::zero() {
                return None;
            }
            // Calculate the answer directly.
            let n = (*self + T::one()) / two;
            let n_64: f64 = f64::from(n);
            let n_plus_half = n_64 + 0.5;
            Some(
                two.powf(n) * T::from_f64(rgsl::gamma_beta::gamma::gamma(n_plus_half)).unwrap()
                    / T::PI().powf(T::from_f32(0.5).unwrap()),
            )
        } else {
            let mut acc = T::one();
            let mut i = two;
            while i <= *self {
                acc = acc * i;
                i = i + two;
            }
            Some(acc)
        }
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
    fn zero_double_fact_is_one() {
        assert_eq!(0u32.double_factorial(), 1u32)
    }

    #[test]
    fn one_double_fact_is_two() {
        assert_eq!(1u32.double_factorial(), 1u32)
    }

    #[test]
    fn two_double_fact_is_two() {
        assert_eq!(2u32.double_factorial(), 2u32)
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

    #[test]
    fn negative_one_double_fact_is_one() {
        assert_eq!((-1.0_f32).double_factorial(), 1.0_f32);
        assert_eq!((-1.0_f64).double_factorial(), 1.0_f64);
    }

    #[test]
    fn negative_three_double_fact_is_negative_one() {
        assert_eq!((-3.0_f32).double_factorial(), -1.0_f32);
        assert_eq!((-3.0_f64).double_factorial(), -1.0_f64);
    }

    #[test]
    fn negative_five_double_fact_is_one_third() {
        assert_eq!((-5.0_f32).double_factorial(), (1.0 / 3.0));
        assert_eq!((-5.0_f64).double_factorial(), (1.0 / 3.0));
    }

    #[test]
    fn negative_nineteen_double_fact() {
        assert_eq!((-19_f32).double_factorial(), (-1.0 / 34459425.0));
    }

    #[test]
    #[should_panic(expected = "Overflow computing double factorial")]
    fn negative_even_double_fact_is_undefined() {
        (-2.0_f32).double_factorial();
    }
}
