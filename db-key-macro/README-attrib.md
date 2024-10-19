# The `#[db_key]` attribute macro.

The `db_key` attribute macro genereates a key structure and a key arguments structure
from a prototype structure.

```rust
use db_key_macro::db_key;

#[db_key]
struct ExampleKey {
    byte: u8,
    word: u16,
    long: u32,
    quad: u64,
    bytes: [u8; 3],
};
```

The above code will generate ExampleKey and ExampleKeyArgs structures. New ExampleKey structures can be
created from `new()` and `from(ExampleKeyArgs)`.

```rust
# use db_key_macro::db_key;
# #[db_key]
# struct ExampleKey { byte: u8, word: u16, long: u32, quad: u64 };
let mut new_key = ExampleKey::new(0x12, 0x3456, 0x789ABCDE, 0xFEDCBA9876543210);
let from_key = ExampleKey::from(ExampleKeyArgs {
    quad: 0xFEDCBA9876543210,
    long: 0x789ABCDE,
    word: 0x3456,
    byte: 0x12,
});
assert_eq!(new_key, from_key);
```

The `db_key` macro automatically generates accessors for all of the fields defined in the
prototype structure.

```rust
# use db_key_macro::db_key;
# #[db_key]
# struct ExampleKey { byte: u8, word: u16, long: u32, quad: u64 };
# let new_key = ExampleKey::new(0x12, 0x3456, 0x789ABCDE, 0xFEDCBA9876543210);
assert_eq!(new_key.byte(), 0x12);
assert_eq!(new_key.word(), 0x3456);
assert_eq!(new_key.long(), 0x789ABCDE);
assert_eq!(new_key.quad(), 0xFEDCBA9876543210);
```

The `db_key` macro also automatically generates modifiers for all of the fields defined in
the prototype structure.

```rust
# use db_key_macro::db_key;
# #[db_key]
# struct ExampleKey { byte: u8, word: u16, long: u32, quad: u64 };
# let mut new_key = ExampleKey::new(0x12, 0x3456, 0x789ABCDE, 0xFEDCBA9876543210);
new_key.set_byte(0x21);
new_key.set_word(0x6543);
new_key.set_long(0xEDCBA987);
new_key.set_quad(0x123456789ABCDEF0);
assert_eq!(new_key.byte(), 0x21);
assert_eq!(new_key.word(), 0x6543);
assert_eq!(new_key.long(), 0xEDCBA987);
assert_eq!(new_key.quad(), 0x123456789ABCDEF0);
```

The `db_key` includes derives for `Copy`, `Clone`, `PartialEq`, `PartialOrd`, `Eq`, &
`Ord`. It also generates implementations for `Default` and `Debug` traits.

```rust
# use db_key_macro::db_key;
# #[db_key]
# struct ExampleKey { byte: u8, word: u16, long: u32, quad: u64 };
# let new_key = ExampleKey::new(0x21, 0x6543, 0xEDCBA987, 0x123456789ABCDEF0);
# let from_key = ExampleKey::new(0x12, 0x3456, 0x789ABCDE, 0xFEDCBA9876543210);
assert!(new_key != from_key);
let orig_key = from_key;
assert!(orig_key == from_key);
assert!(new_key > from_key);
assert!(orig_key < new_key);

let max_key = std::cmp::max(new_key, from_key);
assert!(new_key <= max_key);
assert!(from_key <= max_key);

let min_key = std::cmp::min(new_key, from_key);
assert!(new_key >= min_key);
assert!(from_key >= min_key);

let def_key = ExampleKey::default();
assert_eq!(def_key.byte(), 0);
assert_eq!(def_key.word(), 0);
assert_eq!(def_key.long(), 0);
assert_eq!(def_key.quad(), 0);
assert!(def_key < new_key);
assert!(def_key < from_key);

assert_eq!(&format!("{:?}", def_key),
    concat!("ExampleKey { byte: 0, word: 0, long: 0, quad: 0, raw: ",
        "0x00_0000_00000000_0000000000000000 }"));
assert_eq!(&format!("{:?}", from_key),
    concat!("ExampleKey { byte: 18, word: 13398, long: 2023406814, quad: 18364758544493064720",
        ", raw: 0x12_3456_789ABCDE_FEDCBA9876543210 }"));
assert_eq!(&format!("{:?}", new_key),
    concat!("ExampleKey { byte: 33, word: 25923, long: 3989547399, quad: 1311768467463790320",
        ", raw: 0x21_6543_EDCBA987_123456789ABCDEF0 }"));
assert_eq!(&format!("{:X?}", new_key),
    concat!("ExampleKey { byte: 21, word: 6543, long: EDCBA987, quad: 123456789ABCDEF0",
        ", raw: 0x21_6543_EDCBA987_123456789ABCDEF0 }"));
```

