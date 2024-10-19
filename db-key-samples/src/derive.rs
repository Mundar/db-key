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
#[derive(DBKey)]
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
