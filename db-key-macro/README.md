# DBKey Macros

There are two `DBKey` macros supplied by this crate: an attribute macro
(`#[db_key]`) and a derive macro (`DBKey`). They each provide similar
functionality of creating a fixed-length key type for key-value databases that
use a slice of bytes as the key.

Both macro types work on a struct (the definition structure) that contains
unsigned integers and/or arrays of unsigned bytes. The key that is generated is
a tuple structure with a fixed-length array determined by the contents of the
structure. The attribute macro also generates an argument structure that can be
used to define a key structure from the parts of the key. The attribute macro
replaces the definition structure, whereas with the derive macro, the
definition structure becomes the argument structure.

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
# struct Key { id: u64, index: u32 }
```

The key options for the derive macro are defined in a separate `key` attribute
macro.

```rust
# use db_key_macro::DBKey;
#[derive(DBKey)]
#[key(no_new, custom_debug, alt_name = Key)]
# struct Args { pub id: u64, pub index: u32 }
```

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
#[derive(DBKey)]
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