The `db_key` macro generates `PartialEq`, `PartialOrd`, and `AsRef` traits for \[u8\].

```rust
# use db_key_macro::db_key;
# #[db_key]
# struct ExampleKey { byte: u8, word: u16, long: u32, quad: u64 };
# let new_key = ExampleKey::new(0x21, 0x6543, 0xEDCBA987, 0x123456789ABCDEF0);
# let from_key = ExampleKey::new(0x12, 0x3456, 0x789ABCDE, 0xFEDCBA9876543210);
let new_slice = new_key.as_ref();
let from_slice = from_key.as_ref();
assert!(*new_slice == new_key);
assert!(new_key == *new_slice);
assert!(*new_slice > from_key);
assert!(new_key > *from_slice);
```

The `db_key` generates [From] implementations for the key from `&[u8]` & 
`[u8; KEY_LENGTH]`. It also generates [From] implementations for the key into
`[u8; KEY_LENGTH]`, and `Vec<u8>`.

```rust
# use db_key_macro::db_key;
# #[db_key]
# struct ExampleKey { byte: u8, word: u16, long: u32, quad: u64 };
# let new_key = ExampleKey::new(0x21, 0x6543, 0xEDCBA987, 0x123456789ABCDEF0);
# let from_key = ExampleKey::new(0x12, 0x3456, 0x789ABCDE, 0xFEDCBA9876543210);
let buf: Vec<u8> = new_key.into();
let array: [u8; ExampleKey::KEY_LENGTH] = new_key.into();
let from_vec_key = ExampleKey::from(&buf[..]);
let from_array_key = ExampleKey::from(array);
assert_eq!(new_key, from_vec_key);
assert_eq!(new_key, from_array_key);
```

# DB Key Options

## Change crate name in documentation (crate_name)

The `db_key` macro generates testable code examples as documentation for much of the
generated code. It adds a 'use' line and uses the crate name of the crate in which it is used
by default. The 'crate_name' option allows you to change the crate name used for the
documentation tests.

### Examples

```rust
use db_key_macro::db_key;

#[db_key(crate_name = "other_crate_name")]
struct TestKey {
    id: u64,
    other: u32,
}
```

The above code would generate the following `use` line in the documentation tests:

```text
use other_crate_name::{TestKey, TestKeyArgs};
```

## Change path in documentation (path)

There is currently no way (in stable) to get the source file information. If the `db_key`
macro is used in any other file than `src/lib.rs`, than you will need to either make the two
keys available from within `src/lib.rs` via `pub use path::to::{Key, KeyArgs};` or you can
manually specify the use path to the source file with the keys with the `path` option.

### Examples

```rust
use db_key_macro::db_key;

#[db_key(path = "source_file_dir::source_file_name")]
struct TestKey {
    id: u64,
    other: u32,
}
```

The above code would generate the following `use` line in the documentation tests:

```text
use crate_name::source_file_dir::source_file_name::{TestKey, TestKeyArgs};
```

## Use alternate argument structure name (`alt_name`)

By default, the `db_key` attribute macro creates an argument structure with the
name of the definition structure with `Args` appended to the end (i.e. The
argument structure for `Key` is `KeyArgs`). You can set a different argument
structure name with the `alt_name` option.

### Examples

