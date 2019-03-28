use core::fmt::{ Display, Formatter, Result as FmtResult };
use core::ops::{ Add, Sub, Mul, Div };

#[derive(Clone, Copy, Debug)]
pub struct Fixed16_16 {
    inner: i32,
}

impl Fixed16_16 {
    pub fn from_raw(value: i32) -> Self {
        Self { inner: value }
    }

    pub fn inner(self) -> i32 {
        self.inner
    }
}

impl From<i32> for Fixed16_16 {
    fn from(i: i32) -> Self {
        Self {
            inner: i << 16
        }
    }
}

impl From<f32> for Fixed16_16 {
    fn from(f: f32) -> Self {
        Self {
            inner: (f * ((1 << 16) as f32)) as i32
        }
    }
}

impl From<Fixed16_16> for f32 {
    fn from(f: Fixed16_16) -> Self {
        f.inner() as f32 / (1 << 16) as f32
    }
}

impl Add for Fixed16_16 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            inner: self.inner + other.inner
        }
    }
}

impl Sub for Fixed16_16 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            inner: self.inner - other.inner
        }
    }
}

impl Mul for Fixed16_16 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            inner: ((self.inner as i64 * other.inner as i64) >> 16) as i32
        }
    }
}

impl Div for Fixed16_16 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let result = ((self.inner as i64) << 16) / other.inner as i64;

        Self {
            inner: result as i32
        }
    }
}

impl Display for Fixed16_16 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let val: f32 = (*self).into();
        write!(f, "{}", val)
    }
}
