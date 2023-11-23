# Compute factorials

This crate provides some convenient and safe methods to compute the factorial
with an efficient method. More precisely it uses the prime swing algorithm to
compute the factorial. See [this paper](https://oeis.org/A000142/a000142.pdf)
for more detail.

It can compute the factorial in `O(n (log n loglog n)^2)` operations of
multiplication. The time complexity of this algorithm depends on the time
complexity of the multiplication algorithm used.
