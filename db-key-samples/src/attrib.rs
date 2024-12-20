#![warn(missing_docs, missing_debug_implementations, bare_trait_objects)]

pub mod debug;

use db_key_macro::db_key;
use std::fmt::Debug;

/// This is a sample key to test the `db_key` attribute macro.
#[db_key(path = attrib)]
pub struct SampleKey {
    /// This is the ID number for the something.
    #[name = "ID"]
    #[default = 0x123456789ABCDEF0]
    id: u64,
    /// A u16 value.
    #[name = "Word"]
    word: u16,
    /// A u8 value.
    #[name = "Byte"]
    byte: u8,
    /// A u32 value.
    #[name = "Longword"]
    #[default = 0x12345678_u32]
    long: u32,
    /// A 3-byte array value.
    #[name = "End array"]
    #[default = [0xA5_u8; 3]]
    end: [u8; 3],
}

/// This is a sample key to test the `db_key` attribute macro.
#[db_key(no_new, path = attrib)]
pub struct NoNewKey {
    /// This is the ID number for the something.
    #[name = "ID"]
    id: u64,
    /// A u16 value.
    #[name = "Word"]
    word: u16,
    /// A u8 value.
    #[name = "Byte"]
    byte: u8,
    /// A u32 value.
    #[name = "Longword"]
    long: u32,
    /// A 3-byte array value.
    #[name = "End array"]
    #[default = [0x12_u8, 0x34, 0x56]]
    end: [u8; 3],
}

/// This is a sample private key.
#[db_key]
struct PrivateKey {
    /// This is the ID number for the something.
    #[name = "ID"]
    #[default = 0xFEDCBA9876543210]
    id: u64,
    #[name = "Word"]
    word: u16,
    /// A u8 value.
    #[name = "Byte"]
    byte: u8,
    /// A u32 value.
    #[name = "Longword"]
    #[default = 0x12345678_u32]
    long: u32,
    /// A 3-byte array value.
    #[name = "End array"]
    #[default = [0xA5_u8; 3]]
    end: [u8; 3],
}

/// This is a sample private key.
#[db_key]
pub(crate) struct PublicCrateKey {
    /// This is the ID number for the something.
    #[name = "ID"]
    id: u64,
    #[name = "Word"]
    #[default = 12345_u16]
    word: u16,
    /// A u8 value.
    #[name = "Byte"]
    #[default = 105]
    byte: u8,
    /// A u32 value.
    #[name = "Longword"]
    #[default = 0x12345678_u32]
    long: u32,
    /// A 3-byte array value.
    #[name = "End array"]
    #[default = [0xAB; 3]]
    end: [u8; 3],
}

/// This is a sample key with an alternate name.
#[db_key(alt_name = Args, path = attrib)]
pub struct Key {
    /// This is the ID number for the something.
    #[name = "ID"]
    id: u64,
    /// A u16 value.
    #[name = "Word"]
    #[default = 12453_u16]
    word: u16,
    /// A u8 value.
    #[name = "Byte"]
    #[default = 150]
    byte: u8,
    /// A u32 value.
    #[name = "Longword"]
    #[default = 0x18245637_u32]
    long: u32,
    /// A 3-byte array value.
    #[name = "End array"]
    #[default = [0xBA; 3]]
    end: [u8; 3],
}

/// This tests how large keys are handled.
#[db_key(path = attrib)]
pub struct BigKey {
    /// Big field 0.
    big0: u128,
    /// Big field 1.
    big1: u128,
    /// Big field 2.
    big2: u128,
    /// Big field 3.
    big3: u128,
    /// Big field 4.
    big4: u128,
    /// Big field 5.
    big5: u128,
    /// Big field 6.
    big6: u128,
    /// Big field 7.
    big7: u128,
    /// Big field 8.
    big8: u128,
    /// Big field 9.
    big9: u128,
    /// Big array field.
    array: [u8; 160],
}

