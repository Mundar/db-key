#![doc = include_str!("../README.md")]
use proc_macro::TokenStream;
use syn::{
    self,
    parse_macro_input,
    DeriveInput,
};

mod parse;
mod field;

use crate::parse::DBKey;

#[doc = include_str!("../README-attrib.md")]
#[proc_macro_attribute]
pub fn db_key(attr: TokenStream, input: TokenStream) -> TokenStream {
    // This needs to be done here because any errors are output as a TokenStream.
    let input = parse_macro_input!(input as DeriveInput);
    let db_key = DBKey::attribute(attr.into(), input);
    db_key.generate().into()
}

#[doc = include_str!("../README-derive.md")]
#[proc_macro_derive(DBKey, attributes(key, default, name))]
pub fn db_key_derive(input: TokenStream) -> TokenStream {
    // This needs to be done here because any errors are output as a TokenStream.
    let input = parse_macro_input!(input as DeriveInput);
    let db_key = DBKey::derive(input);
    db_key.generate().into()
}
