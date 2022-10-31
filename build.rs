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

fn prime_swing(n: u128, sieve: &Sieve) -> Option<u128> {
    let mut product = 1u128;
    for prime in sieve.primes_from(2).take_while(|x| *x < n as usize) {
        let mut p = 1u128;
        let mut q = n;
        while q != 0 {
            q = q / prime as u128;
            // q%2 == 1 if q is odd
            if q % 2u128 == 1 {
                p = p.checked_mul(prime as u128)?;
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
    println!("It is executed");
    if !path.exists() {
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
    }
    Ok(())
}
