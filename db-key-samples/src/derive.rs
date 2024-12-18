pub mod debug;

use db_key_macro::DBKey;
use std::fmt::Debug;

/// This is a sample key using `derive(DBKey)`.
#[derive(Copy, Clone, DBKey, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[key(path = derive)]
pub struct Sample {
    /// This is the ID number for the something.
    #[name = "ID"]
    #[default = 0x123456789ABCDEF0]
    pub id: u64,
    #[name = "Word"]
    pub word: u16,
    #[name = "Byte"]
    pub byte: u8,
    #[name = "Longword"]
    #[default = "0x12345678_u32"]
    pub long: u32,
    #[name = "End array"]
    #[default = "[0xA5_u8; 3]"]
    pub end: [u8; 3],
}

/// This is a sample key using `derive(DBKey)`.
#[derive(Copy, Clone, DBKey, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[key(no_new, path = derive)]
pub struct NoNew {
    /// This is the ID number for the something.
    #[name = "ID"]
    pub id: u64,
    #[name = "Word"]
    pub word: u16,
    #[name = "Byte"]
    pub byte: u8,
    #[name = "Longword"]
    pub long: u32,
    #[name = "End array"]
    #[default = "[0x12_u8, 0x34, 0x56]"]
    pub end: [u8; 3],
}

/// This is a sample key with an alternate name.
#[derive(Copy, Clone, DBKey, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[key(alt_name = Args, path = derive)]
pub struct Key {
    /// This is the ID number for the something.
    #[name = "ID"]
    pub id: u64,
    #[name = "Word"]
    #[default = "12453_u16"]
    pub word: u16,
    #[name = "Byte"]
    #[default = 150]
    pub byte: u8,
    #[name = "Longword"]
    #[default = "0x18245637_u32"]
    pub long: u32,
    #[name = "End array"]
    #[default = "[0xBA; 3]"]
    pub end: [u8; 3],
}

/// This tests how large keys are handled.
#[derive(Copy, Clone, DBKey, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[key(path = derive)]
pub struct Big {
    pub big0: u128,
    pub big1: u128,
    pub big2: u128,
    pub big3: u128,
    pub big4: u128,
    pub big5: u128,
    pub big6: u128,
    pub big7: u128,
    pub big8: u128,
    pub big9: u128,
    pub array: [u8; 160],
}

#[derive(Copy, Clone, DBKey, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[key(path = derive)]
pub struct NoDoc {
    pub id: u64,
    #[default = "12345_u16"]
    pub word: u16,
    #[default = 105]
    pub byte: u8,
    #[default = "0x12345678_u32"]
    pub long: u32,
    #[default = "[0xAB; 3]"]
    pub end: [u8; 3],
}

/// This is a sample key with minimums and maximums.
#[derive(Copy, Clone, DBKey, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[key(path = derive)]
pub struct MinMax {
    /// This is the ID number for the something.
    #[name = "ID"]
    #[min=1]
    #[max="0x8000000000000000_u64"]
    pub id: u64,
    /// A u16 value.
    #[name = "Word"]
    #[min = "150_u16"]
    #[default = "12453_u16"]
    #[max = 60000]
    pub word: u16,
    /// A u8 value.
    #[name = "Byte"]
    #[default = "u8::MAX"]
    #[min = "u8::MAX"]
    #[max = "u8::MAX"]
    pub byte: u8,
    /// A u32 value.
    #[name = "Longword"]
    #[min = 0x11111111]
    #[default = "0x18245637_u32"]
    #[min = "0x99999999_u32"]
    pub long: u32,
    /// A 3-byte array value.
    #[name = "End array"]
    #[default = "[0xBA; 3]"]
    #[max = "[0xEF; 3]"]
    #[min = "[0x20; 3]"]
    pub end: [u8; 3],
}

