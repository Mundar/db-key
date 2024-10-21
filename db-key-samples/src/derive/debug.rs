//! Sample keys testing the different raw key debug formats.

use db_key_macro::DBKey;

#[derive(DBKey)]
#[key(path="derive::debug", raw_debug = std)]
pub struct StdDebug {
    pub byte: u8,
    pub word: u16,
    pub long: u32,
    #[default = 0x123456789ABCDEF0]
    pub quad: u64,
    pub octo: u128,
    #[default = "[0xFF; 5]"]
    pub array: [u8; 5],
}

#[derive(DBKey)]
#[key(path="derive::debug", raw_debug = lower_hex)]
pub struct LowerHexDebug {
    pub byte: u8,
    pub word: u16,
    pub long: u32,
    #[default = 0x123456789ABCDEF0]
    pub quad: u64,
    pub octo: u128,
    #[default = "[0xFF; 5]"]
    pub array: [u8; 5],
}

#[derive(DBKey)]
#[key(path="derive::debug", raw_debug = upper_hex)]
pub struct UpperHexDebug {
    pub byte: u8,
    pub word: u16,
    pub long: u32,
    #[default = 0x123456789ABCDEF0]
    pub quad: u64,
    pub octo: u128,
    #[default = "[0xFF; 5]"]
    pub array: [u8; 5],
}

#[derive(DBKey)]
#[key(path="derive::debug", raw_debug = pretty_lower_hex)]
pub struct PrettyLowerHexDebug {
    pub byte: u8,
    pub word: u16,
    pub long: u32,
    #[default = 0x123456789ABCDEF0]
    pub quad: u64,
    pub octo: u128,
    #[default = "[0xFF; 5]"]
    pub array: [u8; 5],
}

#[derive(DBKey)]
#[key(path="derive::debug", raw_debug = pretty_upper_hex)]
pub struct PrettyUpperHexDebug {
    pub byte: u8,
    pub word: u16,
    pub long: u32,
    #[default = 0x123456789ABCDEF0]
    pub quad: u64,
    pub octo: u128,
    #[default = "[0xFF; 5]"]
    pub array: [u8; 5],
}
