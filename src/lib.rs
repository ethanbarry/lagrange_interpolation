/// This function interpolates a function's value
/// when passed a `Vec` of precomputed points and the x-value at
/// which to evaluate it. It uses a sum of Lagrange polynomials.
/// When interpolating a polynomial on n points, if the polynomial's
/// degree is less than n, the calculation is exact.
///
/// ## Conditions
///
/// - The `Vec` passed in must be ordered from least to greatest x-values.
/// - It must contain two or more points.
/// - The coordinates must be valid numbers.
///
///
/// ## Example
///
/// Interpolate sin(x) at x = 51.0 degrees.
/// ```rust
/// use lagrange_interpolation::lagrange_interpolate;
/// let table = vec![(0.0, 0.0), (30.0, 0.5), (60.0, 0.86603), (90.0, 1.0)];
/// let res = lagrange_interpolate(&table, 51.0);
/// assert!((res - 0.7771).abs() <= 1e-3);
/// ```
///
/// ## Panics
///
/// - If the value passed in is not within the bounds of the table.
/// - If a value in the table does not exist or cannot be read.
pub fn lagrange_interpolate(table: &Vec<(f64, f64)>, xval: f64) -> f64 {
    assert!(
        xval > table.first().expect("Must not be empty.").0
            && xval < table.last().expect("Must not be empty.").0
    );
    let n = table.len();
    let mut sum = 0.0;

    for i in 0..n {
        if let Some((x_i, y_i)) = table.get(i) {
            // Evaluate the ith Lagrange poly'n. at xval and add it to sum.
            let mut product = 1.0;

            // Iterator grabs all but i.
            for j in (0..n)
                .enumerate()
                .filter(|&(pos, _)| (pos != i))
                .map(|(_, e)| e)
            {
                let x_j = table.get(j).unwrap().0;
                product *= (xval - x_j) / (x_i - x_j);
            } // product is now the value of L_i(xval).

            sum += y_i * product;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolate_sin() {
        let table = vec![(0.0, 0.0), (30.0, 0.5), (60.0, 0.86603), (90.0, 1.0)];
        let res = lagrange_interpolate(&table, 51.0);
        assert!((res - 0.7771).abs() <= 1e-3);
    }

    #[test]
    #[should_panic]
    fn extrapolate_sin() {
        let table = vec![(0.0, 0.0), (30.0, 0.5), (60.0, 0.86603), (90.0, 1.0)];
        let _res = lagrange_interpolate(&table, 100.0); // xval too large.
    }
}
