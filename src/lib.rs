//! # Compute the factorial
//!
//! This crate provides some convenient and safe methods to compute the
//! factorial and related functions the most naive way possible.
//!
//! They are not necessarily the fastest versions: there are prime sieve methods that
//! compute the factorial in `O(n (log n loglog n)^2)`. Patches are welcome.

#[cfg(test)]
#[macro_use]
extern crate approx;
#[cfg(test)]
extern crate num_bigint;

use num_traits::{CheckedMul, Float, FloatConst, Signed, Unsigned};

/// Unary operator for computing the factorial of an unsigned integer
///
/// Implements checked and unchecked versions of the formula
pub trait UnsignedFactorial<Target = Self> {
    /// Returns `self!`, i.e. the factorial of `self`,
    /// if it doesn't overflow the type `T`.
    ///
    /// # Examples
    /// ```
    /// use factorial::UnsignedFactorial;
    /// assert_eq!(10u32.checked_factorial(), Some(3628800));
    /// ```
    fn checked_factorial(&self) -> Option<Target>;

    /// Returns `self!`, i.e. the factorial of `self`.
    ///
    /// # Examples
    /// ```
    /// use factorial::UnsignedFactorial;
    /// assert_eq!(10u32.factorial(), 3628800);
    /// ```
    fn factorial(&self) -> Target {
        self.checked_factorial()
            .expect("Overflow computing factorial")
    }
}

/// Unary operator for computing the factorial of a signed integer
///
/// Implements checked and unchecked versions of the formula
pub trait SignedFactorial<Target = Self> {
    /// Returns `self!`, i.e. the factorial of `self`,
    /// if it doesn't overflow the type `T`.
    ///
    /// # Examples
    /// ```
    /// use factorial::SignedFactorial;
    /// assert_eq!(10i32.checked_factorial(), Some(3628800));
    /// ```
    fn checked_factorial(&self) -> Option<Target>;

    /// Returns `self!`, i.e. the factorial of `self`.
    ///
    /// # Examples
    /// ```
    /// use factorial::SignedFactorial;
    /// assert_eq!(10i32.factorial(), 3628800);
    /// ```
    fn factorial(&self) -> Target {
        self.checked_factorial()
            .expect("Overflow computing factorial")
    }
}

/// Unary operator for computing the double factorial of an unsigned integer
///
/// Implements checked and unchecked versions of the formula
pub trait UnsignedDoubleFactorial<Target = Self> {
    /// Returns `self!!`, i.e. the double factorial of `self`,
    /// if it doesn't overflow the type `T`.
    ///
    /// # Examples
    /// ```
    /// use factorial::UnsignedDoubleFactorial;
    /// assert_eq!(10u32.checked_double_factorial(), Some(3840));
    /// ```
    fn checked_double_factorial(&self) -> Option<Target>;

    /// Returns `self!!`, i.e. the double factorial of `self`.
    ///
    /// # Examples
    /// ```
    /// use factorial::UnsignedDoubleFactorial;
    /// assert_eq!(10u32.double_factorial(), 3840);
    /// ```
    fn double_factorial(&self) -> Target {
        self.checked_double_factorial()
            .expect("Overflow computing double factorial")
    }
}

/// Unary operator for computing the double factorial of a signed integer
///
/// Implements checked and unchecked versions of the formula
pub trait SignedDoubleFactorial<Target = Self> {
    /// Returns `self!!`, i.e. the double factorial of `self`,
    /// if it doesn't overflow the type `T`.
    ///
    /// # Examples
    /// ```
    /// use factorial::SignedDoubleFactorial;
    /// assert_eq!(10i32.checked_double_factorial(), Some(3840));
    /// ```
    fn checked_double_factorial(&self) -> Option<Target>;

    /// Returns `self!!`, i.e. the double factorial of `self`.
    ///
    /// # Examples
    /// ```
    /// use factorial::SignedDoubleFactorial;
    /// assert_eq!(10i32.double_factorial(), 3840);
    /// ```
    fn double_factorial(&self) -> Target {
        self.checked_double_factorial()
            .expect("Overflow computing double factorial")
    }
}

