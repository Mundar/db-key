//! Sample keys testing the different raw key debug formats.

use db_key_macro::db_key;

/// A key using the standard raw debug format.
#[db_key(path="attrib::debug", raw_debug = std)]
pub struct StdDebugKey {
    /// A u8 value.
	pub byte: u8,
    /// A u16 value.
	pub word: u16,
    /// A u32 value.
	pub long: u32,
    #[default = 0x123456789ABCDEF0]
    /// A u64 value.
	pub quad: u64,
    /// A u128 value.
	pub octo: u128,
    #[default = "[0xFF; 5]"]
    /// A 5-byte array value.
	pub array: [u8; 5],
}

/// A key using the lowercase hexadecimal raw debug format.
#[db_key(path="attrib::debug", raw_debug = lower_hex)]
pub struct LowerHexDebugKey {
    /// A u8 value.
	pub byte: u8,
    /// A u16 value.
	pub word: u16,
    /// A u32 value.
	pub long: u32,
    #[default = 0x123456789ABCDEF0]
    /// A u64 value.
	pub quad: u64,
    /// A u128 value.
	pub octo: u128,
    #[default = "[0xFF; 5]"]
    /// A 5-byte array value.
	pub array: [u8; 5],
}

/// A key using the uppercase hexadecimal raw debug format.
#[db_key(path="attrib::debug", raw_debug = upper_hex)]
pub struct UpperHexDebugKey {
    /// A u8 value.
	pub byte: u8,
    /// A u16 value.
	pub word: u16,
    /// A u32 value.
	pub long: u32,
    #[default = 0x123456789ABCDEF0]
    /// A u64 value.
	pub quad: u64,
    /// A u128 value.
	pub octo: u128,
    #[default = "[0xFF; 5]"]
    /// A 5-byte array value.
	pub array: [u8; 5],
}

/// A key using the pretty lowercase hexadecimal raw debug format.
#[db_key(path="attrib::debug", raw_debug = pretty_lower_hex)]
pub struct PrettyLowerHexDebugKey {
    /// A u8 value.
	pub byte: u8,
    /// A u16 value.
	pub word: u16,
    /// A u32 value.
	pub long: u32,
    #[default = 0x123456789ABCDEF0]
    /// A u64 value.
	pub quad: u64,
    /// A u128 value.
	pub octo: u128,
    #[default = "[0xFF; 5]"]
    /// A 5-byte array value.
	pub array: [u8; 5],
}

/// A key using the pretty uppercase hexadecimal raw debug format.
#[db_key(path="attrib::debug", raw_debug = pretty_upper_hex)]
pub struct PrettyUpperHexDebugKey {
    /// A u8 value.
	pub byte: u8,
    /// A u16 value.
	pub word: u16,
    /// A u32 value.
	pub long: u32,
    #[default = 0x123456789ABCDEF0]
    /// A u64 value.
	pub quad: u64,
    /// A u128 value.
	pub octo: u128,
    #[default = "[0xFF; 5]"]
    /// A 5-byte array value.
	pub array: [u8; 5],
}
