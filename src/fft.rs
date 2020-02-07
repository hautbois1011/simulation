use super::complex::Complex;

pub fn ifft(input: Vec<Complex>) -> Vec<Complex> {
    let len = input.len();
    let l = (len as f64).log2().ceil() as u32;
    let n = 2u32.pow(l) as usize;
    let mut x = vec![vec![Complex::new(0.0, 0.0); n]; (l + 1) as usize];
    for i in 0..len {
        x[0][i] = input[i];
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

pub fn fft(input: Vec<Complex>) -> Vec<Complex> {
    let len = input.len();
    let l = (len as f64).log2().ceil() as u32;
    let n = 2u32.pow(l) as usize;
    let mut x = vec![vec![Complex::new(0.0, 0.0); n]; (l + 1) as usize];
    for i in 0..len {
        x[0][i] = input[i].conj();
    }

    let mut p = n >> 1;
    let mut r = 1;
    for i in 0..l as usize {
        for r_hat in 0..r as usize {
            for p_hat in 0..p as usize {
                x[i + 1][2 * r * p_hat + r_hat] =
                    x[i][r * p_hat + r_hat] + x[i][p * r + r * p_hat + r_hat];
                x[i + 1][2 * r * p_hat + r + r_hat] =
                    Complex::exp2pi(-(p_hat as f64) / (2. * p as f64))
                        * (x[i][r * p_hat + r_hat] - x[i][p * r + r * p_hat + r_hat]);
            }
        }

        p >>= 1;
        r <<= 1;
    }

    x[l as usize].clone().iter().map(|&z| z.conj()).collect()
}