/// Unary operator for computing the double factorial of a floating-point number
///
/// Implements checked and unchecked versions of the formula
///
/// The double factorial can be extended to negative integer arguments,
/// resulting in non-integer values, based on the relation
///
/// \\[ (-n)!! \times n!! = (-1)^{\frac{n - 1}{2}} \times n \\]
///
/// The current implementation cannot accept continuous arguments where the
/// fractional part is non-zero.  See
/// https://en.wikipedia.org/wiki/Double_factorial#Negative_arguments for more
/// information.
pub trait FloatDoubleFactorial<Target = Self> {
    /// Returns `self!!`, i.e. the double factorial of `self`,
    /// if it doesn't overflow the type `T`.
    ///
    /// # Examples
    /// ```
    /// use factorial::FloatDoubleFactorial;
    /// assert_eq!((-5f32).checked_double_factorial(), Some(1.0 / 3.0));
    /// ```
    fn checked_double_factorial(&self) -> Option<Target>;

    /// Returns `self!!`, i.e. the double factorial of `self`.
    ///
    /// # Examples
    /// ```
    /// use factorial::FloatDoubleFactorial;
    /// assert_eq!((-5f32).double_factorial(), 1.0 / 3.0);
    /// ```
    fn double_factorial(&self) -> Target {
        self.checked_double_factorial()
            .expect("Overflow computing double factorial")
    }
}

