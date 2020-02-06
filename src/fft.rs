use super::complex::Complex;
use std::ops::Fn;

pub fn fft<F>(func: F, l: u32) -> Vec<Complex>
where
    F: Fn(f64) -> f64,
{
    let n = 2u32.pow(l) as usize;
    let mut x = vec![vec![Complex::new(0.0, 0.0); n]; (l + 1) as usize];
    for i in 0..n {
        x[0][i] = Complex::new(
            func(2. * std::f64::consts::PI * (i as f64) / (n as f64)),
            0.0,
        );
    }

    let mut p = n >> 1;
    let mut r = 1;
    for i in 0..l as usize {
        for r_hat in 0..r as usize {
            for p_hat in 0..p as usize {
                x[i + 1][2 * r * p_hat + r_hat] =
                    x[i][r * p_hat + r_hat] + x[i][p * r + r * p_hat + r_hat];
                x[i + 1][2 * r * p_hat + r + r_hat] =
                    Complex::exp2pi((p_hat as f64) / (2. * p as f64))
                        * (x[i][r * p_hat + r_hat] - x[i][p * r + r * p_hat + r_hat]);
            }
        }

        p >>= 1;
        r <<= 1;
    }

    x[l as usize].clone()
}
