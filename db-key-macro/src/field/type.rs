use proc_macro2::{
    TokenStream,
};
use quote::{quote, ToTokens};
use syn::{
    self,
    Error,
    Expr,
    Field,
    Lit,
    Result,
    spanned::Spanned,
    Type,
};
use std::{
    str::FromStr,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FieldSize {
    Signed8,
    Signed16,
    Signed32,
    Signed64,
    Signed128,
    Unsigned8,
    Unsigned16,
    Unsigned32,
    Unsigned64,
    Unsigned128,
    Array(usize),
}

impl FieldSize {
    /// Return the size of the field type in bytes.
    pub fn size(&self) -> usize {
        match self {
            FieldSize::Signed8 => 1,
            FieldSize::Signed16 => 2,
            FieldSize::Signed32 => 4,
            FieldSize::Signed64 => 8,
            FieldSize::Signed128 => 16,
            FieldSize::Unsigned8 => 1,
            FieldSize::Unsigned16 => 2,
            FieldSize::Unsigned32 => 4,
            FieldSize::Unsigned64 => 8,
            FieldSize::Unsigned128 => 16,
            FieldSize::Array(size) => *size,
        }
    }
}

impl std::fmt::Display for FieldSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldSize::Signed8 => f.write_str("1"),
            FieldSize::Signed16 => f.write_str("2"),
            FieldSize::Signed32 => f.write_str("4"),
            FieldSize::Signed64 => f.write_str("8"),
            FieldSize::Signed128 => f.write_str("16"),
            FieldSize::Unsigned8 => f.write_str("1"),
            FieldSize::Unsigned16 => f.write_str("2"),
            FieldSize::Unsigned32 => f.write_str("4"),
            FieldSize::Unsigned64 => f.write_str("8"),
            FieldSize::Unsigned128 => f.write_str("16"),
            FieldSize::Array(size) => write!(f, "{}", size),
        }
    }
}

#[derive(Debug)]
pub struct FieldType {
    pub field_type: Type,
    pub string: String,
    pub size: FieldSize,
}

impl TryFrom<&Field> for FieldType {
    type Error = Error;

    fn try_from(field: &Field) -> Result<Self> {
        const ERROR_STR: &'static str = "Unsupported field type for db_key";
        const ERROR_ZERO: &'static str = "Unsupported array size for db_key";
        let field_type = field.ty.clone();  // We always need a clone of this on success.
        match &field_type {
            Type::Path(path) => {
                let ident = match path.path.get_ident() {
                    Some(ident) => ident.clone(),
                    None => { return Err(Error::new(field_type.span(), ERROR_STR)); }
                };
                let string = ident.to_string();
                let size = match string.as_str() {
                    "i8" => FieldSize::Signed8,
                    "i16" => FieldSize::Signed16,
                    "i32" => FieldSize::Signed32,
                    "i64" => FieldSize::Signed64,
                    "i128" => FieldSize::Signed128,
                    "u8" => FieldSize::Unsigned8,
                    "u16" => FieldSize::Unsigned16,
                    "u32" => FieldSize::Unsigned32,
                    "u64" => FieldSize::Unsigned64,
                    "u128" => FieldSize::Unsigned128,
                    _ => { return Err(Error::new(field_type.span(), ERROR_STR)); }
                };
                Ok(Self {
                    field_type,
                    string,
                    size,
                })
            }
            Type::Array(array) => {
                match &*array.elem {
                    Type::Path(path) => match path.path.get_ident() {
                        Some(ident) => if "u8" != &format!("{}", ident) {
                            return Err(Error::new(ident.span(), ERROR_STR));
                        }
                        None => { return Err(Error::new(path.span(), ERROR_STR)); }
                    }
                    _ => {
                        return Err(Error::new(array.elem.span(), ERROR_STR));
                    }

                }
                let size = match &array.len {
                    Expr::Lit(expr_lit) => match &expr_lit.lit {
                        Lit::Int(lit_int) => {
                            match usize::from_str(lit_int.base10_digits()) {
                                Ok(0) => { return Err(Error::new(lit_int.span(), ERROR_ZERO)); },
                                Ok(size) => FieldSize::Array(size),
                                Err(_) => { return Err(Error::new(lit_int.span(), ERROR_STR)); }
                            }
                        }
                        _ => { return Err(Error::new(expr_lit.span(), ERROR_STR)); }
                    }
                    _ => { return Err(Error::new(array.len.span(), ERROR_STR)); }
                };
                let string = format!("[u8; {}]", size);
                Ok(Self {
                    field_type,
                    string,
                    size,
                })
            }
            _ => Err(Error::new(field_type.span(), ERROR_STR)),
        }
    }
}

impl ToTokens for FieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.field_type.to_tokens(tokens);
    }
}

impl FieldType {
    /// Return default value for a specified integer type.
    pub fn default_lit(&self) -> TokenStream {
        match self.size {
            FieldSize::Array(size) => quote! { [0_u8; #size] },
            _ => quote! { 0 },
        }
    }

    /// Return minimum value for a specified integer type.
    pub fn minimum_lit(&self) -> TokenStream {
        match self.size {
            FieldSize::Signed8 => quote! { i8::MIN },
            FieldSize::Signed16 => quote! { i16::MIN },
            FieldSize::Signed32 => quote! { i32::MIN },
            FieldSize::Signed64 => quote! { i64::MIN },
            FieldSize::Signed128 => quote! { i128::MIN },
            FieldSize::Unsigned8 => quote! { u8::MIN },
            FieldSize::Unsigned16 => quote! { u16::MIN },
            FieldSize::Unsigned32 => quote! { u32::MIN },
            FieldSize::Unsigned64 => quote! { u64::MIN },
            FieldSize::Unsigned128 => quote! { u128::MIN },
            FieldSize::Array(size) => quote! { [u8::MIN; #size] },
        }
    }

    /// Return maximum value for a specified integer type.
    pub fn maximum_lit(&self) -> TokenStream {
        match self.size {
            FieldSize::Signed8 => quote! { i8::MAX },
            FieldSize::Signed16 => quote! { i16::MAX },
            FieldSize::Signed32 => quote! { i32::MAX },
            FieldSize::Signed64 => quote! { i64::MAX },
            FieldSize::Signed128 => quote! { i128::MAX },
            FieldSize::Unsigned8 => quote! { u8::MAX },
            FieldSize::Unsigned16 => quote! { u16::MAX },
            FieldSize::Unsigned32 => quote! { u32::MAX },
            FieldSize::Unsigned64 => quote! { u64::MAX },
            FieldSize::Unsigned128 => quote! { u128::MAX },
            FieldSize::Array(size) => quote! { [u8::MAX; #size] },
        }
    }

    /// Return the size of the field type in bytes.
    #[inline]
    pub fn size (&self) -> usize {
        match self.size {
            FieldSize::Signed8 => 1,
            FieldSize::Signed16 => 2,
            FieldSize::Signed32 => 4,
            FieldSize::Signed64 => 8,
            FieldSize::Signed128 => 16,
            FieldSize::Unsigned8 => 1,
            FieldSize::Unsigned16 => 2,
            FieldSize::Unsigned32 => 4,
            FieldSize::Unsigned64 => 8,
            FieldSize::Unsigned128 => 16,
            FieldSize::Array(size) => size,
        }
    }
}

impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.string.as_str())
    }
}
