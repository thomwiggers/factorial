#![doc = include_str!("../README.md")]

use num_traits::{CheckedMul, FromPrimitive, ToPrimitive, Unsigned};
use primal_sieve::Sieve;
use std::ops::Shl;

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

    /// Returns `self!`, i.e. the factorial of `self` using the prime swing algorithm.
    ///
    /// # Examples
    /// ```
    /// use factorial::Factorial;
    /// use primal_sieve::Sieve;
    /// // The sieve must be equal or greater than the argument of the factorial.
    /// let sieve = Sieve::new(10_usize);
    /// assert_eq!(10_usize.factorial(), 3628800);
    /// ```
    fn psw_factorial(&self, sieve: &Sieve) -> Option<Target>;
}

trait PrivateFactorial<Target = Self> {
    fn prime_swing(&self, sieve: &Sieve) -> Option<Target>;

    fn odd_factorial(&self, sieve: &Sieve) -> Option<Target>;

    fn odd_factorial_array(&self) -> Option<Target>;

    fn psw_factorial_with_array(&self) -> Option<Target>;
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

mod array;

fn prime_range(
    sieve: &Sieve,
    lower_bound: usize,
    upper_boud: usize,
) -> impl Iterator<Item = usize> + '_ {
    sieve
        .primes_from(lower_bound)
        .take_while(move |m| *m <= upper_boud)
}

impl<
        T: PartialOrd
            + Unsigned
            + CheckedMul
            + Clone
            + FromPrimitive
            + ToPrimitive
            + Shl<u32, Output = T>,
    > Factorial<T> for T
{
    #[inline(always)]
    fn checked_factorial(&self) -> Option<T> {
        if self < &T::from_usize(array::SMALL_ODD_SWING.len()).unwrap() {
            return self.psw_factorial_with_array();
        }
        let sieve = Sieve::new(self.to_usize()?);
        self.psw_factorial(&sieve)
    }

    #[inline(always)]
    fn psw_factorial(&self, sieve: &Sieve) -> Option<T> {
        if self < &T::from_usize(array::SMALL_ODD_SWING.len())? {
            return self.psw_factorial_with_array();
        }
        let bytes = self.to_u32()? - self.to_u32()?.count_ones() - 1;
        let res = self.odd_factorial(sieve)?;
        res.checked_mul(&T::from_u8(2)?.shl(bytes))
    }
}

impl<
        T: PartialOrd
            + Unsigned
            + CheckedMul
            + Clone
            + FromPrimitive
            + ToPrimitive
            + Shl<u32, Output = T>,
    > PrivateFactorial<T> for T
{
    fn prime_swing(&self, sieve: &Sieve) -> Option<T> {
        let n = self.to_usize()?;
        if n < array::SMALL_ODD_SWING.len() {
            return T::from_u128(array::SMALL_ODD_SWING[n]);
        }
        let sqrt = ((n as f64).sqrt().floor()) as usize;
        let mut product = T::one();

        for prime in prime_range(sieve, n / 2 + 1, n) {
            product = product.checked_mul(&T::from_usize(prime)?)?;
        }

        for prime in prime_range(sieve, sqrt + 1, n / 3) {
            if (n / prime) & 1 == 1 {
                product = product.checked_mul(&T::from_usize(prime)?)?;
            }
        }

        for prime in prime_range(sieve, 3, sqrt) {
            let mut p = 1;
            let mut q = n;
            loop {
                q /= prime;
                if q == 0 {
                    break;
                }
                if q & 1 == 1 {
                    p *= prime;
                }
            }
            if p > 1 {
                product = product.checked_mul(&T::from_usize(p)?)?;
            }
        }
        Some(product)
    }

    fn odd_factorial(&self, sieve: &Sieve) -> Option<T> {
        let two = T::from_u8(2).unwrap();
        if self < &(two) {
            return Some(Self::one());
        }
        let tmp = (self.clone() / two).odd_factorial(sieve)?;
        let tmp_sq = tmp.checked_mul(&tmp)?;
        tmp_sq.checked_mul(&self.prime_swing(sieve)?)
    }

    fn odd_factorial_array(&self) -> Option<T> {
        let two = T::from_u8(2).unwrap();
        if self < &(two) {
            return Some(Self::one());
        }
        let tmp = (self.clone() / two).odd_factorial_array()?;
        let tmp_sq = tmp.checked_mul(&tmp)?;
        tmp_sq.checked_mul(&T::from_u128(array::SMALL_ODD_SWING[self.to_usize()?])?)
    }

    fn psw_factorial_with_array(&self) -> Option<T> {
        if self < &T::from_usize(array::SMALL_FACTORIAL.len()).unwrap() {
            return T::from_u128(array::SMALL_FACTORIAL[self.to_usize().unwrap()]);
        }
        let bytes = self.to_u32()? - self.to_u32()?.count_ones() - 1;
        let res = self.odd_factorial_array()?;
        res.checked_mul(&T::from_u8(2)?.shl(bytes))
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
    use crate::{DoubleFactorial, Factorial};
    use num_bigint::*;
    use primal_sieve::Sieve;

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
    fn one_hundred_fact() {
        let sieve = Sieve::new(100);
        assert_eq!(
            100.to_biguint().unwrap().factorial(),
            100.to_biguint().unwrap().psw_factorial(&sieve).unwrap()
        );
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

    #[test]
    fn factorials_range() {
        for n in 2..=34 {
            let p = n.factorial();
            let mut p_prime = 1u128;
            for i in 2..=n {
                p_prime *= i;
            }
            assert_eq!(p_prime, p, "mismatch for iteration {n}");
        }
    }

    #[test]
    fn psw_factorials_range_bigint() {
        let sieve = Sieve::new(2000);
        for n in 2..=2000u128 {
            let p = n.to_biguint().unwrap().psw_factorial(&sieve).unwrap();
            let mut p_prime = 1u128.to_biguint().unwrap();
            for i in 2..=n {
                p_prime *= i.to_biguint().unwrap();
            }
            assert_eq!(p_prime, p, "mismatch for iteration {n}");
        }
    }

    #[test]
    fn crazy_big_factorial() {
        let sieve = Sieve::new(8000);
        let n = 8000;
        let p = n.to_biguint().unwrap().psw_factorial(&sieve).unwrap();
        let mut p_prime = 1u128.to_biguint().unwrap();
        for i in 2..=n {
            p_prime *= i.to_biguint().unwrap();
        }
        assert_eq!(p_prime, p, "mismatch for iteration {n}");
    }
}