/// This is a sample key with signed integer fields.
#[derive(Copy, Clone, DBKey, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[key(path = derive)]
pub struct Signed {
    /// This is the ID number for the something.
    #[name = "ID"]
    #[min="-0x8000000000000000_i64"]
    #[max=0]
    pub id: i64,
    /// A i16 value.
    #[name = "Word"]
    #[min = "-30000_i16"]
    #[default = "12453_i16"]
    #[max = 30000]
    pub word: i16,
    /// A i8 value.
    #[name = "Byte"]
    #[default = "i8::MAX"]
    #[min = "i8::MAX"]
    #[max = "i8::MAX"]
    pub byte: i8,
    /// A i32 value.
    #[name = "Longword"]
    #[min = "-0x77777777"]
    #[default = "0x18245637_i32"]
    #[min = "0x77777777_i32"]
    pub long: i32,
    /// A 3-byte array value.
    #[name = "End array"]
    #[default = "[0xBA; 3]"]
    #[max = "[u8::MAX, 0xF0_u8, 0o360u8]"]
    #[min = "[0x20; 3]"]
    pub end: [u8; 3],
}

/// This is a sample key the `no_min` and `no_max` options.
#[derive(Copy, Clone, DBKey, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[key(no_min, no_max, path = derive)]
pub struct NoMinMax {
    /// This is the ID number for the something.
    #[name = "ID"]
    pub id: u64,
    #[name = "Word"]
    pub word: u16,
    #[name = "Byte"]
    pub byte: u8,
    #[name = "Longword"]
    pub long: u32,
    #[name = "End array"]
    #[default = "[0x12_u8, 0x34, i8::MAX as u8]"]
    pub end: [u8; 3],
}

/// This is a sample key the `no_min` option.
#[derive(Copy, Clone, DBKey, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[key(no_min, path = derive)]
pub struct NoMin {
    /// This is the ID number for the something.
    #[name = "ID"]
    pub id: u64,
    #[name = "Word"]
    pub word: u16,
    #[name = "Byte"]
    pub byte: u8,
    #[name = "Longword"]
    pub long: u32,
    #[name = "End array"]
    #[default = "[0x12_u8, 0x34, i8::MAX as u8]"]
    pub end: [u8; 3],
}

/// This is a sample key the `no_max` option.
#[derive(Copy, Clone, DBKey, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[key(no_max, path = derive)]
pub struct NoMax {
    /// This is the ID number for the something.
    #[name = "ID"]
    pub id: u64,
    #[name = "Word"]
    pub word: u16,
    #[name = "Byte"]
    pub byte: u8,
    #[name = "Longword"]
    pub long: u32,
    #[name = "End array"]
    #[default = "[0x12_u8, 0x34, i8::MAX as u8]"]
    pub end: [u8; 3],
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn simple_tests(
            ids in proptest::collection::btree_set(0..=u64::MAX, 2..=2),
            words in proptest::collection::btree_set(0..=u16::MAX, 2..=2),
            bytes in proptest::collection::btree_set(0..=u8::MAX, 2..=2),
            longs in proptest::collection::btree_set(0..=u32::MAX, 2..=2),
            ends in proptest::collection::btree_set(
                proptest::collection::vec(0..=u8::MAX, 3..=3), 2..=2),
        ) {
            let id: Vec<u64> = ids.iter().map(|x| *x).collect();
            let word: Vec<u16> = words.iter().map(|x| *x).collect();
            let byte: Vec<u8> = bytes.iter().map(|x| *x).collect();
            let long: Vec<u32> = longs.iter().map(|x| *x).collect();
            let end: Vec<[u8; 3]> = ends.iter().map(|x| {
                let mut buf = [0_u8; 3];
                buf.copy_from_slice(x);
                buf
            }).collect();
            let mut array = [[0_u8; 18]; 2];
            for i in 0..2 {
                array[i][0..8].copy_from_slice(&id[i].to_be_bytes());
                array[i][8..10].copy_from_slice(&word[i].to_be_bytes());
                array[i][10..11].copy_from_slice(&byte[i].to_be_bytes());
                array[i][11..15].copy_from_slice(&long[i].to_be_bytes());
                array[i][15..18].copy_from_slice(&end[i]);
            }
            let mut new_key = Vec::with_capacity(2);
            let mut key_from_array = Vec::with_capacity(2);
            for i in 0..2 {
                new_key.push(SampleKey::new(id[i], word[i], byte[i], long[i], end[i]));
                key_from_array.push(SampleKey::from(array[i].clone()));
                assert_eq!(new_key[i], key_from_array[i]);
            }
        }
    }
}