```rust
use db_key_macro::db_key;

#[db_key(alt_name = Args)]
struct Key {
    id: u64,
    other: u32,
}

let from_new = Key::new(0x123456789ABCDEF0, 0);
let from_args = Key::from( Args {
    id: 0x123456789ABCDEF0,
    ..Default::default()
});

assert_eq!(from_new, from_args);
```

## Don't auto-generate new() (`no_new`)

Normally, the `db_key` macro generates a `new()` implementation that includes all of the
fields from the specification in order. If you don't want this autogenerated `new()` because
you want to manually create one or just don't want one, then use the `no_new` option.

### Examples

```rust
use db_key_macro::db_key;
use std::ops::Range;

#[db_key(no_new)]
struct NoNewKey {
    year: u16,
    month: u8,
    day: u8,
}

impl NoNewKey {
    const DATE_START: usize = Self::YEAR_START;
    const DATE_END: usize = Self::DAY_END;
    const DATE_RANGE: Range<usize> =
        Self::DATE_START..Self::DATE_END;

    // We want a new with a 32-bit value as input instead of the 16-bit word and 2 8-bit bytes.
    pub fn new(date: u32) -> Self {
        let mut buf = [0_u8; Self::KEY_LENGTH];
        buf[Self::DATE_RANGE].copy_from_slice(&date.to_be_bytes());
        Self(buf)
    }
}

let key = NoNewKey::new(0x07E8_05_0A);
assert_eq!(key.year(), 2024);
assert_eq!(key.month(), 5);
assert_eq!(key.day(), 10);
```

## Manually write Debug implementation (custom_debug)

Normally, the `db_key` macro generates a `Debug` implementation that includes all of the
fields from the specification in the output. If you want to write a custom Debug
implementation, then use the `custom_debug` option. There must be a Debug implementation.

There is a private function `raw_debug()` in the key to return the raw key formatted
like the default Debug implementation in case you wish to use it.

### Examples

```rust
use db_key_macro::db_key;

#[db_key(custom_debug)]
struct DateKey {
    user: u64,
    date: u32,
}

// We want the Debug output to display the year, month, and day instead of the date value.
impl std::fmt::Debug for DateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = self.date();
        let year = (date >> 16) as u16;
        let month = (date >> 8) as u8;
        let day = date as u8;
        f.debug_struct("DateKey")
            .field("user", &format_args!("{:#018X}", self.user()))
            .field("year", &year)
            .field("month", &month)
            .field("day", &day)
            .field("my_raw",
	    	&format_args!("0x{:08X}_{:04X}_{:02X}_{:02X}", self.user(), year, month, day))
            .field("default_raw", &self.raw_debug())
            .finish()
    }
}

let key = DateKey::new(0x123456789ABCDEF0, 0x0009_05_0A);
assert_eq!(&format!("{:?}", key),
    concat!("DateKey { user: 0x123456789ABCDEF0, year: 9, month: 5, day: 10, ",
        "my_raw: 0x123456789ABCDEF0_0009_05_0A, ",
        "default_raw: 0x123456789ABCDEF0_0009050A }"));
```

## Select Debug output of raw key data (`raw_debug`)

There are six options for the output of the raw key data:

### Compact output (`compact`) (default)

The compact output is the default for displaying the raw key data. You can also
explicitly select it with the `compact` option for `raw_debug`. It is a custom
format that display the parts of the key (in big endian hexidecimal) with
underscores demarcating the breaks between the parts of the key.

```rust
# use db_key_macro::db_key;
# #[db_key]
# struct Key { w: u16, b: u8, l: u32, q: u64 }
# assert_eq!(
#   &format!("\nraw: {:?}\n", Key::new(0x1234, 0x56, 0x789ABCDE, 0xF0123456789ABCDE).raw_debug()), "
raw: 0x1234_56_789ABCDE_F0123456789ABCDE
# ");
```

```rust
use db_key_macro::db_key;

#[db_key]
struct Key {
    #[default = 0x1234]
    first: u16,
    #[default = 0x56]
    second: u8,
    #[default = 0x789ABCDE]
    third: u32,
    #[default = 0xF0123456789ABCDE]
    fourth: u64,
}

assert_eq!(&format!("{:?}", Key::default()),
    concat!("Key { first: 4660, second: 86, third: 2023406814, fourth: 17298946664678735070, ",
    "raw: 0x1234_56_789ABCDE_F0123456789ABCDE }"));
```

