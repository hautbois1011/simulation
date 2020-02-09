use super::complex::Complex;

pub fn fft(input: &Vec<Complex>, inverse: bool) -> Vec<Complex> {
    let len = input.len();
    let l = (len as f64).log2().ceil() as u32;
    let n = 2u32.pow(l) as usize;
    let mut x = input.clone();
    let y = &mut vec![Complex::new(0.0, 0.0); n];

    let mut p = n >> 1;
    let mut r = 1;

    for _ in 0..l as usize {
        let mut twiddle = (0..p)
            .map(|k| Complex::exp2pi(-(k as f64) / (2. * p as f64)))
            .collect::<Vec<Complex>>();

        if inverse {
            twiddle = twiddle.iter().map(|z| z.conj()).collect();
        }

        for r_hat in 0..r as usize {
            for p_hat in 0..p as usize {
                y[2 * r * p_hat + r_hat] = x[r * p_hat + r_hat] + x[p * r + r * p_hat + r_hat];
                y[2 * r * p_hat + r + r_hat] =
                    twiddle[p_hat] * (x[r * p_hat + r_hat] - x[p * r + r * p_hat + r_hat]);
            }
        }

        p >>= 1;
        r <<= 1;
        x = y.clone();
    }

    x.clone()
}

pub fn real_fft(input: &Vec<f64>) -> Vec<Complex> {
    let n = input.len();
    let mut y = vec![Complex::new(0.0, 0.0); n / 2];

    for i in 0..n / 2 {
        y[i] = Complex::new(input[2 * i], input[2 * i + 1]);
    }

    let c = fft(&y, true);

    let mut a = vec![Complex::new(0.0, 0.0); n / 2 + 1];

    a[0] = Complex::new(c[0].re + c[0].im, 0.0);
    a[n / 2] = Complex::new(c[0].re - c[0].im, 0.0);

    for k in 1..n / 2 {
        a[k] = c[n / 2 - k]
            + c[k].conj()
            + Complex::new(0.0, -1.0)
                * Complex::exp2pi(-(k as f64) / (n as f64))
                * (c[n / 2 - k] - c[k].conj());
    }

    a[0] = a[0] / Complex::new(n as f64, 0.0);
    a[n / 2] = a[n / 2] / Complex::new(n as f64, 0.0);
    for k in 1..n / 2 {
        a[k] = a[k] / Complex::new((n * 2) as f64, 0.0);
    }

    a
}

pub fn real_ifft(input: &Vec<Complex>) -> Vec<f64> {
    let n = input.len() - 1;

    let d = (0..n)
        .map(|k| {
            input[k]
                + input[n - k].conj()
                + Complex::new(0.0, 1.0)
                    * Complex::exp2pi((k as f64) / ((2 * n) as f64))
                    * (input[k] - input[n - k].conj())
        })
        .collect();

    let y = fft(&d, true);
    let mut x = vec![0.0; 2 * n];
    for i in 0..n {
        x[2 * i] = y[i].re;
        x[2 * i + 1] = y[i].im;
    }

    x
}