#[db_key(path = attrib)]
pub struct NoDocKey {
    /// A 64-bit identifier field.
    id: u64,
    /// A u16 value.
    #[default = 12345_u16]
    /// A u16 value.
    word: u16,
    #[default = 105]
    /// A u8 value.
    byte: u8,
    #[default = 0x12345678_u32]
    /// A u32 value.
    long: u32,
    #[default = [0xAB; 3]]
    /// A 3-byte array value.
    end: [u8; 3],
}

/// This is a sample key with minimums and maximums.
#[db_key(path = attrib)]
pub struct MinMaxKey {
    /// This is the ID number for the something.
    #[name = "ID"]
    #[min=1]
    #[max=0x8000000000000000_u64]
    id: u64,
    /// A u16 value.
    #[name = "Word"]
    #[min = 150_u16]
    #[default = 12453_u16]
    #[max = 60000]
    word: u16,
    /// A u8 value.
    #[name = "Byte"]
    #[default = u8::MAX]
    #[min = u8::MAX]
    #[max = u8::MAX]
    byte: u8,
    /// A u32 value.
    #[name = "Longword"]
    #[min = 0x11111111]
    #[default = 0x18245637_u32]
    #[min = 0x99999999_u32]
    long: u32,
    /// A 3-byte array value.
    #[name = "End array"]
    #[default = [0xBA; 3]]
    #[max = [0xEF; 3]]
    #[min = [0x20; 3]]
    end: [u8; 3],
}

/// This is a sample key with signed integer values.
#[db_key(path = attrib)]
pub struct SignedKey {
    /// This is the ID number for the something.
    #[name = "ID"]
    #[min=-1234567890]
    #[max=1234567890]
    id: i64,
    /// A i16 value.
    #[name = "Word"]
    #[min = 150_i16]
    #[default = 12453_i16]
    #[max = 30000]
    word: i16,
    /// A i8 value.
    #[name = "Byte"]
    #[default = i8::MAX]
    #[min = i8::MAX]
    #[max = i8::MAX]
    byte: i8,
    /// A i32 value.
    #[name = "Longword"]
    #[min = -0x77777777]
    #[default = 0x18245637_i32]
    #[min = 0x77777777_i32]
    long: i32,
    /// A 3-byte array value.
    #[name = "End array"]
    #[default = [0xBA; 3]]
    #[max = [u8::MAX, 0xF0_u8, 0o360u8]]
    #[min = [0x20; 3]]
    end: [u8; 3],
}

/// This is a sample key to test the `no_min` and `no_max` options.
#[db_key(no_min, no_max, path = attrib)]
pub struct NoMinMaxKey {
    /// This is the ID number for the something.
    #[name = "ID"]
    id: u64,
    /// A u16 value.
    #[name = "Word"]
    word: u16,
    /// A u8 value.
    #[name = "Byte"]
    byte: u8,
    /// A u32 value.
    #[name = "Longword"]
    long: u32,
    /// A 3-byte array value.
    #[name = "End array"]
    #[default = [0x12_u8, 0x34, i8::MAX as u8]]
    end: [u8; 3],
}

/// This is a sample key to test the `no_min` option.
#[db_key(no_min, path = attrib)]
pub struct NoMinKey {
    /// This is the ID number for the something.
    #[name = "ID"]
    id: u64,
    /// A u16 value.
    #[name = "Word"]
    word: u16,
    /// A u8 value.
    #[name = "Byte"]
    byte: u8,
    /// A u32 value.
    #[name = "Longword"]
    long: u32,
    /// A 3-byte array value.
    #[name = "End array"]
    #[default = [0x12_u8, 0x34, i8::MAX as u8]]
    end: [u8; 3],
}

/// This is a sample key to test the `no_max` option.
#[db_key(no_max, path = attrib)]
pub struct NoMaxKey {
    /// This is the ID number for the something.
    #[name = "ID"]
    id: u64,
    /// A u16 value.
    #[name = "Word"]
    word: u16,
    /// A u8 value.
    #[name = "Byte"]
    byte: u8,
    /// A u32 value.
    #[name = "Longword"]
    long: u32,
    /// A 3-byte array value.
    #[name = "End array"]
    #[default = [0x12_u8, 0x34, i8::MAX as u8]]
    end: [u8; 3],
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
