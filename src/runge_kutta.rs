use super::complex::Complex;
use std::ops::*;

#[derive(Debug, Clone)]
pub struct ComplexVec {
    pub vec: Vec<Complex>,
}

impl Add for ComplexVec {
    type Output = ComplexVec;
    #[inline(always)]
    fn add(self, rhs: ComplexVec) -> ComplexVec {
        ComplexVec {
            vec: self
                .vec
                .iter()
                .zip(rhs.vec.iter())
                .map(|(&x, &y)| x + y)
                .collect(),
        }
    }
}

impl Mul<f64> for ComplexVec {
    type Output = ComplexVec;
    #[inline(always)]
    fn mul(self, rhs: f64) -> ComplexVec {
        ComplexVec {
            vec: self.vec.iter().map(|&x| x * rhs).collect(),
        }
    }
}

impl Mul<ComplexVec> for f64 {
    type Output = ComplexVec;
    #[inline(always)]
    fn mul(self, rhs: ComplexVec) -> ComplexVec {
        ComplexVec {
            vec: rhs.vec.iter().map(|&x| x * self).collect(),
        }
    }
}

pub struct RungeKutta<F>
where
    F: Fn(ComplexVec, f64) -> ComplexVec,
{
    pub x: ComplexVec,
    pub func: F,
    pub t: f64,
    pub dt: f64,
}

impl<F> RungeKutta<F>
where
    F: Fn(ComplexVec, f64) -> ComplexVec,
{
    pub fn step(&mut self) {
        let k1 = self.dt * (self.func)(self.x.clone(), self.t);
        let k2 = self.dt * (self.func)(self.x.clone() + 0.5 * k1.clone(), self.t + 0.5 * self.dt);
        let k3 = self.dt * (self.func)(self.x.clone() + 0.5 * k2.clone(), self.t + 0.5 * self.dt);
        let k4 = self.dt * (self.func)(self.x.clone() + k3.clone(), self.t + self.dt);
        self.t += self.dt;
        self.x = self.x.clone() + 1. / 6. * (k1 + 2. * k2 + 2. * k3 + k4);
    }
}
