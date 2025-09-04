use core::{
    fmt::{Debug, Display},
    str::FromStr,
};

#[expect(non_camel_case_types, reason = "convention")]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct i256 {
    hi: u128,
    lo: u128,
}
impl i256 {
    pub const ZERO: Self = Self { hi: 0, lo: 0 };
    pub const ONE: Self = Self { hi: 0, lo: 1 };
    pub const MINUS_ONE: Self = Self {
        hi: u128::MAX,
        lo: u128::MAX,
    };
    pub const MIN: Self = Self {
        hi: 1 << 127,
        lo: 0,
    };
    pub const MAX: Self = Self {
        hi: (u128::MAX >> 1),
        lo: u128::MAX,
    };
    pub const fn new() -> Self {
        Self::ZERO
    }
    /// Normally returns a u256 but i dont have it so it returns a i256
    /// Because of that it will return None if it is `Self::MIN`
    pub fn unsigned_abs(self) -> Option<Self> {
        if self == Self::MIN {
            return None;
        }
        if self.is_negative() {
            Some(self.checked_neg().unwrap())
        } else {
            Some(self)
        }
    }
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        if (self.is_negative() ^ rhs.is_negative()) && (self.unsigned_abs() == rhs.unsigned_abs()) {
            return Some(Self::ZERO);
        }
        let ret = self.wrapping_add(rhs);
        if self.is_negative() == rhs.is_negative() && self.is_negative() != ret.is_negative() {
            return None;
        }
        Some(ret)
    }
    #[must_use]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        let (lo, carry) = self.lo.overflowing_add(rhs.lo);
        let hi = self
            .hi
            .wrapping_add(rhs.hi.wrapping_add(if carry { 1 } else { 0 }));
        Self { hi, lo }
    }
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        let ret = self.wrapping_sub(rhs);
        match self.cmp(&rhs) {
            core::cmp::Ordering::Less => ret.is_negative().then_some(ret),
            core::cmp::Ordering::Equal => Some(Self::ZERO),
            core::cmp::Ordering::Greater => {
                if ret.is_negative() {
                    None
                } else {
                    Some(ret)
                }
            }
        }
    }
    #[must_use]
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        let (lo, carry) = self.lo.overflowing_sub(rhs.lo);
        let hi = self
            .hi
            .wrapping_sub(rhs.hi.wrapping_add(if carry { 1 } else { 0 }));
        Self { hi, lo }
    }
    /// Only fails if it is `i256::MIN`
    pub fn checked_neg(self) -> Option<Self> {
        if self == Self::MIN {
            None
        } else {
            Some(
                Self {
                    hi: !self.hi,
                    lo: !self.lo,
                }
                .wrapping_add(Self::ONE),
            )
        }
    }
    pub const fn is_negative(&self) -> bool {
        self.hi >> 127 == 1
    }
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        if self == rhs {
            return Some(Self::ONE);
        }
        let n = self
            .unsigned_abs()
            .expect("not yet implemented for i256::MIN");
        let d = rhs
            .unsigned_abs()
            .expect("not yet implemented for i256::MIN");
        n.checked_unsigned_div(d).map(|ret| {
            if self.is_negative() == rhs.is_negative() {
                ret
            } else {
                ret.checked_neg().expect("should never be i256::MIN")
            }
        })
    }
    pub fn checked_unsigned_div(self, rhs: Self) -> Option<Self> {
        self.checked_unsigned_div_and_rem(rhs).map(|e| e.0)
    }
    pub fn checked_unsigned_div_and_rem(self, rhs: Self) -> Option<(Self, Self)> {
        let d = if rhs.is_negative() { return None } else { rhs };
        let n = if self.is_negative() {
            return None;
        } else {
            self
        };
        let mut q = Self::ZERO;
        let mut r = Self::ZERO;
        if n == Self::ZERO {
            return None;
        }
        for i in (0..=255).rev() {
            r = r << 1;
            r.set_bit(0, n.get_bit(i));
            if r >= d {
                r = r.checked_sub(d).unwrap();
                q.set_bit(i, true);
            }
        }
        Some((q, r))
    }

    pub fn checked_shl(self, rhs: u8) -> Option<Self> {
        if rhs == 0 {
            return Some(self);
        }
        match rhs.cmp(&128) {
            core::cmp::Ordering::Less => {
                let mut hi = self.hi.checked_shl(rhs.into())?;
                hi |= self.lo.checked_shr((128 - rhs).into()).expect("cant fail");
                let lo = self.lo.checked_shl(rhs.into())?;
                Some(Self { hi, lo })
            }
            core::cmp::Ordering::Equal => Some(Self { hi: self.lo, lo: 0 }),
            core::cmp::Ordering::Greater => {
                let hi = self.lo.checked_shl((rhs - 128).into()).expect("cant fail");
                Some(Self { hi, lo: 0 })
            }
        }
    }
    pub fn checked_shr(self, rhs: u8) -> Option<Self> {
        if rhs == 0 {
            return Some(self);
        }
        match rhs.cmp(&128) {
            core::cmp::Ordering::Less => {
                let mut lo = self.lo.checked_shr(rhs.into())?;
                lo |= self.hi.checked_shl((128 - rhs).into()).expect("cant fail");
                let hi = self.hi.checked_shr(rhs.into())?;
                Some(Self { hi, lo })
            }
            core::cmp::Ordering::Equal => Some(Self { hi: 0, lo: self.hi }),
            core::cmp::Ordering::Greater => {
                let lo = self.hi.checked_shr((rhs - 128).into()).expect("cant fail");
                Some(Self { hi: 0, lo })
            }
        }
    }
    fn index_highest_one(&self) -> u8 {
        for i in (0..=255).rev() {
            if self.get_bit(i) {
                return i;
            }
        }
        0
    }
    pub fn unsigned_checked_mul(self, rhs: Self) -> Option<Self> {
        if self == Self::ZERO || rhs == Self::ZERO {
            return Some(Self::ZERO);
        }
        if self < Self::ZERO || rhs < Self::ZERO {
            return None;
        }
        let mut ret = Self::default();
        for i in (0..=255).rev() {
            if self.get_bit(i) {
                i.checked_add(rhs.index_highest_one())?.checked_add(1)?;
                if let Some(x) = ret.checked_add(rhs.checked_shl(i)?) {
                    ret = x;
                } else {
                    return None;
                }
            }
        }
        if ret.is_negative() { None } else { Some(ret) }
    }
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        let n = self
            .unsigned_abs()
            .expect("not yet implemented for i256::MIN");
        let d = rhs
            .unsigned_abs()
            .expect("not yet implemented for i256::MIN");
        n.unsigned_checked_mul(d).map(|ret| {
            if self.is_negative() == rhs.is_negative() {
                ret
            } else {
                ret.checked_neg().expect("should never be i256::MIN")
            }
        })
    }
    const fn set_bit(&mut self, i: u8, x: bool) {
        if i >= 128 {
            //hi
            if x {
                self.hi |= 1 << (i - 128);
            } else {
                self.hi &= !(1 << (i - 128));
            }
        } else {
            //lo
            if x {
                self.lo |= 1 << i;
            } else {
                self.lo &= !(1 << i);
            }
        }
    }
    const fn get_bit(&self, i: u8) -> bool {
        if i >= 128 {
            self.hi & (1 << (i - 128)) > 0
        } else {
            self.lo & (1 << (i)) > 0
        }
    }
    pub fn to_f64(self) -> Option<f64> {
        let n = self.unsigned_abs()?;

        let i = n.lo;
        #[expect(clippy::cast_precision_loss, reason = "its fine")]
        let mut ret = i as f64 * if self.is_negative() { -1.0 } else { 1.0 };
        // MSB cant be set
        for i in 128..255 {
            if n.get_bit(i) {
                let factor = 2.0_f64.powi(i32::from(i));
                ret *= factor;
            }
        }
        ret.is_finite().then_some(ret)
    }
    fn to_string_intern(self) -> String {
        let mut s = String::new();
        if self == Self::MIN {
            let mut s = self.checked_add(Self::ONE).unwrap().to_string();
            assert_eq!(s.pop(), Some('7'));
            s.push('8');
            return s;
        }
        let mut div = self.unsigned_abs().unwrap();
        let neg = self.is_negative();
        let mut rem;
        let ten: Self = Self::from(10);
        while div != Self::ZERO {
            (div, rem) = div.checked_unsigned_div_and_rem(ten).unwrap();
            let digit = u8::try_from(rem).unwrap();
            assert!((0..10).contains(&digit), "Error. Algortihm wrong?");
            s.push_str(&digit.to_string());
        }
        if neg {
            s.push('-');
        }
        s.chars().rev().collect()
    }
}
impl PartialOrd for i256 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for i256 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match (self.is_negative(), other.is_negative()) {
            (true, false) => core::cmp::Ordering::Less,
            (false, true) => core::cmp::Ordering::Greater,
            (true, true) | (false, false) => (self.hi, self.lo).cmp(&(other.hi, other.lo)),
        }
    }
}
impl Debug for i256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self}")
    }
}
impl Display for i256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_string_intern())
    }
}
impl Default for i256 {
    fn default() -> Self {
        Self::new()
    }
}
impl core::ops::Shl<u8> for i256 {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        self.checked_shl(rhs).unwrap()
    }
}
impl core::ops::Add for i256 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs).unwrap()
    }
}
impl core::ops::Sub for i256 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs).unwrap()
    }
}
impl core::ops::Div for i256 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.checked_div(rhs).unwrap()
    }
}
impl core::ops::Neg for i256 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.checked_neg().unwrap()
    }
}
impl core::ops::Mul for i256 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.checked_mul(rhs).unwrap()
    }
}
impl FromStr for i256 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rem, neg) = if s.starts_with('-') {
            (s.strip_prefix('-').unwrap(), true)
        } else {
            (s, false)
        };
        let mut n = Self::new();
        for c in rem.chars() {
            let Some(c) = c.to_digit(10) else {
                return Err(s.to_owned());
            };
            n = match n.unsigned_checked_mul(10.into()) {
                Some(x) => x,
                None => return Err(s.to_owned()),
            };
            n = match n.checked_add(c.into()) {
                Some(x) => x,
                None => return Err(s.to_owned()),
            };

            if n.is_negative() {
                return Err(s.to_owned());
            }
        }

        if neg {
            n.checked_neg().map_or_else(|| Err(s.to_owned()), Ok)
        } else {
            Ok(n)
        }
    }
}

