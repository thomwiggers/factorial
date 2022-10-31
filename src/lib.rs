//! # Compute the factorial
//!
//! This crate provides some convenient and safe methods to compute the
//! factorial and related functions the most naive way possible.
//!
//! They are not necessarily the fastest versions: there are prime sieve methods that
//! compute the factorial in `O(n (log n loglog n)^2)`. Patches are welcome.

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

const SMALL_FACTORIAL: [usize; 20] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600, 6227020800, 87178291200, 1307674368000, 20922789888000, 355687428096000, 6402373705728000, 121645100408832000];

const SMALL_PRIME_SWING: [usize; 63] = [1, 1, 2, 6, 6, 30, 20, 140, 70, 630, 252, 2772, 924, 12012, 3432, 51480, 12870, 218790, 48620, 923780, 184756, 3879876, 705432, 16224936, 2704156, 67603900, 10400600, 280816200, 40116600, 1163381400, 155117520, 4808643120, 601080390, 19835652870, 2333606220, 81676217700, 9075135300, 335780006100, 35345263800, 1378465288200, 137846528820, 5651707681620, 538257874440, 23145088600920, 2104098963720, 94684453367400, 8233430727600, 386971244197200, 32247603683100, 1580132580471900, 126410606437752, 6446940928325352, 495918532948104, 26283682246249512, 1946939425648112, 107081668410646160, 7648690600760440, 435975364243345080, 30067266499541040, 1773968723472921360, 118264581564861424, 7214139475456546864, 465428353255261088];


impl<T: PartialOrd + Unsigned + CheckedMul + Clone + FromPrimitive + ToPrimitive> Factorial<T>
    for T
{
    #[inline(always)]
    fn checked_factorial(&self) -> Option<T> {
        let mut acc = T::one();
        let mut i = T::from_usize(2).unwrap();
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
        let mut product = T::one();
        let two = T::from_usize(2).unwrap();
        for prime in sieve
            .primes_from(2)
            .take_while(|x| *x < n.to_usize().unwrap())
        {
            let mut p = T::one();
            let mut q = n.clone();
            while q != T::zero() {
                q = q / T::from_usize(prime).unwrap();
                // q%2 == 1 if q is odd
                if q.clone() % two.clone() == T::one() {
                    p = p * T::from_usize(prime).unwrap();
                }
            }
            if p > T::one() {
                product = product * p;
            }
        }
        product
    }

    fn psw_factorial(&self, sieve: &Sieve) -> T {
        if *self < T::from_usize(20).unwrap() {
            // return Self::factorial(&self);
            return T::from_usize(SMALL_FACTORIAL[self.to_usize().unwrap()]).unwrap();
        }
        let first_term = Self::psw_factorial(&(self.clone() / T::from_usize(2).unwrap()), sieve);
        let swing = if *self < T::from_usize(63).unwrap() {
            T::from_usize(SMALL_PRIME_SWING[self.to_usize().unwrap()]).unwrap()
        } else {
            Self::prime_swing(self.clone(), sieve)
        };
        first_term.clone() * first_term.clone() * swing
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
    use std::time::Instant;

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

        let sieve = Sieve::new(1000);
        let time_psw_fac = Instant::now();
        let psw_fac = 1000_usize.to_biguint().unwrap().psw_factorial(&sieve);
        let time_psw_fac = time_psw_fac.elapsed().as_micros();

        assert_eq!(fac, psw_fac);
        println!("{} >=? {}", time_fac, time_psw_fac);
        assert!(time_fac >= time_psw_fac);
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
