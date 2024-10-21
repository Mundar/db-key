# DBKey Macros

The db-key macros generate a structure to be used as the fixed-size key for a
key-value database. Both macros take a prototype (or definition) structure and
create a tuple struture wrapped around an array.

```rust
use db_key_macro::db_key;

#[db_key]
struct ExampleKey {
    byte: u8,
    long: u32,
    array: [u8; 3],
}

let key = ExampleKey::new(0x12, 0x3456789A, [0xBC, 0xDE, 0xF0]);

assert_eq!(key.long(), 0x3456789A);
assert_eq!(key.array(), &[0xBC, 0xDE, 0xF0]);
assert_eq!(key.as_ref(), &[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]);
```

Currently, the data types that can be used in the definition structure are
limited to unsigned integers (except `usize`), and arrays of u8. The key data
is always packed and lexographically ordered so that the definition structure
and the key structure will have the same order when sorted.

There are two `DBKey` macros supplied by this crate: an attribute macro
(`#[db_key]`) and a derive macro (`DBKey`). They each provide similar
functionality of creating a fixed-length key type for key-value databases that
use a slice of bytes as the key. The generated code is documented with
examples.

For clarification the definition structure is the struture used to define the
key, the key struture is the tuple structure wrapping an array, and the
argument struture provides an easy way to define a key from the individual
components.

```rust
# use db_key_macro::db_key;
// Definition struture:
#[db_key]
struct ExampleKey {
    byte: u8,
    long: u32,
    array: [u8; 3],
}
```

```rust
// The generated key struture:
struct ExampleKey([u8; 8]);
```

```rust
// The (generated) argument struture:
struct ExampleKeyArgs {
    pub byte: u8,
    pub long: u32,
    pub array: [u8; 3],
}
```

The attribute macro consumes the definition structure and generates both a key
struture and an argument structure. The derive macro only generates a key
struture and the definition struture can be used an the argument struture.

With the attribute macro, the name of the definition structure is used for the
key structure and the argument structure uses the definition structure name
with `Args` appended to the end. With the derive macro, the definition
structure becomes the argument structure, and the key structure is given the
name of the definition structure with `Key` appended to the end.

Macro Type | Definition Name | Key Struct Name | Argument Struct Name |
-|-|-|-
Attribute | ExampleKey | ExampleKey | ExampleKeyArgs |
Derive | Example | ExampleKey | Example |

The key options for the attribute macro are defined in the attribute macro
call.

```rust
# use db_key_macro::db_key;
#[db_key(no_new, custom_debug, alt_name = Args)]
struct Key {
    id: u64,
    index: u32,
}
```

The key options for the derive macro are defined in a separate `key` attribute
macro.

```rust
# use db_key_macro::DBKey;
#[derive(Clone, DBKey, PartialEq, Eq, PartialOrd, Ord)]
#[key(no_new, custom_debug, alt_name = Key)]
struct Args {
    pub id: u64,
    pub index: u32,
}
```

The key struture always derives `Clone`, `Hash`, `PartialEq`, `Eq`,
`PartialOrd`, and `Ord`. If the size of the key is 64-bytes or less, than it
also derives `Copy` as well. A custom `Debug` implementation is supplied for
the key structure. For the attribute macro, the argument structure has all the
same derived implementations except `Hash` and it uses the standard derived
`Debug`.

Since the attribute macro replaces the definition structure, it is more
permissive about the data that is allowed in the definition strucure.

```rust
# use db_key_macro::db_key;
#[db_key(no_new, custom_debug, alt_name = Args)]
struct Key {
    /// The ID number.
    #[name = "ID"]
    // This isn't allowed in the derive macro. It will trigger an error.
    #[default = 0x123456789ABCDEF_u64]
    // The attribute macro doesn't need this to be public.
    id: u64,
    /// The item index
    #[name = "index value"]
    // This isn't allowed in the derive macro. It will trigger an error.
    #[default = 1_u32]
    index: u32,
    /// The array value
    #[name = "array value"]
    // This isn't allowed in the derive macro. It will trigger an error.
    #[default = [1, 2, 3]]
    array: [u8; 3],
}
```

Since the derive macro uses the definition structure as the argument structure,
it is more restrictive about what is expected.

```rust
# use db_key_macro::DBKey;
#[derive(Copy, Clone, DBKey, PartialEq, Eq, PartialOrd, Ord)]
#[key(no_new, custom_debug, alt_name = Key)]
struct Args {
    /// The ID number.
    #[name = "ID"]
    // The derive macro will accept raw values but not with type specifier.
    #[default = 0x123456789ABCDEF]
    // The derive macro need this to be public in order to use it.
    pub id: u64,
    /// The item index
    #[name = "index value"]
    // You can get around the limitation for default values by using a &str.
    #[default = "1_u32"]
    pub index: u32,
    // You can get around the limitation for default values by using a &str.
    #[default = "[1, 2, 3]"]
    array: [u8; 3],
}
```