impl TryFrom<i256> for u8 {
    type Error = i256;

    fn try_from(value: i256) -> Result<Self, Self::Error> {
        if value > i256::from(255) || value < i256::ZERO {
            return Err(value);
        }
        Ok(value.lo.try_into().unwrap())
    }
}
impl From<u128> for i256 {
    fn from(value: u128) -> Self {
        Self { hi: 0, lo: value }
    }
}
impl From<u8> for i256 {
    fn from(value: u8) -> Self {
        Self::from(u128::from(value))
    }
}
impl From<u16> for i256 {
    fn from(value: u16) -> Self {
        Self::from(u128::from(value))
    }
}
impl From<u32> for i256 {
    fn from(value: u32) -> Self {
        Self::from(u128::from(value))
    }
}
impl From<u64> for i256 {
    fn from(value: u64) -> Self {
        Self::from(u128::from(value))
    }
}
#[expect(clippy::fallible_impl_from, reason = "i128 to i256 is always safe")]
impl From<i128> for i256 {
    fn from(value: i128) -> Self {
        if value.is_negative() {
            Self::from(value.unsigned_abs()).checked_neg().unwrap()
        } else {
            Self::from(value.unsigned_abs())
        }
    }
}
impl From<i8> for i256 {
    fn from(value: i8) -> Self {
        Self::from(i128::from(value))
    }
}
impl From<i16> for i256 {
    fn from(value: i16) -> Self {
        Self::from(i128::from(value))
    }
}
impl From<i32> for i256 {
    fn from(value: i32) -> Self {
        Self::from(i128::from(value))
    }
}
impl From<i64> for i256 {
    fn from(value: i64) -> Self {
        Self::from(i128::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_unsigned_abs() {
        assert_eq!(i256::from(0).unsigned_abs(), Some(i256::from(0)));
        assert_eq!(i256::from(1).unsigned_abs(), Some(i256::from(1)));
        assert_eq!(i256::from(-1).unsigned_abs(), Some(i256::from(1)));
        assert_eq!(i256::from(-123).unsigned_abs(), Some(i256::from(123)));
        assert_eq!(i256::from(123).unsigned_abs(), Some(i256::from(123)));
        assert_eq!(i256::MAX.unsigned_abs(), Some(i256::MAX));
        assert_eq!(i256::MIN.unsigned_abs(), None);
    }
    #[test]
    fn test_ordering() {
        assert!(i256::from(0) < i256::from(1));
        assert!(i256::MIN < i256::from(1));
        assert!(i256::MIN < i256::from(-1));
        assert!(i256::MIN < i256::MAX);
        assert!(i256::MIN < i256::from(-1_234_567_890));
        assert!(i256::MAX > i256::from(1_234_567_890));
        assert!(i256::MAX > i256::from(-1_234_567_890));
        assert!(i256::MAX > i256::from(-1));
    }
    #[test]
    fn test_checked_neg() {
        assert_eq!(i256::from(1).checked_neg(), Some(i256::from(-1)));
        assert_eq!(i256::from(2).checked_neg(), Some(i256::from(-2)));
        assert_eq!(i256::from(-25).checked_neg(), Some(i256::from(25)));
        assert_eq!(i256::from(0).checked_neg(), Some(i256::from(0)));
        assert_eq!(
            i256::MAX.checked_neg(),
            Some(i256::MIN.checked_add(i256::from(1)).unwrap())
        );
        assert_eq!(i256::MIN.checked_neg(), None);
    }
    #[test]
    fn test_checked_add() {
        assert_eq!(
            i256::from(1).checked_add(i256::from(1)),
            Some(i256::from(2))
        );
        assert_eq!(
            i256::from(2).checked_add(i256::from(-2)),
            Some(i256::from(0))
        );
        assert_eq!(
            i256::from(1).checked_add(i256::from(-1)),
            Some(i256::from(0))
        );
        assert_eq!(
            i256::from(-1).checked_add(i256::from(1)),
            Some(i256::from(0))
        );
        assert_eq!(
            i256::from(-23).checked_add(i256::from(4)),
            Some(i256::from(-19))
        );
        assert_eq!(i256::MAX.checked_add(i256::from(1)), None);
        assert_eq!(i256::MAX.checked_add(i256::MAX), None);
        assert_eq!(i256::MIN.checked_add(i256::from(-1)), None);
        assert_eq!(i256::MIN.checked_add(i256::MIN), None);
        assert_eq!(i256::MIN.checked_add(i256::MAX), Some(i256::from(-1)));
        assert_eq!(i256::MAX.checked_add(i256::MIN), Some(i256::from(-1)));
    }
    #[test]
    fn test_wrapping_add() {
        assert_eq!(i256::from(1).wrapping_add(i256::from(1)), (i256::from(2)));
        assert_eq!(i256::from(2).wrapping_add(i256::from(-2)), (i256::from(0)));
        assert_eq!(i256::from(1).wrapping_add(i256::from(-1)), (i256::from(0)));
        assert_eq!(i256::from(-1).wrapping_add(i256::from(1)), (i256::from(0)));
        assert_eq!(
            i256::from(-23).wrapping_add(i256::from(4)),
            (i256::from(-19))
        );
        assert_eq!(i256::MAX.wrapping_add(i256::from(1)), i256::MIN);
        assert_eq!(i256::MAX.wrapping_add(i256::MAX), i256::from(-2));
        assert_eq!(i256::MIN.wrapping_add(i256::from(-1)), i256::MAX);
        assert_eq!(i256::MIN.wrapping_add(i256::MIN), i256::ZERO);
        assert_eq!(i256::MIN.wrapping_add(i256::MAX), (i256::from(-1)));
        assert_eq!(i256::MAX.wrapping_add(i256::MIN), (i256::from(-1)));
    }
    #[test]
    fn test_checked_sub() {
        assert_eq!(
            i256::from(1).checked_sub(i256::from(1)),
            Some(i256::from(0))
        );
        assert_eq!(
            i256::from(2).checked_sub(i256::from(-2)),
            Some(i256::from(4))
        );
        assert_eq!(
            i256::from(1).checked_sub(i256::from(-1)),
            Some(i256::from(2))
        );
        assert_eq!(
            i256::from(-1).checked_sub(i256::from(1)),
            Some(i256::from(-2))
        );
        assert_eq!(
            i256::from(-23).checked_sub(i256::from(4)),
            Some(i256::from(-27))
        );
        assert_eq!(
            i256::MAX.checked_sub(i256::from(1)),
            Some(i256 {
                hi: (u128::MAX >> 1),
                lo: (u128::MAX - 1)
            })
        );
        assert_eq!(i256::MAX.checked_sub(i256::MAX), Some(i256::ZERO));
        assert!(i256::MIN.checked_sub(i256::from(-1)).is_some());
        assert_eq!(i256::MIN.checked_sub(i256::MIN), Some(i256::ZERO));
        assert_eq!(i256::MIN.checked_sub(i256::MAX), None);
        assert_eq!(i256::MAX.checked_sub(i256::MIN), None);
    }
    #[test]
    fn test_wrapping_sub() {
        assert_eq!(i256::from(1).wrapping_sub(i256::from(1)), (i256::from(0)));
        assert_eq!(i256::from(2).wrapping_sub(i256::from(-2)), (i256::from(4)));
        assert_eq!(i256::from(1).wrapping_sub(i256::from(-1)), (i256::from(2)));
        assert_eq!(i256::from(-1).wrapping_sub(i256::from(1)), (i256::from(-2)));
        assert_eq!(
            i256::from(-23).wrapping_sub(i256::from(4)),
            (i256::from(-27))
        );
        assert_eq!(i256::MAX.wrapping_sub(i256::from(-1)), i256::MIN);
        assert_eq!(i256::MAX.wrapping_sub(i256::MAX), i256::from(0));
        assert_eq!(i256::MIN.wrapping_sub(i256::from(1)), i256::MAX);
        assert_eq!(i256::MIN.wrapping_sub(i256::MIN), i256::ZERO);
        assert_eq!(i256::MIN.wrapping_sub(i256::MAX), (i256::from(1)));
        assert_eq!(i256::MAX.wrapping_sub(i256::MIN), (i256::from(-1)));
    }
    #[test]
    fn test_checked_div() {
        assert_eq!(
            i256::from(10).checked_div(i256::from(5)),
            Some(i256::from(2))
        );
        assert_eq!(
            i256::from(-10).checked_div(i256::from(5)),
            Some(i256::from(-2))
        );
        assert_eq!(
            i256::from(10).checked_div(i256::from(-5)),
            Some(i256::from(-2))
        );
        assert_eq!(
            i256::from(-10).checked_div(i256::from(-5)),
            Some(i256::from(2))
        );
        assert_eq!(
            i256::from(14).checked_div(i256::from(5)),
            Some(i256::from(2))
        );
        assert_eq!(
            i256::from(-14).checked_div(i256::from(5)),
            Some(i256::from(-2))
        );
        assert_eq!(
            i256::from(14).checked_div(i256::from(-5)),
            Some(i256::from(-2))
        );
        assert_eq!(
            i256::from(-14).checked_div(i256::from(-5)),
            Some(i256::from(2))
        );
        assert_eq!(
            i256::MAX.checked_div(i256::from(2)),
            Some(i256::MAX.checked_shr(1).unwrap())
        );

        assert_eq!(
            i256::MAX
                .checked_div(i256::from(2))
                .unwrap()
                .checked_shr(254),
            Some(i256::ZERO)
        );
    }
    #[test]
    fn test_rem() {
        assert_eq!(
            i256::from(10).checked_unsigned_div_and_rem(i256::from(5)),
            Some((i256::from(2), i256::from(0)))
        );
        assert_eq!(
            i256::from(14).checked_unsigned_div_and_rem(i256::from(5)),
            Some((i256::from(2), i256::from(4)))
        );
    }
    #[test]
    fn test_unsigned_mul() {
        assert_eq!(
            i256::from(3).unsigned_checked_mul(i256::from(4)),
            Some(i256::from(12))
        );
        assert_eq!(
            i256::from(4).unsigned_checked_mul(i256::from(3)),
            Some(i256::from(12))
        );
        assert_eq!(i256::MAX.unsigned_checked_mul(i256::from(2)), None);
        assert_eq!(
            (i256::MAX
                .checked_div(2.into())
                .unwrap()
                .checked_add(1.into()))
            .unwrap()
            .unsigned_checked_mul(i256::from(2)),
            None
        );
        assert_eq!(
            (i256::MAX.checked_div(2.into()))
                .unwrap()
                .unsigned_checked_mul(i256::from(2)),
            Some(i256::MAX.checked_sub(1.into()).unwrap())
        );
    }
    #[test]
    fn parsing_works() {
        let vec = vec![
            ("1234".to_owned(), Ok(i256::from(1_234))),
            ("g".to_owned(), Err("g".to_owned())),
            ("-34".to_owned(), Ok(i256::from(-34))),
            ("-34".to_owned(), Ok(i256::from(-34))),
            (i256::MAX.to_string(), Ok(i256::MAX)),
            //(i256::MIN.to_string(), Ok(i256::MIN)), //Doesnt work right now
        ];
        for (x, y) in vec {
            assert_eq!(x.parse(), y, "{x} != {y:?}");
        }
    }
}
