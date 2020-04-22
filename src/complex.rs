use std::f32::consts::*;
use std::ops::*;
use std::iter::Sum;
use crate::square;

#[derive(Copy, Clone)]
pub struct C {
    pub re: f32,
    pub im: f32
}
impl Mul<f32> for C {
    type Output = Self;
    fn mul(self, rhs: f32) -> C {
        C { re: self.re * rhs, im: self.im * rhs }
    }
}
impl Mul for C {
    type Output = Self;
    fn mul(self, rhs: C) -> C {
        C { re: self.re * rhs.re - self.im * rhs.im, im: self.re * rhs.im + self.im + rhs.re }
    }
}
impl Add for C {
    type Output = Self;
    fn add(self, rhs: C) -> C {
        C { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}
impl Sum for C {
    fn sum<I>(iter: I) -> Self where I: Iterator<Item = Self> {
        let mut acc = C::new(0.0, 0.0);
        for val in iter {
            acc = acc + val;
        }
        acc
    }
}
impl C {
    pub fn new(real: f32, imaginary: f32) -> Self {
        C { re: real, im: imaginary }
    }
    pub fn polar(r: f32, phi: f32) -> Self {
        C::new(phi.cos(), phi.sin()) * r
    }
    pub fn square(self) -> Self {
        self * self
    }
    pub fn exp(self) -> Self {
        C::polar(self.re.exp(), self.im)
    }
    pub fn abs(self) -> f32 {
        (square(self.re) + square(self.im)).sqrt()
    }
}
