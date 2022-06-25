use std::{
    fmt::Display,
    iter::once,
    ops::{Add, Deref, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone)]
pub struct Value {
    pub current: isize,
    pub all: Vec<Kind>,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.current == other.current
    }
}

#[derive(Debug, Clone)]
pub enum Kind {
    Direct(isize),
    Roll(Vec<Kind>),
    Token(String),
}

impl Value {
    pub fn direct(inner: isize) -> Self {
        Self::new(inner, vec![Kind::Direct(inner)])
    }

    pub fn kind(inner: isize, kind: Kind) -> Self {
        Self::new(inner, vec![kind])
    }

    pub fn new(inner: isize, all: Vec<Kind>) -> Self {
        Self {
            current: inner,
            all,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <= (", self.current)?;
        self.all
            .iter()
            .intersperse(&Kind::Token(" ".to_string()))
            .chain(once(&Kind::Token(")".to_string())))
            .try_for_each(|kind| write!(f, "{kind}"))
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Direct(i) => write!(f, "{i}"),
            Kind::Roll(roll) => once(&Kind::Token("(".to_string()))
                .chain(roll.into_iter().intersperse(&Kind::Token(" ".to_string())))
                .chain(once(&Kind::Token(")".to_string())))
                .try_for_each(|kind| write!(f, "{kind}")),
            Kind::Token(s) => write!(f, "{s}"),
        }
    }
}

impl PartialEq<isize> for Value {
    fn eq(&self, other: &isize) -> bool {
        self.current == *other
    }
}

impl Deref for Value {
    type Target = isize;

    fn deref(&self) -> &Self::Target {
        &self.current
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.current + rhs.current,
            self.all
                .into_iter()
                .chain(once(Kind::Token("+".to_string())))
                .chain(rhs.all)
                .collect(),
        )
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.current - rhs.current,
            self.all
                .into_iter()
                .chain(once(Kind::Token("-".to_string())))
                .chain(rhs.all)
                .collect(),
        )
    }
}
impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.current * rhs.current,
            self.all
                .into_iter()
                .chain(once(Kind::Token("x".to_string())))
                .chain(rhs.all)
                .collect(),
        )
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.current / rhs.current,
            self.all
                .into_iter()
                .chain(once(Kind::Token("÷".to_string())))
                .chain(rhs.all)
                .collect(),
        )
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        Self::direct(-self.current)
    }
}
