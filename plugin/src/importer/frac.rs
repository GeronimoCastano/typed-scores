//! Exact signed fractions for durations measured in whole notes.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Frac {
    pub n: i64,
    pub d: i64,
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

impl Frac {
    pub const ZERO: Frac = Frac { n: 0, d: 1 };
    pub const ONE: Frac = Frac { n: 1, d: 1 };

    pub fn new(n: i64, d: i64) -> Frac {
        Frac::reduce(n as i128, d as i128)
    }

    pub fn int(n: i64) -> Frac {
        Frac { n, d: 1 }
    }

    fn reduce(mut n: i128, mut d: i128) -> Frac {
        if d == 0 {
            // Malformed sources can divide by zero; treat the value as empty.
            return Frac::ZERO;
        }
        if d < 0 {
            n = -n;
            d = -d;
        }
        let divisor = gcd(n, d).max(1);
        Frac {
            n: (n / divisor) as i64,
            d: (d / divisor) as i64,
        }
    }

    /// True when the denominator is a power of two.
    pub fn is_binary(self) -> bool {
        self.d & (self.d - 1) == 0
    }

    pub fn is_positive(self) -> bool {
        self.n > 0
    }

    /// floor(self / other) for a positive divisor.
    pub fn floor_div(self, other: Frac) -> i64 {
        let numerator = self.n as i128 * other.d as i128;
        let denominator = self.d as i128 * other.n as i128;
        numerator.div_euclid(denominator) as i64
    }
}

impl Ord for Frac {
    fn cmp(&self, other: &Frac) -> Ordering {
        (self.n as i128 * other.d as i128).cmp(&(other.n as i128 * self.d as i128))
    }
}

impl PartialOrd for Frac {
    fn partial_cmp(&self, other: &Frac) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Frac {
    type Output = Frac;
    fn add(self, other: Frac) -> Frac {
        Frac::reduce(
            self.n as i128 * other.d as i128 + other.n as i128 * self.d as i128,
            self.d as i128 * other.d as i128,
        )
    }
}

impl Sub for Frac {
    type Output = Frac;
    fn sub(self, other: Frac) -> Frac {
        Frac::reduce(
            self.n as i128 * other.d as i128 - other.n as i128 * self.d as i128,
            self.d as i128 * other.d as i128,
        )
    }
}

impl Mul for Frac {
    type Output = Frac;
    fn mul(self, other: Frac) -> Frac {
        Frac::reduce(self.n as i128 * other.n as i128, self.d as i128 * other.d as i128)
    }
}

impl Div for Frac {
    type Output = Frac;
    fn div(self, other: Frac) -> Frac {
        Frac::reduce(self.n as i128 * other.d as i128, self.d as i128 * other.n as i128)
    }
}

impl AddAssign for Frac {
    fn add_assign(&mut self, other: Frac) {
        *self = *self + other;
    }
}

impl SubAssign for Frac {
    fn sub_assign(&mut self, other: Frac) {
        *self = *self - other;
    }
}

impl fmt::Display for Frac {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.d == 1 {
            write!(formatter, "{}", self.n)
        } else {
            write!(formatter, "{}/{}", self.n, self.d)
        }
    }
}
