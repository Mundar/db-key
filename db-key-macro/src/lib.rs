#![doc = include_str!("../README.md")]
#![forbid(future_incompatible)]
#![warn(missing_docs, missing_debug_implementations, bare_trait_objects)]

use proc_macro::TokenStream;
use syn::{
    self,
    parse_macro_input,
    Attribute,
    DeriveInput,
    ItemMod,
    parse_quote,
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

/// Add a documenation line to a vector of `Attribute`.
pub(crate) fn add_doc(docs: &mut Vec<Attribute>, doc: &str) {
    // There is probably a better way to do this, but this was the easiest way I found to do so.
    let item_mod: ItemMod = parse_quote! {
        #[doc = #doc]
        mod doc;
    };
    for attr in item_mod.attrs.iter() {
        if attr.path().is_ident("doc") {
            docs.push(attr.clone());
        }
    }
}
