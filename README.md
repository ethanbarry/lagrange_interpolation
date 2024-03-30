# Lagrange Interpolation

This is a technique that interpolates a function from tabulated values. An arbitrary number of tabulated values may be used, and they may
have any arbitrary spacing. This is a useful routine for Gaussian Quadrature and other advanced algorithms.

## Usage

Just import the single function in the crate with this statement.
```shell
use lagrange_interpolation::lagrange_interpolate;
```
Refer to the tests for examples of how to set up the function.

## Credit

The implementation was guided by [this](https://phys.libretexts.org/@go/page/8091?pdf) excellent textbook on Celestial Mechanics.
While I'm sure many clever FORTRAN and C implementations are available, I felt it would be nice to implement this in Rust.
