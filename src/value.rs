use std::{
    fmt::Display,
    ops::{Add, Deref, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    inner: f64,
}

impl Value {
    pub fn new(inner: f64) -> Self {
        Self { inner }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl PartialEq<f64> for Value {
    fn eq(&self, other: &f64) -> bool {
        self.inner == *other
    }
}

impl Deref for Value {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.inner + rhs.inner)
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.inner - rhs.inner)
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        Self::new(-self.inner)
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.inner * rhs.inner)
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.inner / rhs.inner)
    }
}
