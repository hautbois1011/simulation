use std::ops::*;

/// Complex Number
#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(r: f64, i: f64) -> Complex {
        Complex { re: r, im: i }
    }

    pub fn abs(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn abs2(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn exp(&self) -> Complex {
        let n = self.re.exp();
        Complex {
            re: n * self.im.cos(),
            im: n * self.im.sin(),
        }
    }

    pub fn conj(&self) -> Complex {
        Complex {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn exp2pi(x: f64) -> Complex {
        let theta = 2. * std::f64::consts::PI * x;
        Complex {
            re: theta.cos(),
            im: theta.sin(),
        }
    }
}

// Overload Operators
impl Add for Complex {
    type Output = Complex;
    #[inline(always)]
    fn add(self, rhs: Complex) -> Complex {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Sub for Complex {
    type Output = Complex;
    #[inline(always)]
    fn sub(self, rhs: Complex) -> Complex {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Complex {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Mul<f64> for Complex {
    type Output = Complex;
    #[inline(always)]
    fn mul(self, rhs: f64) -> Complex {
        Complex {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl Div<Complex> for Complex {
    type Output = Complex;
    #[inline(always)]
    fn div(self, rhs: Complex) -> Complex {
        let r = rhs.re * rhs.re + rhs.im * rhs.im;
        Complex {
            re: (self.re * rhs.re + self.im * rhs.im) / r,
            im: (-self.re * rhs.im + self.im * rhs.re) / r,
        }
    }
}

impl Div<f64> for Complex {
    type Output = Complex;
    #[inline(always)]
    fn div(self, rhs: f64) -> Complex {
        Complex {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

impl Neg for Complex {
    type Output = Complex;
    #[inline(always)]
    fn neg(self) -> Complex {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}
