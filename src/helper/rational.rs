use core::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rational {
    num: i64,
    den: i64,
}
impl Debug for Rational {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self}")
    }
}
impl Display for Rational {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}
impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.num * other.den).cmp(&(other.num * self.den))
    }
}
impl From<i64> for Rational {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl Rational {
    pub const fn new(x: i64) -> Self {
        Self { num: x, den: 1 }
    }
    pub const fn rat(num: i64, den: i64) -> Self {
        let mut x = Self { num, den };
        x.reduce();
        x
    }

    const fn reduce(&mut self) {
        let g = gcd(self.num, self.den);
        self.num /= g;
        self.den /= g;
        if self.den < 0 {
            self.num = -self.num;
            self.den = -self.den;
        }
    }

    const fn expand(&mut self, x: i64) {
        self.num *= x;
        self.den *= x;
    }
    pub const fn get_integer(self) -> Option<i64> {
        if self.den == 1 { Some(self.num) } else { None }
    }

    const fn reciprocal(&self) -> Self {
        Self::rat(self.den, self.num)
    }

    pub const fn floor(&self) -> i64 {
        let q = self.num / self.den;
        let r = self.num % self.den;

        if r == 0 || self.num >= 0 { q } else { q - 1 }
    }
    pub const fn ceil(&self) -> i64 {
        let q = self.num / self.den;
        let r = self.num % self.den;

        if r == 0 {
            q
        } else if self.num > 0 {
            q + 1
        } else {
            q
        }
    }
}
const fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}
impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut x = self;
        x.expand(rhs.den);
        let mut y = rhs;
        y.expand(self.den);
        debug_assert_eq!(x.den, y.den);
        Self::rat(x.num + y.num, x.den)
    }
}
impl AddAssign for Rational {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut x = self;
        x.expand(rhs.den);
        let mut y = rhs;
        y.expand(self.den);
        debug_assert_eq!(x.den, y.den);
        Self::rat(x.num - y.num, x.den)
    }
}
impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let g1 = gcd(self.num.abs(), rhs.den.abs());
        let g2 = gcd(self.den.abs(), rhs.num.abs());

        let n1 = self.num / g1;
        let d1 = self.den / g2;
        let n2 = rhs.num / g2;
        let d2 = rhs.den / g1;

        Self::rat(n1 * n2, d1 * d2)
    }
}

impl Div for Rational {
    type Output = Self;

    #[expect(clippy::suspicious_arithmetic_impl, reason = "thats the definition")]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.reciprocal()
    }
}
impl Neg for Rational {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::rat(-self.num, self.den)
    }
}
