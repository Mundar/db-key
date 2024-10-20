use db_key_macro::{DBKey, db_key};
use proptest::prelude::*;

#[db_key]
struct AttributeKey {
    one: u64,
    two: u32,
    three: u16,
    four: u8,
    five: [u8; 3],
}

#[derive(DBKey, PartialEq, Eq, PartialOrd, Ord)]
struct Derive {
    one: u64,
    two: u32,
    three: u16,
    four: u8,
    five: [u8; 3],
}

prop_compose! {
    fn ordered_inputs(count: usize)(
        ones in proptest::collection::btree_set(0..=u64::MAX, count..=count),
        twos in proptest::collection::btree_set(0..=u32::MAX, count..=count),
        threes in proptest::collection::btree_set(0..=u16::MAX, count..=count),
        fours in proptest::collection::btree_set(0..=u8::MAX, count..=count),
        fives in proptest::collection::btree_set([0..=u8::MAX, 0..=u8::MAX, 0..=u8::MAX], count..=count),
    ) -> (Vec<u64>, Vec<u32>, Vec<u16>, Vec<u8>, Vec<[u8; 3]>) {
        let ones: Vec<u64> = ones.iter().map(|v| *v).collect();
        let twos: Vec<u32> = twos.iter().map(|v| *v).collect();
        let threes: Vec<u16> = threes.iter().map(|v| *v).collect();
        let fours: Vec<u8> = fours.iter().map(|v| *v).collect();
        let fives: Vec<[u8; 3]> = fives.iter().map(|v| *v).collect();
        (ones, twos, threes, fours, fives)
    }
}

proptest! {
    #[test]
    fn attribute_key_order((ones, twos, threes, fours, fives) in ordered_inputs(2)) {
        let args = &[
            AttributeKeyArgs{one: ones[0], two: twos[0], three: threes[0], four: fours[0], five: fives[0]},
            AttributeKeyArgs{one: ones[0], two: twos[0], three: threes[0], four: fours[0], five: fives[1]},
            AttributeKeyArgs{one: ones[0], two: twos[0], three: threes[0], four: fours[1], five: fives[0]},
            AttributeKeyArgs{one: ones[0], two: twos[0], three: threes[1], four: fours[0], five: fives[0]},
            AttributeKeyArgs{one: ones[0], two: twos[1], three: threes[0], four: fours[0], five: fives[0]},
            AttributeKeyArgs{one: ones[1], two: twos[0], three: threes[0], four: fours[0], five: fives[0]},
            AttributeKeyArgs{one: ones[1], two: twos[1], three: threes[0], four: fours[0], five: fives[0]},
            AttributeKeyArgs{one: ones[1], two: twos[1], three: threes[1], four: fours[0], five: fives[0]},
            AttributeKeyArgs{one: ones[1], two: twos[1], three: threes[1], four: fours[1], five: fives[0]},
            AttributeKeyArgs{one: ones[1], two: twos[1], three: threes[1], four: fours[1], five: fives[1]},
        ];

        // The attribute argument structures should be in the same order as the key structures.
        for i in 1..args.len() {
            assert!(args[i-1] < args[i]);
        }

        let keys: Vec<AttributeKey> = args.into_iter().map(|a| AttributeKey::from(a)).collect();

        for i in 1..keys.len() {
            assert!(keys[i-1] < keys[i]);
        }
    }
}

proptest! {
    #[test]
    fn derive_key_order((ones, twos, threes, fours, fives) in ordered_inputs(2)) {
        let args = &[
            Derive{one: ones[0], two: twos[0], three: threes[0], four: fours[0], five: fives[0]},
            Derive{one: ones[0], two: twos[0], three: threes[0], four: fours[0], five: fives[1]},
            Derive{one: ones[0], two: twos[0], three: threes[0], four: fours[1], five: fives[0]},
            Derive{one: ones[0], two: twos[0], three: threes[1], four: fours[0], five: fives[0]},
            Derive{one: ones[0], two: twos[1], three: threes[0], four: fours[0], five: fives[0]},
            Derive{one: ones[1], two: twos[0], three: threes[0], four: fours[0], five: fives[0]},
            Derive{one: ones[1], two: twos[1], three: threes[0], four: fours[0], five: fives[0]},
            Derive{one: ones[1], two: twos[1], three: threes[1], four: fours[0], five: fives[0]},
            Derive{one: ones[1], two: twos[1], three: threes[1], four: fours[1], five: fives[0]},
            Derive{one: ones[1], two: twos[1], three: threes[1], four: fours[1], five: fives[1]},
        ];

        // The derive argument structures should be in the same order as the key structures.
        for i in 1..args.len() {
            assert!(args[i-1] < args[i]);
        }

        let keys: Vec<DeriveKey> = args.into_iter().map(|a| DeriveKey::from(a)).collect();

        for i in 1..keys.len() {
            assert!(keys[i-1] < keys[i]);
        }
    }
}

proptest! {
    #[test]
    fn derive_hash_key(
        (ones, twos, threes, fours, fives) in ordered_inputs(2),
        fake_data in proptest::collection::vec(proptest::collection::vec(0..=u8::MAX, 20..50), 10..=10)
    ) {
        use std::collections::HashMap;

        let keys = &[
            DeriveKey::new(ones[0], twos[0], threes[0], fours[0], fives[0]),
            DeriveKey::new(ones[0], twos[0], threes[0], fours[0], fives[1]),
            DeriveKey::new(ones[0], twos[0], threes[0], fours[1], fives[0]),
            DeriveKey::new(ones[0], twos[0], threes[1], fours[0], fives[0]),
            DeriveKey::new(ones[0], twos[1], threes[0], fours[0], fives[0]),
            DeriveKey::new(ones[1], twos[0], threes[0], fours[0], fives[0]),
            DeriveKey::new(ones[1], twos[1], threes[0], fours[0], fives[0]),
            DeriveKey::new(ones[1], twos[1], threes[1], fours[0], fives[0]),
            DeriveKey::new(ones[1], twos[1], threes[1], fours[1], fives[0]),
            DeriveKey::new(ones[1], twos[1], threes[1], fours[1], fives[1]),
        ];

        let mut hash_map = HashMap::new();
        for (i, key) in keys.iter().enumerate() {
            hash_map.insert(key.clone(), fake_data[i].clone());
        }

        for i in 0..keys.len() {
            assert_eq!(hash_map.get(&keys[i]).unwrap(), &fake_data[i]);
        }
    }
}
