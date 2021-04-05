use std::{
    cmp::Ordering,
    fmt,
    num::ParseIntError,
    ops::{Add, Div, Mul, Not, Rem, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Eq, Ord, Hash)]
pub enum TRSDATA {
    Constant(i64),
    Boolean(bool),
}

impl Add for TRSDATA {
    type Output = Option<TRSDATA>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            TRSDATA::Constant(a) => match rhs {
                TRSDATA::Constant(b) => Some(TRSDATA::Constant(a + b)),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Add for &TRSDATA {
    type Output = Option<TRSDATA>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            TRSDATA::Constant(a) => match rhs {
                TRSDATA::Constant(b) => Some(TRSDATA::Constant(*a + *b)),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Sub for &TRSDATA {
    type Output = Option<TRSDATA>;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            TRSDATA::Constant(a) => match rhs {
                TRSDATA::Constant(b) => Some(TRSDATA::Constant(*a - *b)),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Mul for &TRSDATA {
    type Output = Option<TRSDATA>;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            TRSDATA::Constant(a) => match rhs {
                TRSDATA::Constant(b) => Some(TRSDATA::Constant(*a * *b)),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Div for &TRSDATA {
    type Output = Option<TRSDATA>;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            TRSDATA::Constant(a) => match rhs {
                TRSDATA::Constant(b) => Some(TRSDATA::Constant(*a / *b)),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Not for &TRSDATA {
    type Output = Option<TRSDATA>;

    fn not(self) -> Self::Output {
        match self {
            TRSDATA::Boolean(a) => Some(TRSDATA::Boolean(!(*a))),
            _ => None,
        }
    }
}

impl Rem for &TRSDATA {
    type Output = Option<TRSDATA>;

    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            TRSDATA::Constant(a) => match rhs {
                TRSDATA::Constant(b) => {
                    if *b == 0 {
                        Some(TRSDATA::Constant(0))
                    } else {
                        Some(TRSDATA::Constant(*a % *b))
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }
}

pub fn and(var1: &TRSDATA, var2: &TRSDATA) -> Option<TRSDATA> {
    match *var1 {
        TRSDATA::Boolean(a) => match *var2 {
            TRSDATA::Boolean(b) => Some(TRSDATA::Boolean(a && b)),
            _ => None,
        },
        _ => None,
    }
}

pub fn or(var1: &TRSDATA, var2: &TRSDATA) -> Option<TRSDATA> {
    match *var1 {
        TRSDATA::Boolean(a) => match *var2 {
            TRSDATA::Boolean(b) => Some(TRSDATA::Boolean(a || b)),
            _ => None,
        },
        _ => None,
    }
}

impl PartialEq for TRSDATA {
    fn eq(&self, other: &Self) -> bool {
        match self {
            TRSDATA::Constant(a) => match other {
                TRSDATA::Constant(b) => a == b,
                TRSDATA::Boolean(_) => false,
            },
            TRSDATA::Boolean(a) => match other {
                TRSDATA::Constant(_) => false,
                TRSDATA::Boolean(b) => a == b,
            },
        }
    }

    fn ne(&self, other: &Self) -> bool {
        match self {
            TRSDATA::Constant(a) => match other {
                TRSDATA::Constant(b) => a != b,
                TRSDATA::Boolean(_) => false,
            },
            TRSDATA::Boolean(a) => match other {
                TRSDATA::Constant(_) => false,
                TRSDATA::Boolean(b) => a != b,
            },
        }
    }
}

impl PartialOrd for TRSDATA {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            TRSDATA::Constant(a) => match other {
                TRSDATA::Constant(b) => a.partial_cmp(b),
                TRSDATA::Boolean(_) => None,
            },
            TRSDATA::Boolean(a) => match other {
                TRSDATA::Constant(_) => None,
                TRSDATA::Boolean(b) => a.partial_cmp(b),
            },
        }
    }

    fn lt(&self, other: &Self) -> bool {
        match self {
            TRSDATA::Constant(a) => match other {
                TRSDATA::Constant(b) => a < b,
                _ => false,
            },
            _ => false,
        }
    }

    fn le(&self, other: &Self) -> bool {
        match self {
            TRSDATA::Constant(a) => match other {
                TRSDATA::Constant(b) => a <= b,
                _ => false,
            },
            _ => false,
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match self {
            TRSDATA::Constant(a) => match other {
                TRSDATA::Constant(b) => a > b,
                _ => false,
            },
            _ => false,
        }
    }

    fn ge(&self, other: &Self) -> bool {
        match self {
            TRSDATA::Constant(a) => match other {
                TRSDATA::Constant(b) => a >= b,
                _ => false,
            },
            _ => false,
        }
    }
}

impl fmt::Display for TRSDATA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TRSDATA::Boolean(b) => write!(f, "{:?}", b),
            TRSDATA::Constant(constant) => write!(f, "{:?}", constant),
        }
    }
}

impl FromStr for TRSDATA {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "true" => Ok(TRSDATA::Boolean(true)),
            "false" => Ok(TRSDATA::Boolean(false)),
            _ => Ok(TRSDATA::Constant(s.parse::<i64>()?)),
        }
    }
}
