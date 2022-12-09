#![doc = include_str!("../README.md")]

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

    fn psw_factorial_with_array(&self) -> Option<Target>;

    fn small_factorial(&self) -> Option<Target>;
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

impl<T: PartialOrd + Unsigned + CheckedMul + Clone + FromPrimitive + ToPrimitive> Factorial<T>
    for T
{
    #[inline(always)]
    fn checked_factorial(&self) -> Option<T> {
        if self < &T::from_usize(array::SMALL_PRIME_SWING.len()).unwrap() {
            return self.psw_factorial_with_array();
        } else if self < &T::from_usize(1200).unwrap() {
            return self.small_factorial();
        }
        let sieve = Sieve::new(self.to_usize()?);
        self.psw_factorial(&sieve)
    }

    #[inline(always)]
    fn psw_factorial(&self, sieve: &Sieve) -> Option<T> {
        if self < &T::from_usize(array::SMALL_PRIME_SWING.len())? {
            return self.psw_factorial_with_array();
        }
        let first_term = Self::psw_factorial(&(self.clone() / T::from_usize(2)?), sieve)?;
        let swing = self.clone().prime_swing(sieve)?;
        first_term.checked_mul(&first_term)?.checked_mul(&swing)
    }
}

impl<T: PartialOrd + Unsigned + CheckedMul + Clone + FromPrimitive + ToPrimitive>
    PrivateFactorial<T> for T
{
    fn prime_swing(&self, sieve: &Sieve) -> Option<T> {
        let mut product = T::one();
        let two = T::from_usize(2)?;
        for prime in sieve
            .primes_from(2)
            .take_while(|x| *x < self.to_usize().unwrap())
        {
            let mut p = T::one();
            let mut q = self.clone();
            while q != T::zero() {
                q = q / T::from_usize(prime).unwrap();
                // q%2 == 1 if q is odd
                if q.clone() % two.clone() == T::one() {
                    p = p.checked_mul(&T::from_usize(prime).unwrap())?;
                }
            }
            if p > T::one() {
                product = product.checked_mul(&p)?;
            }
        }
        Some(product)
    }

    fn psw_factorial_with_array(&self) -> Option<T> {
        if self < &T::from_usize(array::SMALL_FACTORIAL.len()).unwrap() {
            // return Self::factorial(&self);
            return T::from_u128(array::SMALL_FACTORIAL[self.to_usize().unwrap()]);
        }
        let first_term = (self.clone() / T::from_usize(2).unwrap()).psw_factorial_with_array()?;
        let swing = T::from_u128(array::SMALL_PRIME_SWING[self.to_usize().unwrap()])?;
        first_term.checked_mul(&first_term)?.checked_mul(&swing)
    }

    fn small_factorial(&self) -> Option<T> {
        let small_prime_swing_limit = T::from_usize(array::SMALL_PRIME_SWING.len() - 1).unwrap();
        let mut acc = small_prime_swing_limit.psw_factorial_with_array()?;
        let mut i = small_prime_swing_limit + T::one();
        while &i <= self {
            acc = acc.checked_mul(&i)?;
            i = i + T::one();
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
}
