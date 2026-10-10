use num_complex::Complex;
use std::f64::consts::PI;

// B_(2k) / (2k)!, for the Euler–Maclaurin tail.
const TAIL_COEFFICIENTS: [f64; 12] = [
    8.333333333333333e-2,
    -1.388888888888889e-3,
    3.306878306878307e-5,
    -8.267195767195768e-7,
    2.08767569878681e-8,
    -5.284190138687493e-10,
    1.338253653068468e-11,
    -3.389680296322583e-13,
    8.586062056277845e-15,
    -2.174868698558062e-16,
    5.50900282836023e-18,
    -1.3954464685812522e-19,
];

/// Riemann zeta with analytic continuation, evaluated in double precision.
///
/// Uses at least 200 series terms and 12 Euler–Maclaurin corrections.
/// Inputs with |s| > 10,000 or nonfinite components return NaN to bound render
/// work. The pole at s = 1 is represented by (infinity, 0); very large outputs
/// can still overflow f64. This is a plotting approximation, not an error bound.
pub fn zeta(s: Complex<f64>) -> Complex<f64> {
    if !s.is_finite() || s.norm() > 10_000.0 {
        return Complex::new(f64::NAN, f64::NAN);
    }
    if s == Complex::new(1.0, 0.0) {
        return Complex::new(f64::INFINITY, 0.0);
    }
    if s == Complex::new(0.0, 0.0) {
        return Complex::new(-0.5, 0.0);
    }
    if s.re < -0.5 {
        // Exact trivial zeros avoid roundoff in sin(pi*s/2).
        if s.im == 0.0 && s.re % 2.0 == 0.0 {
            return Complex::new(0.0, 0.0);
        }
        // Functional equation, DLMF 25.4.2. Combine factors in logarithms
        // so gamma underflow and sine overflow do not cancel numerically.
        let reflected = Complex::new(1.0, 0.0) - s;
        let log_factor =
            s * 2.0_f64.ln() + (s - 1.0) * PI.ln() + log_sin(s * (PI / 2.0)) + log_gamma(reflected);
        return log_factor.exp() * euler_maclaurin(reflected);
    }
    euler_maclaurin(s)
}

fn euler_maclaurin(s: Complex<f64>) -> Complex<f64> {
    // Larger |s| needs a longer sum to keep the tail corrections decreasing.
    let terms = 200.max((s.norm() / 2.0).ceil() as usize + 16);
    let n = (terms + 1) as f64;
    let mut sum = Complex::new(0.0, 0.0);
    let mut compensation = Complex::new(0.0, 0.0);
    for k in 1..=terms {
        // Compensated summation helps where oscillating terms cancel.
        let term = (-s).expf(k as f64) - compensation;
        let next = sum + term;
        compensation = (next - sum) - term;
        sum = next;
    }

    // Sum through n-1, then approximate the tail starting at n.
    // DLMF 25.11.7, specialized to the Riemann zeta function.
    let n_to_minus_s = (-s).expf(n);
    sum += n_to_minus_s * (n / (s - 1.0) + 0.5);
    let mut factor = s * n_to_minus_s / n;
    for (index, coefficient) in TAIL_COEFFICIENTS.iter().enumerate() {
        sum += factor * coefficient;
        let k = (index + 1) as f64;
        factor *= ((s + (2.0 * k - 1.0)) / n) * ((s + 2.0 * k) / n);
    }
    sum
}

fn log_sin(z: Complex<f64>) -> Complex<f64> {
    if z.im.abs() < 20.0 {
        return z.sin().ln();
    }
    let sign = z.im.signum();
    let small = Complex::new(-2.0 * z.im.abs(), 2.0 * sign * z.re).exp();
    Complex::new(z.im.abs() - 2.0_f64.ln(), sign * (PI / 2.0 - z.re))
        + (Complex::new(1.0, 0.0) - small).ln()
}

// Lanczos approximation with g = 7. Called only with Re(z) > 1.5.
fn log_gamma(z: Complex<f64>) -> Complex<f64> {
    const COEFFICIENTS: [f64; 8] = [
        676.5203681218851,
        -1259.1392167224028,
        771.3234287776531,
        -176.6150291621406,
        12.507343278686905,
        -0.13857109526572012,
        9.984369578019572e-6,
        1.5056327351493116e-7,
    ];
    let shifted = z - 1.0;
    let mut series = Complex::new(0.9999999999998099, 0.0);
    for (index, coefficient) in COEFFICIENTS.iter().enumerate() {
        series += coefficient / (shifted + (index + 1) as f64);
    }
    let t = shifted + 7.5;
    (shifted + 0.5) * t.ln() - t + series.ln() + 0.5 * (2.0 * PI).ln()
}
