//! Sample keys testing the different raw key debug formats.

use db_key_macro::db_key;

#[db_key(path="attrib::debug", raw_debug = std)]
pub struct StdDebugKey {
    pub byte: u8,
    pub word: u16,
    pub long: u32,
    #[default = 0x123456789ABCDEF0]
    pub quad: u64,
    pub octo: u128,
    #[default = "[0xFF; 5]"]
    pub array: [u8; 5],
}

#[db_key(path="attrib::debug", raw_debug = lower_hex)]
pub struct LowerHexDebugKey {
    pub byte: u8,
    pub word: u16,
    pub long: u32,
    #[default = 0x123456789ABCDEF0]
    pub quad: u64,
    pub octo: u128,
    #[default = "[0xFF; 5]"]
    pub array: [u8; 5],
}

#[db_key(path="attrib::debug", raw_debug = upper_hex)]
pub struct UpperHexDebugKey {
    pub byte: u8,
    pub word: u16,
    pub long: u32,
    #[default = 0x123456789ABCDEF0]
    pub quad: u64,
    pub octo: u128,
    #[default = "[0xFF; 5]"]
    pub array: [u8; 5],
}

#[db_key(path="attrib::debug", raw_debug = pretty_lower_hex)]
pub struct PrettyLowerHexDebugKey {
    pub byte: u8,
    pub word: u16,
    pub long: u32,
    #[default = 0x123456789ABCDEF0]
    pub quad: u64,
    pub octo: u128,
    #[default = "[0xFF; 5]"]
    pub array: [u8; 5],
}

#[db_key(path="attrib::debug", raw_debug = pretty_upper_hex)]
pub struct PrettyUpperHexDebugKey {
    pub byte: u8,
    pub word: u16,
    pub long: u32,
    #[default = 0x123456789ABCDEF0]
    pub quad: u64,
    pub octo: u128,
    #[default = "[0xFF; 5]"]
    pub array: [u8; 5],
}
