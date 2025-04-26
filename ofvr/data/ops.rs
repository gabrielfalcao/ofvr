use std::ops::{
    Add, BitXor, BitXorAssign, Div, Index, IndexMut, Mul, Not, Rem, Shl, Shr, Sub,
};

use super::core::Data;
use crate::utils::{xor, xor_ip};


impl BitXor for Data {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        Data::from(xor(&self.to_vec(), &other.to_vec()))
    }
}

impl BitXorAssign for Data {
    fn bitxor_assign(&mut self, rhs: Self) {
        xor_ip(&mut self.inner, &rhs.to_vec())
    }
}

impl Add for Data {
    type Output = Data;

    fn add(self, other: Data) -> Data {
        Data::from(
            self.to_vec()
                .iter()
                .zip(other.to_vec())
                .map(|(s, o)| s + o)
                .collect::<Vec<u8>>(),
        )
    }
}
impl Sub for Data {
    type Output = Data;

    fn sub(self, other: Data) -> Data {
        Data::from(
            self.to_vec()
                .iter()
                .zip(other.to_vec())
                .map(|(s, o)| s - o)
                .collect::<Vec<u8>>(),
        )
    }
}
impl Div for Data {
    type Output = Data;

    fn div(self, other: Data) -> Data {
        Data::from(
            self.to_vec()
                .iter()
                .zip(other.to_vec())
                .map(|(s, o)| s / o)
                .collect::<Vec<u8>>(),
        )
    }
}
impl Mul for Data {
    type Output = Data;

    fn mul(self, other: Data) -> Data {
        Data::from(
            self.to_vec()
                .iter()
                .zip(other.to_vec())
                .map(|(s, o)| s * o)
                .collect::<Vec<u8>>(),
        )
    }
}
impl Rem for Data {
    type Output = Data;

    fn rem(self, other: Data) -> Data {
        Data::from(
            self.to_vec()
                .iter()
                .zip(other.to_vec())
                .map(|(s, o)| s % o)
                .collect::<Vec<u8>>(),
        )
    }
}
impl Shl for Data {
    type Output = Data;

    fn shl(self, other: Data) -> Data {
        Data::from(
            self.to_vec()
                .iter()
                .zip(other.to_vec())
                .map(|(s, o)| s << o)
                .collect::<Vec<u8>>(),
        )
    }
}
impl Shr for Data {
    type Output = Data;

    fn shr(self, other: Data) -> Data {
        Data::from(
            self.to_vec()
                .iter()
                .zip(other.to_vec())
                .map(|(s, o)| s >> o)
                .collect::<Vec<u8>>(),
        )
    }
}
impl Index<usize> for Data {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl IndexMut<usize> for Data {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.inner.index_mut(index)
    }
}

impl Not for Data {
    type Output = Data;

    fn not(self) -> Self::Output {
        Data::from(self.to_vec().iter().map(|s| !s).collect::<Vec<u8>>())
    }
}