### Standard output (`std`)

The standard output displays the raw key value in the standard way that arrays
of bytes are displayed.

```rust
# use db_key_macro::db_key;
# #[db_key(raw_debug = std)]
# struct Key { w: u16, b: u8, l: u32, q: u64 }
# assert_eq!(
#   &format!("\nraw: {:?}\n", Key::new(0x1234, 0x56, 0x789ABCDE, 0xF0123456789ABCDE).raw_debug()), "
raw: [18, 52, 86, 120, 154, 188, 222, 240, 18, 52, 86, 120, 154, 188, 222]
# ");
```

```rust
use db_key_macro::db_key;

#[db_key(raw_debug = std)]
struct Key {
    #[default = 0x1234]
    first: u16,
    #[default = 0x56]
    second: u8,
    #[default = 0x789ABCDE]
    third: u32,
    #[default = 0xF0123456789ABCDE]
    fourth: u64,
}

assert_eq!(&format!("{:?}", Key::default()),
    concat!("Key { first: 4660, second: 86, third: 2023406814, fourth: 17298946664678735070, ",
        "raw: [18, 52, 86, 120, 154, 188, 222, 240, 18, 52, 86, 120, 154, 188, 222] }"));
```

### Standard lowercase hexadecimal output (`lower_hex`)

The hex output displays the raw key value in the standard way that arrays are
displayed, but the values are in lowercase hexadecimal no matter what format
the rest of the values are in.

```rust
# use db_key_macro::db_key;
# #[db_key(raw_debug = lower_hex)]
# struct Key { w: u16, b: u8, l: u32, q: u64 }
# assert_eq!(
#   &format!("\nraw: {:?}\n", Key::new(0x1234, 0x56, 0x789ABCDE, 0xF0123456789ABCDE).raw_debug()), "
raw: [12, 34, 56, 78, 9a, bc, de, f0, 12, 34, 56, 78, 9a, bc, de]
# ");
```

```rust
use db_key_macro::db_key;

#[db_key(raw_debug = lower_hex)]
struct Key {
    #[default = 0x1234]
    first: u16,
    #[default = 0x56]
    second: u8,
    #[default = 0x789ABCDE]
    third: u32,
    #[default = 0xF0123456789ABCDE]
    fourth: u64,
}

assert_eq!(&format!("{:?}", Key::default()),
    concat!("Key { first: 4660, second: 86, third: 2023406814, fourth: 17298946664678735070, ",
        "raw: [12, 34, 56, 78, 9a, bc, de, f0, 12, 34, 56, 78, 9a, bc, de] }"));
```

### Standard uppercase hexadecimal output (`upper_hex`)

The hex output displays the raw key value in the standard way that arrays are
displayed, but the values are in uppercase hexadecimal no matter what format
the rest of the values are in.

```rust
# use db_key_macro::db_key;
# #[db_key(raw_debug = upper_hex)]
# struct Key { w: u16, b: u8, l: u32, q: u64 }
# assert_eq!(
#   &format!("\nraw: {:?}\n", Key::new(0x1234, 0x56, 0x789ABCDE, 0xF0123456789ABCDE).raw_debug()), "
raw: [12, 34, 56, 78, 9A, BC, DE, F0, 12, 34, 56, 78, 9A, BC, DE]
# ");
```

```rust
use db_key_macro::db_key;

#[db_key(raw_debug = upper_hex)]
struct Key {
    #[default = 0x1234]
    first: u16,
    #[default = 0x56]
    second: u8,
    #[default = 0x789ABCDE]
    third: u32,
    #[default = 0xF0123456789ABCDE]
    fourth: u64,
}

assert_eq!(&format!("{:?}", Key::default()),
    concat!("Key { first: 4660, second: 86, third: 2023406814, fourth: 17298946664678735070, ",
        "raw: [12, 34, 56, 78, 9A, BC, DE, F0, 12, 34, 56, 78, 9A, BC, DE] }"));
```

