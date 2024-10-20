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
    #[name = "Word"]
    word: u16,
    #[name = "Byte"]
    byte: u8,
    #[name = "Longword"]
    #[default = 0x12345678_u32]
    long: u32,
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
    #[name = "Word"]
    word: u16,
    #[name = "Byte"]
    byte: u8,
    #[name = "Longword"]
    long: u32,
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
    #[name = "Byte"]
    byte: u8,
    #[name = "Longword"]
    #[default = 0x12345678_u32]
    long: u32,
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
    #[name = "Byte"]
    #[default = 105]
    byte: u8,
    #[name = "Longword"]
    #[default = 0x12345678_u32]
    long: u32,
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
    #[name = "Word"]
    #[default = 12453_u16]
    word: u16,
    #[name = "Byte"]
    #[default = 150]
    byte: u8,
    #[name = "Longword"]
    #[default = 0x18245637_u32]
    long: u32,
    #[name = "End array"]
    #[default = [0xBA; 3]]
    end: [u8; 3],
}

/// This tests how large keys are handled.
#[db_key(path = attrib)]
pub struct BigKey {
    big0: u128,
    big1: u128,
    big2: u128,
    big3: u128,
    big4: u128,
    big5: u128,
    big6: u128,
    big7: u128,
    big8: u128,
    big9: u128,
    array: [u8; 160],
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
