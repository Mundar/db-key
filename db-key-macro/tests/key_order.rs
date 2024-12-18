use db_key_macro::{DBKey, db_key};
use proptest::prelude::*;

#[db_key]
struct AttributeKey {
    one: u64,
    two: u32,
    three: u16,
    four: u8,
    five: [u8; 3],
    six: i64,
    seven: i32,
    eight: i16,
    nine: i8,
    ten: u128,
    eleven: i128,
}

#[derive(Copy, Clone, DBKey, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Derive {
    one: u64,
    two: u32,
    three: u16,
    four: u8,
    five: [u8; 3],
    six: i64,
    seven: i32,
    eight: i16,
    nine: i8,
    ten: u128,
    eleven: i128,
}

#[derive(Clone, Debug, Default)]
struct OrderedInputs {
    ones: Vec<u64>,
    twos: Vec<u32>,
    threes: Vec<u16>,
    fours: Vec<u8>,
    fives: Vec<[u8; 3]>,
    sixes: Vec<i64>,
    sevens: Vec<i32>,
    eights: Vec<i16>,
    nines: Vec<i8>,
    tens: Vec<u128>,
    elevens: Vec<i128>,
}

prop_compose! {
    fn ordered_inputs(count: usize)(
        ones in proptest::collection::btree_set(0..=u64::MAX, count..=count),
        twos in proptest::collection::btree_set(0..=u32::MAX, count..=count),
        threes in proptest::collection::btree_set(0..=u16::MAX, count..=count),
        fours in proptest::collection::btree_set(0..=u8::MAX, count..=count),
        fives in proptest::collection::btree_set([0..=u8::MAX, 0..=u8::MAX, 0..=u8::MAX], count..=count),
        sixes in proptest::collection::btree_set(i64::MIN..=i64::MAX, count..=count),
        sevens in proptest::collection::btree_set(i32::MIN..=i32::MAX, count..=count),
        eights in proptest::collection::btree_set(i16::MIN..=i16::MAX, count..=count),
        nines in proptest::collection::btree_set(i8::MIN..=i8::MAX, count..=count),
        tens in proptest::collection::btree_set(0..=u128::MAX, count..=count),
        elevens in proptest::collection::btree_set(i128::MIN..=i128::MAX, count..=count),
    ) -> OrderedInputs {
        OrderedInputs {
            ones: ones.iter().map(|v| *v).collect(),
            twos: twos.iter().map(|v| *v).collect(),
            threes: threes.iter().map(|v| *v).collect(),
            fours: fours.iter().map(|v| *v).collect(),
            fives: fives.iter().map(|v| *v).collect(),
            sixes: sixes.iter().map(|v| *v).collect(),
            sevens: sevens.iter().map(|v| *v).collect(),
            eights: eights.iter().map(|v| *v).collect(),
            nines: nines.iter().map(|v| *v).collect(),
            tens: tens.iter().map(|v| *v).collect(),
            elevens: elevens.iter().map(|v| *v).collect(),
        }
    }
}

impl OrderedInputs {
    fn attribute_arg(&self, indexes: [usize; 11]) -> AttributeKeyArgs {
        AttributeKeyArgs {
            one: self.ones[indexes[0]],
            two: self.twos[indexes[1]],
            three: self.threes[indexes[2]],
            four: self.fours[indexes[3]],
            five: self.fives[indexes[4]],
            six: self.sixes[indexes[5]],
            seven: self.sevens[indexes[6]],
            eight: self.eights[indexes[7]],
            nine: self.nines[indexes[8]],
            ten: self.tens[indexes[9]],
            eleven: self.elevens[indexes[10]],
        }
    }

    fn derive_arg(&self, indexes: [usize; 11]) -> Derive {
        Derive {
            one: self.ones[indexes[0]],
            two: self.twos[indexes[1]],
            three: self.threes[indexes[2]],
            four: self.fours[indexes[3]],
            five: self.fives[indexes[4]],
            six: self.sixes[indexes[5]],
            seven: self.sevens[indexes[6]],
            eight: self.eights[indexes[7]],
            nine: self.nines[indexes[8]],
            ten: self.tens[indexes[9]],
            eleven: self.elevens[indexes[10]],
        }
    }
}

const INPUT_SIZE: usize = 22;

