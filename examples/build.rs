use primal_sieve::Sieve;
use std::{fs::File, io::Write, path::Path};

fn checked_naive_factorial(n: u128) -> Option<u128> {
    let mut acc = 1u128;
    let mut i = 2u128;
    while i <= n {
        acc = acc.checked_mul(i)?;
        i += 1;
    }
    Some(acc)
}

fn prime_range(
    sieve: &Sieve,
    lower_bound: u128,
    upper_boud: u128,
) -> impl Iterator<Item = u128> + '_ {
    sieve
        .primes_from(lower_bound as usize)
        .map(|m| m as u128)
        .take_while(move |m| *m <= upper_boud)
}

const SMALL_SWING: [u128; 33] = [
    1, 1, 1, 3, 3, 15, 5, 35, 35, 315, 63, 693, 231, 3003, 429, 6435, 6435, 109395, 12155, 230945,
    46189, 969969, 88179, 2028117, 676039, 16900975, 1300075, 35102025, 5014575, 145422675,
    9694845, 300540195, 300540195,
];

fn prime_swing(n: u128, sieve: &Sieve) -> Option<u128> {
    if n < 33 {
        return Some(SMALL_SWING[n as usize]);
    }
    let sqrt = ((n as f64).sqrt().floor()) as u128;
    let mut product: u128 = 1;
    for prime in prime_range(sieve, n / 2 + 1, n) {
        product = product.checked_mul(prime)?;
    }

    for prime in prime_range(sieve, sqrt + 1, n / 3) {
        if (n / prime) & 1 == 1 {
            product = product.checked_mul(prime)?;
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
            product = product.checked_mul(p)?;
        }
    }
    Some(product)
}

fn main() -> std::io::Result<()> {
    let path = Path::new("./src/array.rs");
    println!("Generating {}.", path.display());
    let mut file_content = String::new();
    file_content.push_str("pub const SMALL_FACTORIAL: ");
    let mut n = 0u128;
    let mut factorials = vec![];
    while let Some(fac) = checked_naive_factorial(n) {
        factorials.push(fac);
        n += 1;
    }
    file_content.push_str(&format!(
        "[u128; {}] = {:#?};\n",
        factorials.len(),
        factorials
    ));

    let sieve = Sieve::new(1_000);
    file_content.push_str("pub const SMALL_PRIME_SWING: ");
    let mut n = 0u128;
    let mut prime_swings = vec![];
    while let Some(swing) = prime_swing(n, &sieve) {
        prime_swings.push(swing);
        n += 1;
    }
    file_content.push_str(&format!(
        "[u128; {}] = {:#?};\n",
        prime_swings.len(),
        prime_swings
    ));

    let mut file = File::create(path)?;
    file.write_all(file_content.as_bytes())?;
    Ok(())
}
