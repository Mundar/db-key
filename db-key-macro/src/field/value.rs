use rand::{Rng, rngs::ThreadRng};
use crate::field::FieldSize;
use std::{
    borrow::Borrow,
    fmt::{Display, Formatter, Result},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldValue {
    size: FieldSize,
    value: Vec<u8>,
}

impl FieldValue {
    pub fn random(size: FieldSize) -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        let len = size.size();
        let mut value = Vec::with_capacity(len);
        for _ in 0..len {
            value.push(rng.gen());
        }
        Self {
            size,
            value,
        }
    }

    pub fn assert_eq<'v>(&'v self) -> FieldValueAssertEq<'v> {
        FieldValueAssertEq(self)
    }
}

macro_rules! impl_to_from_unsigned {
    ($(($ux:ident, $size:ident),)+) => {
        $(
        impl From<$ux> for FieldValue {
            fn from(value: $ux) -> Self {
                Self {
                    size: FieldSize::$size,
                    value: value.to_be_bytes().to_vec(),
                }
            }
        }

        impl TryFrom<&FieldValue> for $ux {
            type Error = syn::Error;

            fn try_from(value: &FieldValue) -> std::result::Result<Self, Self::Error> {
                match value.size {
                    FieldSize::$size => {
                        let mut buf = [0_u8; std::mem::size_of::<$ux>()];
                        buf.copy_from_slice(&value.value);
                        Ok($ux::from_be_bytes(buf))
                    }
                    wrong_type => Err(Self::Error::new(proc_macro2::Span::mixed_site(),
                        format_args!(concat!("Failed to convert a FieldValue to a ",
                            stringify!($ux), " because it is the wrong type: {:?}"), wrong_type))),
                }
            }
        }
        )+
    }
}

macro_rules! impl_to_from_array {
    ($($size:literal)+) => {
        $(
        impl From<[u8; $size]> for FieldValue {
            fn from(value: [u8; $size]) -> Self {
                Self {
                    size: FieldSize::Array($size),
                    value: value.to_vec(),
                }
            }
        }

        impl TryFrom<&FieldValue> for [u8; $size] {
            type Error = syn::Error;

            fn try_from(value: &FieldValue) -> std::result::Result<Self, Self::Error> {
                let value = value.borrow();
                match value.size {
                    FieldSize::Array($size) => {
                        let mut array = [0_u8; $size];
                        array.copy_from_slice(&value.value);
                        Ok(array)
                    }
                    FieldSize::Array(wrong_size) => Err(Self::Error::new(
                        proc_macro2::Span::mixed_site(),
                        format_args!(concat!("Failed to convert a FieldValue to a ",
                            stringify!($size), "-byte array because it is the wrong size: {:?}"), wrong_size))),
                    wrong_type => Err(Self::Error::new(proc_macro2::Span::mixed_site(),
                        format_args!(concat!("Failed to convert a FieldValue to a ",
                            stringify!($size), "-byte array because it is the wrong type: {:?}"), wrong_type))),
                }
            }
        }
        )+
    };
}

impl_to_from_unsigned! {
    (u8, Unsigned8),
    (u16, Unsigned16),
    (u32, Unsigned32),
    (u64, Unsigned64),
    (u128, Unsigned128),
}

impl_to_from_array! {
    1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
}

macro_rules! from_be_bytes {
    ($value:expr, $ty:ident) => {
        {
            let mut buf = [0_u8; std::mem::size_of::<$ty>()];
            buf.copy_from_slice(&$value);
            $ty::from_be_bytes(buf)
        }
    }
}

impl Display for FieldValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.size {
            FieldSize::Unsigned8 => write!(f, "{:#04X}_u8", self.value[0]),
            FieldSize::Unsigned16 => write!(f, "{:#06X}_u16", from_be_bytes!(self.value, u16)),
            FieldSize::Unsigned32 => write!(f, "{:#010X}_u32", from_be_bytes!(self.value, u32)),
            FieldSize::Unsigned64 => write!(f, "{:#018X}_u64", from_be_bytes!(self.value, u64)),
            FieldSize::Unsigned128 => write!(f, "{:#034X}_u128", from_be_bytes!(self.value, u128)),
            FieldSize::Array(size) => {
                f.write_str("[")?;
                f.write_fmt(format_args!("{:#04X}_u8", self.value[0]))?;
                for i in 1..size {
                    f.write_fmt(format_args!(", {:#04X}", self.value[i]))?;
                }
                f.write_str("]")
            }
        }
    }
}

/// A reference to a `FieldValue` with a `Display` implementation for comparing in an assert_eq
/// macro.
pub struct FieldValueAssertEq<'v>(&'v FieldValue);

impl<'v> Display for FieldValueAssertEq<'v> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0.size {
            FieldSize::Array(_size) => {
                f.write_str("&")?;
                self.0.fmt(f)
            }
            _ => self.0.fmt(f),
        }
    }
}

/// A reference to a `FieldValue` with a `Display` implementation for outputing just a comma
/// separated list of bytes.
pub struct FieldValueJustBytes<'v>(&'v FieldValue);

impl<'v> Display for FieldValueJustBytes<'v> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let size = self.0.size.size();
        if 0 < size {
            f.write_fmt(format_args!("{:#04X}", self.0.value[0]))?;
            for i in 1..size {
                f.write_fmt(format_args!(", {:#04X}", self.0.value[i]))?;
            }
        }
        Ok(())
    }
}