impl<T: PartialOrd + Unsigned + CheckedMul> UnsignedFactorial<T> for T {
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

impl<T: PartialOrd + Signed + CheckedMul> SignedFactorial<T> for T {
    #[inline(always)]
    fn checked_factorial(&self) -> Option<T> {
        // The regular factorial is undefined for negative arguments.
        let zero = T::zero();
        if *self < zero {
            return None;
        }
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

impl<T: PartialOrd + Unsigned + CheckedMul + Copy> UnsignedDoubleFactorial<T> for T {
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

impl<T: PartialOrd + Signed + CheckedMul + Copy> SignedDoubleFactorial<T> for T {
    #[inline(always)]
    fn checked_double_factorial(&self) -> Option<T> {
        // Negative one and negative three double factorial are representable
        // as signed integers, but we choose not to.
        let zero = T::zero();
        if *self < zero {
            return None;
        }
        let one = T::one();
        let two = one + one;
        let mut acc = one;
        let mut i = if *self % two == zero { two } else { one };
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

impl<T: Float + FloatConst> FloatDoubleFactorial<T> for T {
    #[inline(always)]
    fn checked_double_factorial(&self) -> Option<T> {
        let zero = T::zero();
        if (*self).fract() != zero {
            return None;
        }
        let one = T::one();
        let two = one + one;
        if *self < zero {
            // The double factorial of a negative even argument is undefined
            if *self % two == zero {
                None
            } else {
                Some(*self * (-one).powf((*self - one) / two) / (-*self).double_factorial())
            }
        } else {
            let mut acc = T::one();
            let mut i = if *self % two == zero { two } else { one };
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
        assert_eq!(0i64.factorial(), 1i64);
    }

    #[test]
    fn one_fact_is_one() {
        assert_eq!(1u32.factorial(), 1u32);
        assert_eq!(1i64.factorial(), 1i64);
    }

    #[test]
    fn two_fact_is_two() {
        assert_eq!(2u32.factorial(), 2u32);
        assert_eq!(2i64.factorial(), 2i64);
    }

    #[test]
    fn ten_fact() {
        assert_eq!(10u32.factorial(), 3_628_800);
        assert_eq!(10i64.factorial(), 3_628_800);
    }

    #[test]
    #[should_panic(expected = "Overflow computing factorial")]
    fn too_large() {
        100u32.factorial();
    }

    #[test]
    fn too_large_safe() {
        assert_eq!(100u32.checked_factorial(), None);
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
    #[should_panic(expected = "Overflow computing factorial")]
    fn negative_fact_not_defined() {
        (-1i32).factorial();
    }

    #[test]
    fn zero_double_fact_is_one() {
        assert_eq!(0u32.double_factorial(), 1u32);
        assert_eq!(0i32.double_factorial(), 1i32);
        assert_abs_diff_eq!(0f32.double_factorial(), 1f32);
    }

    #[test]
    fn one_double_fact_is_one() {
        assert_eq!(1u32.double_factorial(), 1u32);
        assert_eq!(1i32.double_factorial(), 1i32);
        assert_abs_diff_eq!(1f32.double_factorial(), 1f32);
    }

    #[test]
    fn two_double_fact_is_two() {
        assert_eq!(2u32.double_factorial(), 2u32);
        assert_eq!(1i32.double_factorial(), 1i32);
        assert_abs_diff_eq!(2f32.double_factorial(), 2f32);
    }

    #[test]
    fn three_double_fact_is_three() {
        assert_eq!(3u32.double_factorial(), 3u32);
        assert_eq!(3i32.double_factorial(), 3i32);
        assert_abs_diff_eq!(3f32.double_factorial(), 3f32);
    }

    #[test]
    fn four_double_fact_is_eight() {
        assert_eq!(4u32.double_factorial(), 8u32);
        assert_eq!(4i32.double_factorial(), 8i32);
        assert_abs_diff_eq!(4f32.double_factorial(), 8f32);
    }

    #[test]
    fn five_double_fact_is_fifteen() {
        assert_eq!(5u32.double_factorial(), 15u32);
        assert_eq!(5i32.double_factorial(), 15i32);
        assert_eq!(5f32.double_factorial(), 15f32);
        assert_eq!(5u64.double_factorial(), 15u64);
        assert_eq!(5u128.double_factorial(), 15u128);
    }

    #[test]
    fn ten_double_fact() {
        assert_eq!(10u32.double_factorial(), 3840u32);
        assert_abs_diff_eq!(10f32.double_factorial(), 3840f32);
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
        assert_eq!(100u32.checked_double_factorial(), None);
    }

    #[test]
    fn negative_one_double_fact_is_one() {
        assert_abs_diff_eq!((-1f32).double_factorial(), 1f32);
        assert_abs_diff_eq!((-1f64).double_factorial(), 1f64);
        // This is representable with the input type, but we choose not to.
        assert_eq!((-1i32).checked_double_factorial(), None);
    }

    #[test]
    fn negative_three_double_fact_is_negative_one() {
        assert_abs_diff_eq!((-3f32).double_factorial(), -1f32);
        assert_abs_diff_eq!((-3f64).double_factorial(), -1f64, epsilon = 1.0e-15);
        // This is representable with the input type, but we choose not to.
        assert_eq!((-3i32).checked_double_factorial(), None);
    }

    #[test]
    fn negative_even_double_factorials_are_undefined() {
        assert_eq!((-4i32).checked_double_factorial(), None);
        assert_eq!((-2f32).checked_double_factorial(), None);
        assert_eq!((-4f32).checked_double_factorial(), None);
        assert_eq!((-6f32).checked_double_factorial(), None);
    }

    #[test]
    fn negative_five_double_fact_is_one_third() {
        assert_abs_diff_eq!((-5f32).double_factorial(), (1.0 / 3.0));
        assert_abs_diff_eq!((-5f64).double_factorial(), (1.0 / 3.0), epsilon = 1.0e-15);
        // We can no longer represent the output with the same type as the
        // input.
        assert_eq!((-5i32).checked_double_factorial(), None);
    }

    #[test]
    fn negative_nineteen_double_fact() {
        assert_abs_diff_eq!((-19f32).double_factorial(), (-1.0 / 34_459_425.0));
    }

    #[test]
    #[should_panic(expected = "Overflow computing double factorial")]
    fn negative_even_double_fact_is_undefined() {
        (-2f32).double_factorial();
    }

    #[test]
    #[should_panic(expected = "Overflow computing double factorial")]
    fn fractional_double_factorial_is_undefined() {
        (3.4).double_factorial();
    }
}
