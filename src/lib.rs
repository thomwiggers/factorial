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

    fn checked_naive_factorial(&self) -> Option<Target>;

    fn naive_factorial(&self) -> Target {
        self.checked_naive_factorial()
            .expect("Overflow computing factorial")
    }
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
    fn psw_factorial(&self, sieve: &Sieve) -> Option<Target>;
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

#[rustfmt::skip]
const SMALL_FACTORIAL: [usize; 20] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600, 6227020800, 87178291200, 1307674368000, 20922789888000, 355687428096000, 6402373705728000, 121645100408832000];
#[rustfmt::skip]
const SMALL_PRIME_SWING: [usize; 63] = [1, 1, 2, 6, 6, 30, 20, 140, 70, 630, 252, 2772, 924, 12012, 3432, 51480, 12870, 218790, 48620, 923780, 184756, 3879876, 705432, 16224936, 2704156, 67603900, 10400600, 280816200, 40116600, 1163381400, 155117520, 4808643120, 601080390, 19835652870, 2333606220, 81676217700, 9075135300, 335780006100, 35345263800, 1378465288200, 137846528820, 5651707681620, 538257874440, 23145088600920, 2104098963720, 94684453367400, 8233430727600, 386971244197200, 32247603683100, 1580132580471900, 126410606437752, 6446940928325352, 495918532948104, 26283682246249512, 1946939425648112, 107081668410646160, 7648690600760440, 435975364243345080, 30067266499541040, 1773968723472921360, 118264581564861424, 7214139475456546864, 465428353255261088];

impl<T: PartialOrd + Unsigned + CheckedMul + Clone + FromPrimitive + ToPrimitive> Factorial<T>
    for T
{
    #[inline(always)]
    fn checked_factorial(&self) -> Option<T> {
        if self < &T::from_usize(SMALL_PRIME_SWING.len()).unwrap() {
            return psw_factorial_with_array(self);
        } else if self < &T::from_usize(800).unwrap() {
            return small_factorial(self);
        }
        let sieve = Sieve::new(self.to_usize()?);
        self.psw_factorial(&sieve)
    }

    #[inline(always)]
    fn checked_naive_factorial(&self) -> Option<T> {
        if self < &T::from_usize(SMALL_PRIME_SWING.len()).unwrap() {
            return psw_factorial_with_array(self);
        }
        let small_prime_swing_limit = T::from_usize(SMALL_PRIME_SWING.len() - 1).unwrap();
        let mut acc = psw_factorial_with_array(&small_prime_swing_limit)?;
        let mut i = small_prime_swing_limit + T::one();
        while &i <= self {
            acc = acc.checked_mul(&i)?;
            i = i + T::one();
        }
        Some(acc)
    }

    #[inline(always)]
    fn psw_factorial(&self, sieve: &Sieve) -> Option<T> {
        if self < &T::from_usize(SMALL_PRIME_SWING.len())? {
            return psw_factorial_with_array(&self);
        }
        let first_term = Self::psw_factorial(&(self.clone() / T::from_usize(2)?), sieve)?;
        let swing = prime_swing(self.clone(), sieve)?;
        first_term.checked_mul(&first_term)?.checked_mul(&swing)
    }
}

#[inline(always)]
fn prime_swing<T: PartialOrd + Unsigned + CheckedMul + Clone + FromPrimitive + ToPrimitive>(
    n: T,
    sieve: &Sieve,
) -> Option<T> {
    let mut product = T::one();
    let two = T::from_usize(2)?;
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
                p = p.checked_mul(&T::from_usize(prime).unwrap())?;
            }
        }
        if p > T::one() {
            product = product.checked_mul(&p)?;
        }
    }
    Some(product)
}


#[inline(always)]
fn psw_factorial_with_array<T: PartialOrd + Unsigned + CheckedMul + Clone + FromPrimitive + ToPrimitive>(n: &T) -> Option<T> {
    if n < &T::from_usize(20).unwrap() {
        // return Self::factorial(&self);
        return T::from_usize(SMALL_FACTORIAL[n.to_usize().unwrap()]);
    }
    let first_term = psw_factorial_with_array(&(n.clone() / T::from_usize(2).unwrap()))?;
    let swing = T::from_usize(SMALL_PRIME_SWING[n.to_usize().unwrap()])?;
    first_term.checked_mul(&first_term)?.checked_mul(&swing)
}

fn small_factorial<T: PartialOrd + Unsigned + CheckedMul + Clone + FromPrimitive + ToPrimitive>(n: &T) -> Option<T> {
    let small_prime_swing_limit = T::from_usize(SMALL_PRIME_SWING.len() - 1).unwrap();
    let mut acc = psw_factorial_with_array(&small_prime_swing_limit)?;
    let mut i = small_prime_swing_limit + T::one();
    while &i <= n {
        acc = acc.checked_mul(&i)?;
        i = i + T::one();
    }
    Some(acc)
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
        assert_eq!(100.to_biguint().unwrap().checked_naive_factorial(), 100.to_biguint().unwrap().psw_factorial(&sieve));
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