macro_rules! impl_key_tests {
    ($(($arg_name:ident, $key_name:ident, $fn_name:ident, $order_test:ident, $hash_test:ident, $arg_fn:ident),)*) => {
        $(
            prop_compose! {
                fn $fn_name(count: usize)(inputs in ordered_inputs(count)) -> Vec<$arg_name> {
                    let mut keys = Vec::new();
                    for one in inputs.ones.iter() {
                        for two in inputs.twos.iter() {
                            for three in inputs.threes.iter() {
                                for four in inputs.fours.iter() {
                                    for five in inputs.fives.iter() {
                                        for six in inputs.sixes.iter() {
                                            for seven in inputs.sevens.iter() {
                                                for eight in inputs.eights.iter() {
                                                    for nine in inputs.nines.iter() {
                                                        for ten in inputs.tens.iter() {
                                                            for eleven in inputs.elevens.iter() {
                                                                keys.push($arg_name {
                                                                    one: *one,
                                                                    two: *two,
                                                                    three: *three,
                                                                    four: *four,
                                                                    five: *five,
                                                                    six: *six,
                                                                    seven: *seven,
                                                                    eight: *eight,
                                                                    nine: *nine,
                                                                    ten: *ten,
                                                                    eleven: *eleven,
                                                                });
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    keys
                }
            }

            proptest! {
                #[test]
                fn $order_test(args in $fn_name(2)) {
                    // The attribute argument structures should be in the same order as the key structures.
                    for i in 1..args.len() {
                        prop_assert!(args[i-1] < args[i]);
                    }

                    let keys: Vec<$arg_name> = args.into_iter().map(|a| $arg_name::from(a)).collect();

                    for i in 1..keys.len() {
                        prop_assert!(keys[i-1] < keys[i], "Failed on key[{i}]: {:?} < {:?}", keys[i-1],
                            keys[i]);
                    }
                }
            }

            proptest! {
                #[test]
                fn $hash_test(
                    inputs in ordered_inputs(2),
                    fake_data in proptest::collection::vec(proptest::collection::vec(0..=u8::MAX, 20..50), INPUT_SIZE..=INPUT_SIZE)
                ) {
                    use std::collections::HashMap;
                    let mut args = Vec::with_capacity(INPUT_SIZE);
                    args.push(inputs.$arg_fn([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
                    args.push(inputs.$arg_fn([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]));
                    args.push(inputs.$arg_fn([0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]));
                    args.push(inputs.$arg_fn([0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0]));
                    args.push(inputs.$arg_fn([0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0]));
                    args.push(inputs.$arg_fn([0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0]));
                    args.push(inputs.$arg_fn([0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0]));
                    args.push(inputs.$arg_fn([0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]));
                    args.push(inputs.$arg_fn([0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]));
                    args.push(inputs.$arg_fn([0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]));
                    args.push(inputs.$arg_fn([0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
                    args.push(inputs.$arg_fn([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
                    args.push(inputs.$arg_fn([1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
                    args.push(inputs.$arg_fn([1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]));
                    args.push(inputs.$arg_fn([1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0]));
                    args.push(inputs.$arg_fn([1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0]));
                    args.push(inputs.$arg_fn([1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0]));
                    args.push(inputs.$arg_fn([1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0]));
                    args.push(inputs.$arg_fn([1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0]));
                    args.push(inputs.$arg_fn([1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0]));
                    args.push(inputs.$arg_fn([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0]));
                    args.push(inputs.$arg_fn([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]));
                    prop_assert_eq!(args.len(), INPUT_SIZE);

                    let keys: Vec<$key_name> = args.into_iter().map(|a| $key_name::from(a)).collect();

                    let mut hash_map = HashMap::new();
                    for (i, key) in keys.iter().enumerate() {
                        hash_map.insert(key.clone(), fake_data[i].clone());
                    }

                    for i in 0..keys.len() {
                        prop_assert_eq!(hash_map.get(&keys[i]).unwrap(), &fake_data[i]);
                    }
                }
            }

        )*
    };
}

impl_key_tests!{
    (AttributeKeyArgs, AttributeKey, attribute_keys, attribute_key_order, attribute_hash_key, attribute_arg),
    (Derive, DeriveKey, derive_keys, derive_key_order, derive_hash_key, derive_arg),
}