### Pretty hexadecimal output (`pretty_lower_hex`)

The hex output displays the raw key value as an array of bytes, but it make clear that the output
is displayed in lowercase hexadecimal by prepending a '0x' to each byte.

```rust
# use db_key_macro::db_key;
# #[db_key(raw_debug = pretty_lower_hex)]
# struct Key { w: u16, b: u8, l: u32, q: u64 }
# assert_eq!(
#   &format!("\nraw: {:?}\n", Key::new(0x1234, 0x56, 0x789ABCDE, 0xF0123456789ABCDE).raw_debug()), "
raw: [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde]
# ");
```

```rust
use db_key_macro::db_key;

#[db_key(raw_debug = pretty_lower_hex)]
struct Key {
    #[default = 0x1234]
    first: u16,
    #[default = 0x56]
    second: u8,
    #[default = 0x789ABCDE]
    third: u32,
    #[default = 0xF0123456789ABCDE]
    fourth: u64,
}

assert_eq!(&format!("{:?}", Key::default()),
    concat!("Key { first: 4660, second: 86, third: 2023406814, fourth: 17298946664678735070, ",
        "raw: [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde] }"));
```

### Pretty hexadecimal output (`pretty_upper_hex`)

The hex output displays the raw key value as an array of bytes, but it make clear that the output
is displayed in uppercase hexadecimal by prepending a '0x' to each byte.

```rust
# use db_key_macro::db_key;
# #[db_key(raw_debug = pretty_upper_hex)]
# struct Key { w: u16, b: u8, l: u32, q: u64 }
# assert_eq!(
#   &format!("\nraw: {:?}\n", Key::new(0x1234, 0x56, 0x789ABCDE, 0xF0123456789ABCDE).raw_debug()), "
raw: [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE]
# ");
```

```rust
use db_key_macro::db_key;

#[db_key(raw_debug = pretty_upper_hex)]
struct Key {
    #[default = 0x1234]
    first: u16,
    #[default = 0x56]
    second: u8,
    #[default = 0x789ABCDE]
    third: u32,
    #[default = 0xF0123456789ABCDE]
    fourth: u64,
}

assert_eq!(&format!("{:?}", Key::default()),
    concat!("Key { first: 4660, second: 86, third: 2023406814, fourth: 17298946664678735070, ",
        "raw: [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE] }"));
```

# Field Attributes

## Field name (name)

This allows you to define the name that will be used in the documentation. If it is not
defined, then it will default to the field name.

### Examples

```rust
use db_key_macro::db_key;

#[db_key]
struct DateKey {
    /// The event ID for the data item.
    #[name = "User ID"]    // The name is used in the generated documentation.
    user: u64,
    /// The date data item contains the year in the high 16-bits, the month in
    /// the high byte of the low 16-bit word, and the day in the low byte.
    #[name = "Date (year, month, and day)"]
    date: u32,
}
```

## Default value (default)

The default field attruibute allows you to change the value that is used for the default.

### Examples

```rust
use db_key_macro::db_key;

#[db_key]
struct DefaultKey {
    #[default = 0x123456789ABCDEF0]
    quad: u64,
    #[default = 0xFEDCBA98]
    long: u32,
    #[default = 0x1234]
    word: u16,
    byte: u8,
}

let def_key = DefaultKey::default();

assert_eq!(def_key.quad(), 0x123456789ABCDEF0);
assert_eq!(def_key.long(), 0xFEDCBA98);
assert_eq!(def_key.word(), 0x1234);
assert_eq!(def_key.byte(), 0);

let part_key = DefaultKey::from( DefaultKeyArgs {
    long: 0x12345678,
    byte: 0x5A,
    ..Default::default()
});

assert_eq!(part_key.quad(), 0x123456789ABCDEF0);
assert_eq!(part_key.long(), 0x12345678);
assert_eq!(part_key.word(), 0x1234);
assert_eq!(part_key.byte(), 0x5A);
```