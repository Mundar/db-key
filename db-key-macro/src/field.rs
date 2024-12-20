use proc_macro2::{
    Ident,
    LexError,
    TokenStream,
};
use quote::{quote, quote_spanned};
use syn::{
    self,
    Attribute,
    Data,
    DeriveInput,
    Error,
    Expr,
    Fields,
    Field,
    Lit,
    Meta,
    Result,
    spanned::Spanned,
};
use std::{
    fmt::Write,
};
use crate::{
    field::{
        value::FieldValue,
        r#type::{FieldSize, FieldType},
    },
    parse::DBKeyStruct,
};

pub mod r#type;
pub mod value;

/// Stores the data about the fields in the structure being parsed.
#[derive(Debug, Default)]
pub struct DBKeyFields {
    fields: Vec<DBKeyField>,
}

macro_rules! impl_fields {
    ($(($fn_name: ident, $name: literal),)*) => {
        $(
        #[doc = concat!("Aggregate the ", $name, " data for all fields")]
        pub fn $fn_name(&self) -> Vec<TokenStream> {
            let mut streams = Vec::with_capacity(self.fields.len());
            for field in self.fields.iter() {
                streams.push(field.$fn_name());
            }
            streams
        }
        )*
    };
}

macro_rules! impl_fields_self {
    ($(($fn_name: ident, $name: literal),)*) => {
        $(
        #[doc = concat!("Aggregate the ", $name, " data for all fields")]
        pub fn $fn_name(&self, db_key: &DBKeyStruct) -> Vec<TokenStream> {
            let mut streams = Vec::with_capacity(self.fields.len());
            for field in self.fields.iter() {
                streams.push(field.$fn_name(db_key));
            }
            streams
        }
        )*
    };
}

macro_rules! impl_fields_tests {
    ($(($fn_name:ident, $fn_verify:ident, $init_fn:ident, $verify_fn:ident, $key_name:ident, $docs:literal),)*) => {
        $(
        #[doc = $docs]
        pub fn $fn_name(
            &self,
            head: ::std::fmt::Arguments<'_>,
            tail: &str,
        ) -> String {
            let mut string = String::new();
            let _ = string.write_fmt(head);
            for (i, field) in self.fields.iter().enumerate() {
                field.$init_fn(&mut string, stringify!($key_name), i);
            }
            let _ = string.write_str(tail);
            string
        }

        #[doc = $docs]
        pub fn $fn_verify(
            &self,
            head: ::std::fmt::Arguments<'_>,
            tail: &str,
        ) -> String {
            let mut string = String::new();
            let _ = string.write_fmt(head);
            for (i, field) in self.fields.iter().enumerate() {
                field.$verify_fn(&mut string, stringify!($key_name), i);
            }
            let _ = string.write_str(tail);
            string
        }
        )*
    };
}

impl DBKeyFields {
    impl_fields! {
        (consts, "constants"),
        (sizes, "sizes"),
        (params, "new parameters"),
        (struct_fields, "new structure fields"),
        (defines, "code to initialize new()"),
        (from_args, "code to initialize from(KeyArgs)"),
        (debug, "code to impelment Debug"),
        (defaults, "code to implement Default for the key structure"),
        (minimums, "code to implement MIN_KEY for the key structure"),
        (maximums, "code to implement MAX_KEY for the key structure"),
        (arg_defaults, "code to implement the Default for the arguments structure"),
    }
    impl_fields_self! {
        (gets, "get functions"),
        (sets, "set functions"),
    }
    impl_fields_tests! {
        (new_init_doc, verify_new_parts, new_init_doc, verify_parts, new_key, ""),
        (new_init_partial, verify_new_partial, new_init_partial, verify_partial, partial_new_key, ""),
        (from_init_doc, verify_from_parts, from_init_doc, verify_parts, from_key, ""),
        (from_init_partial, verify_from_partial, from_init_partial, verify_partial, partial_from_key, ""),
    }

    /// Return the total size of the key in bytes.
    pub fn total_size(&self) -> usize {
        let mut total = 0;
        for field in self.fields.iter() {
            total += field.field_type.size();
        }
        total
    }
}

impl TryFrom<&DeriveInput> for DBKeyFields {
    type Error = Error;

    fn try_from(input: &DeriveInput) -> Result<Self> {
        match &input.data {
            Data::Struct(data) => {
                match &data.fields {
                    Fields::Named(named_fields) => {
                        let mut fields = Vec::<DBKeyField>::with_capacity(named_fields.named.len());
                        let mut size = 0;
                        for field in named_fields.named.iter() {
                            fields.push(DBKeyField::try_new(field, &mut size)?);
                        }
                        Ok(Self {
                            fields,
                        })
                    }
                    Fields::Unnamed(_) => {
                        Err(Error::new(input.ident.span(),
                            "Struct with unnamed fields is not supported."))
                    }
                    Fields::Unit => {
                        Err(Error::new(input.ident.span(),
                            "Struct with no fields is not supported."))
                    }
                }
            }
            Data::Enum(_) => {
                Err(Error::new(input.ident.span(),
                    "Enums not supported by the db_key attribute macro."))
            }
            Data::Union(_) => {
                Err(Error::new(input.ident.span(),
                    "Unions not supported by the db_key attribute macro."))
            }
        }
    }
}

#[derive(Debug)]
struct FieldAttributes {
    docs: Vec<Attribute>,
    name: String,
    default: TokenStream,
    minimum: TokenStream,
    maximum: TokenStream,
}

impl FieldAttributes {
    /// Parse the attribute values for a field. It returns a tuple with a vector of the doc
    /// attributes and the proper name for the field.
    pub fn try_new(field: &Field, field_type: &FieldType) -> Result<Self> {
        let mut docs = Vec::new();
        let mut name = field.ident.clone().unwrap().to_string();
        let mut default = field_type.default_lit();
        let mut minimum = field_type.minimum_lit();
        let mut maximum = field_type.maximum_lit();
        for attr in field.attrs.iter() {
            if let Some(ident) = attr.path().get_ident() {
                let s = ident.to_string();
                match s.as_str() {
                    "doc" => {
                        docs.push(attr.clone());
                    }
                    "name" => {
                        match &attr.meta {
                            Meta::NameValue(name_value) => {
                                match &name_value.value {
                                    Expr::Lit(expr_lit) => {
                                        match &expr_lit.lit {
                                            Lit::Str(lit_str) => {
                                                name = lit_str.value();
                                            }
                                            lit => {
                                                return Err(Error::new(lit.span(),
                                                    "The name attribute expects a literal string."));
                                            }
                                        }
                                    }
                                    _ => {
                                        return Err(Error::new(ident.span(),
                                            "The name attribute expects a literal value."));
                                    }
                                }
                            }
                            _ => {
                                return Err(Error::new(ident.span(),
                                    "The name attribute expects a value."));
                            }
                        }
                    }
                    "default" => {
                        match &attr.meta {
                            Meta::NameValue(name_value) => {
                                default = Self::parse_default_value(&name_value.value)?;
                            }
                            _ => {
                                return Err(Error::new(ident.span(),
                                    "The default attribute expects a value."));
                            }
                        }
                    }
                    "min" => {
                        match &attr.meta {
                            Meta::NameValue(name_value) => {
                                minimum = Self::parse_default_value(&name_value.value)?;
                            }
                            _ => {
                                return Err(Error::new(ident.span(),
                                    "The min attribute expects a value."));
                            }
                        }
                    }
                    "max" => {
                        match &attr.meta {
                            Meta::NameValue(name_value) => {
                                maximum = Self::parse_default_value(&name_value.value)?;
                            }
                            _ => {
                                return Err(Error::new(ident.span(),
                                    "The max attribute expects a value."));
                            }
                        }
                    }
                    unknown => {
                        println!("Unexpected attribute: {}", unknown);
                    }
                }
            }
        }
        Ok(Self {
            docs,
            name,
            default,
            minimum,
            maximum,
        })
    }

    fn parse_default_value(value: &Expr) -> Result<TokenStream> {
        match value {
            Expr::Lit(lit) => {
                match &lit.lit {
                    Lit::Str(s) => {
                        let string = s.value();
                        let trimmed = string.trim_matches('"');
                        let stream: std::result::Result<TokenStream, LexError> = trimmed.parse();
                        match stream {
                            Ok(stream) => {
                                Ok(quote_spanned! {
                                    value.span()=>
                                    #stream
                                })
                            }
                            Err(err) => {
                                Err(Error::new(value.span(),
                                    format!("Failed to parse parameter: {}", err)))
                            }
                        }
                    }
                    _ => Ok(quote_spanned! {
                        value.span()=>
                            #value
                    }),
                }
            }
            _ => Ok(quote_spanned! {
                value.span()=>
                    #value
            }),
        }
    }
}

macro_rules! impl_const_define {
    ($fn_ident:ident, $attr_ident:ident, $const_name:literal) => {
        #[doc = concat!("Define the code to initialize ", $const_name, " for this field.")]
        pub fn $fn_ident(&self) -> TokenStream {
            let field_type = &self.field_type;
            let xor_mask = &self.field_type.minimum_lit();
            let value = &self.attr.$attr_ident;
            match self.field_type.size {
                FieldSize::Signed8 | FieldSize::Signed16 | FieldSize::Signed32 |
                    FieldSize::Signed64 | FieldSize::Signed128 =>
                {
                    let size = field_type.size();
                    quote! {
                        let value: #field_type = #value ^ #xor_mask;
                        if 0 == value {
                            buf_i += #size;
                        }
                        else {
                            let bytes = value.to_be_bytes();
                            let mut i = 0;
                            while i < bytes.len() {
                                buf[buf_i] = bytes[i];
                                buf_i += 1;
                                i += 1;
                            }
                        }
                    }
                }
                FieldSize::Unsigned8 | FieldSize::Unsigned16 | FieldSize::Unsigned32 |
                    FieldSize::Unsigned64 | FieldSize::Unsigned128 =>
                {
                    let size = field_type.size();
                    quote! {
                        let value: #field_type = #value;
                        if 0 == value {
                            buf_i += #size;
                        }
                        else {
                            let bytes = value.to_be_bytes();
                            let mut i = 0;
                            while i < bytes.len() {
                                buf[buf_i] = bytes[i];
                                buf_i += 1;
                                i += 1;
                            }
                        }
                    }
                }
                FieldSize::Array(_) => {
                    quote! {
                        let value: #field_type = #value;
                        let mut i = 0;
                        while i < value.len() {
                            buf[buf_i] = value[i];
                            buf_i += 1;
                            i += 1;
                        }
                    }
                }
            }
        }
    };
}

#[derive(Debug)]
pub struct DBKeyField {
    attr: FieldAttributes,
    ident: Ident,
    set_ident: Ident,
    size_ident: Ident,
    start_ident: Ident,
    end_ident: Ident,
    range_ident: Ident,
    default_ident: Ident,
    min_ident: Ident,
    max_ident: Ident,
    field_type: FieldType,
    random: FieldValue,
    start_index: usize,
}

impl DBKeyField {
    pub fn try_new(field: &Field, start_byte: &mut usize) -> Result<Self> {
        let ident = field.ident.clone().unwrap();
        let upper_str = ident.to_string().to_uppercase();
        let set_ident = Ident::new(&format!("set_{}", ident), ident.span());
        let size_ident = Ident::new(&format!("{}_SIZE", upper_str), ident.span());
        let start_ident = Ident::new(&format!("{}_START", upper_str), ident.span());
        let end_ident = Ident::new(&format!("{}_END", upper_str), ident.span());
        let range_ident = Ident::new(&format!("{}_RANGE", upper_str), ident.span());
        let default_ident = Ident::new(&format!("{}_DEFAULT", upper_str), ident.span());
        let min_ident = Ident::new(&format!("{}_MIN", upper_str), ident.span());
        let max_ident = Ident::new(&format!("{}_MAX", upper_str), ident.span());
        let field_type = FieldType::try_from(field)?;
        let attr = FieldAttributes::try_new(field, &field_type)?;
        let random = FieldValue::random(field_type.size);
        let start_index = *start_byte;
        *start_byte = start_index + field_type.size();
        Ok(Self {
            attr,
            ident,
            set_ident,
            size_ident,
            start_ident,
            end_ident,
            range_ident,
            default_ident,
            min_ident,
            max_ident,
            field_type,
            random,
            start_index,
        })
    }

    /// Define the constants for this field.
    pub fn consts(&self) -> TokenStream {
        let name = &self.attr.name;
        let size_ident = &self.size_ident;
        let start_ident = &self.start_ident;
        let end_ident = &self.end_ident;
        let range_ident = &self.range_ident;
        let default_ident = &self.default_ident;
        let min_ident = &self.min_ident;
        let max_ident = &self.max_ident;
        let size = &self.field_type.size();
        let start = &self.start_index;
        let default = &self.attr.default;
        let field_type = &self.field_type;
        let min = &self.attr.minimum;
        let max = &self.attr.maximum;
        quote! {
            #[doc = concat!("The size of the ", #name, " field.")]
            pub(crate) const #size_ident: usize = #size;
            #[doc = concat!("The starting byte of the ", #name, " field in the key array.")]
            pub(crate) const #start_ident: usize = #start;
            #[doc = concat!("The byte after the last byte of the ", #name, " field in the key array.")]
            pub(crate) const #end_ident: usize = Self::#start_ident + Self::#size_ident;
            #[doc = concat!("The range of the bytes for the ", #name, " field in the key array.")]
            pub(crate) const #range_ident: ::std::ops::Range<usize> = Self::#start_ident..Self::#end_ident;
            #[doc = concat!("The default value of the ", #name, " field in the key array.")]
            pub(crate) const #default_ident: #field_type = #default;
            #[doc = concat!("The minimum value of the ", #name, " field in the key array.")]
            pub(crate) const #min_ident: #field_type = #min;
            #[doc = concat!("The maximum value of the ", #name, " field in the key array.")]
            pub(crate) const #max_ident: #field_type = #max;
        }
    }

    /// Define the sizes for this field.
    pub fn sizes(&self) -> TokenStream {
        let size_ident = &self.size_ident;
        quote!{
            Self::#size_ident
        }
    }

    /// Define the new parameters for this field.
    pub fn params(&self) -> TokenStream {
        let ident = &self.ident;
        let field_type = &self.field_type;
        quote!{
            #ident: #field_type,
        }
    }

    /// Define the new structure field for this field.
    pub fn struct_fields(&self) -> TokenStream {
        let ident = &self.ident;
        let docs = &self.attr.docs;
        let field_type = &self.field_type;
        quote!{
            #(#docs)*
            pub #ident: #field_type,
        }
    }

    /// Define the field type as an array.
    fn as_array(&self) -> TokenStream {
        let ident = &self.ident;
        match self.field_type.size {
            FieldSize::Array(_) => quote! { #ident },
            FieldSize::Unsigned8 | FieldSize::Unsigned16 | FieldSize::Unsigned32 |
                FieldSize::Unsigned64 | FieldSize::Unsigned128
                => quote! { #ident.to_be_bytes() },
            FieldSize::Signed8 | FieldSize::Signed16 | FieldSize::Signed32 |
                FieldSize::Signed64 | FieldSize::Signed128
                => {
                    let xor_mask = self.field_type.minimum_lit();
                    quote! { (#ident ^ #xor_mask).to_be_bytes() }
                }
        }
    }

    /// Define the code to initialize new() for this field.
    pub fn defines(&self) -> TokenStream {
        let range_ident = &self.range_ident;
        let as_array = &self.as_array();
        quote!{
            buf[Self::#range_ident].copy_from_slice(&#as_array);
        }
    }

    /// Define the code to initialize from(KeyArgs) for this field.
    pub fn from_args(&self) -> TokenStream {
        let range_ident = &self.range_ident;
        match self.field_type.size {
            FieldSize::Signed8 | FieldSize::Signed16 | FieldSize::Signed32 |
                FieldSize::Signed64 | FieldSize::Signed128
            => {
                let ident = &self.ident;
                let xor_mask = self.field_type.minimum_lit();
                quote! {
                    buf[Self::#range_ident]
                        .copy_from_slice(&(args.#ident ^ #xor_mask).to_be_bytes());
                }
            }
            _ => {
                let as_array = &self.as_array();
                quote!{
                    buf[Self::#range_ident].copy_from_slice(&args.#as_array);
                }
            }
        }
    }

    /// Define the code to initialize from(KeyArgs) for this field.
    pub fn debug(&self) -> TokenStream {
        let ident = &self.ident;
        quote!{
            .field(stringify!(#ident), &self.#ident())
        }
    }

    impl_const_define!{defaults, default, "Default"}
    impl_const_define!{maximums, maximum, "maximum value"}
    impl_const_define!{minimums, minimum, "minimum value"}
    /// Define the code to initialize from(KeyArgs) for this field.
    pub fn arg_defaults(&self) -> TokenStream {
        let ident = &self.ident;
        let default = &self.attr.default;
        quote! {
            #ident: #default,
        }
    }

    /// Define the code to extract the value for this field.
    fn get_code(&self) -> TokenStream {
        let ident = &self.ident;
        match self.field_type.size {
            FieldSize::Signed8 => {
                let start_ident = &self.start_ident;
                quote! {
                    pub const fn #ident(&self) -> i8 {
                        (self.0[Self::#start_ident] as i8) ^ i8::MIN
                    }
                }
            }
            FieldSize::Unsigned8 => {
                let start_ident = &self.start_ident;
                quote! {
                    pub const fn #ident(&self) -> u8 {
                        self.0[Self::#start_ident]
                    }
                }
            }
            FieldSize::Signed16 | FieldSize::Signed32 | FieldSize::Signed64 |
                FieldSize::Signed128 =>
            {
                let size_ident = &self.size_ident;
                let range_ident = &self.range_ident;
                let field_type = &self.field_type;
                let xor_mask = &self.field_type.minimum_lit();
                quote! {
                    pub fn #ident(&self) -> #field_type {
                        let mut buf = [0_u8; Self::#size_ident];
                        buf.copy_from_slice(&self.0[Self::#range_ident]);
                        #field_type::from_be_bytes(buf) ^ #xor_mask
                    }
                }
            }
            FieldSize::Unsigned16 | FieldSize::Unsigned32 | FieldSize::Unsigned64 |
                FieldSize::Unsigned128 =>
            {
                let size_ident = &self.size_ident;
                let range_ident = &self.range_ident;
                let field_type = &self.field_type;
                quote! {
                    pub fn #ident(&self) -> #field_type {
                        let mut buf = [0_u8; Self::#size_ident];
                        buf.copy_from_slice(&self.0[Self::#range_ident]);
                        #field_type::from_be_bytes(buf)
                    }
                }
            }
            FieldSize::Array(_size) => {
                let range_ident = &self.range_ident;
                quote! {
                    pub fn #ident(&self) -> &[u8] {
                        &self.0[Self::#range_ident]
                    }
                }
            }
        }
    }

    /// Define the code to extract the value for this field.
    pub fn gets(&self, db_key: &DBKeyStruct) -> TokenStream {
        let example_start = db_key.example_start();
        let struct_ident = &db_key.ident;
        let get_doc = format!("Get the {} value from the `{}`.", &self.attr.name, struct_ident);
        let random = FieldValue::random(self.field_type.size);
        let min_lines = if db_key.attr.min_key {
            [ format!("\nlet min_key = {0}::MIN_KEY;", struct_ident),
            format!("\nassert_eq!(min_key.{0}(), {1});", &self.ident, &self.attr.minimum) ]
        }
        else {
            [String::new(), String::new()]
        };
        let max_lines = if db_key.attr.max_key {
            [ format!("\nlet max_key = {0}::MAX_KEY;", struct_ident),
            format!("\nassert_eq!(max_key.{0}(), {1});", &self.ident, &self.attr.maximum) ]
        }
        else {
            [String::new(), String::new()]
        };
        let get_example = format!(r#"
let default_key = {0}::default();{5}{6}
{1}

assert_eq!(default_key.{2}(), {3});{7}{8}
assert_eq!(key.{2}(), {4});"#,
            struct_ident, // 0
            db_key.doc_init_key("key", &self.ident, &random),   // 1
            &self.ident, // 2
            &self.attr.default, // 3
            random.assert_eq(), // 4
            min_lines[0], // 5
            max_lines[0], // 6
            min_lines[1], // 7
            max_lines[1], // 8
        );
        let docs = &self.attr.docs;
        let get_code = self.get_code();
        quote! {
            #[doc = #get_doc]
            ///
            #(#docs)*
            ///
            /// # Examples
            ///
            #[doc = #example_start]
            #[doc = #get_example]
            /// ```
            #get_code
        }
    }

    /// Define the code to extract the value for this field.
    fn set_code(&self) -> TokenStream {
        let set_ident = &self.set_ident;
        match self.field_type.size {
            FieldSize::Signed8 => {
                let start_ident = &self.start_ident;
                quote! {
                    pub fn #set_ident(&mut self, value: i8) {
                        self.0[Self::#start_ident] = (value ^ i8::MIN) as u8;
                    }
                }
            }
            FieldSize::Unsigned8 => {
                let start_ident = &self.start_ident;
                quote! {
                    pub fn #set_ident(&mut self, value: u8) {
                        self.0[Self::#start_ident] = value;
                    }
                }
            }
            FieldSize::Signed16 | FieldSize::Signed32 | FieldSize::Signed64 |
                FieldSize::Signed128 =>
            {
                let range_ident = &self.range_ident;
                let field_type = &self.field_type;
                let xor_mask = self.field_type.minimum_lit();
                quote! {
                    pub fn #set_ident(&mut self, value: #field_type) {
                        self.0[Self::#range_ident].copy_from_slice(&(value ^ #xor_mask).to_be_bytes());
                    }
                }
            }
            FieldSize::Unsigned16 | FieldSize::Unsigned32 | FieldSize::Unsigned64 |
                FieldSize::Unsigned128 =>
            {
                let range_ident = &self.range_ident;
                let field_type = &self.field_type;
                quote! {
                    pub fn #set_ident(&mut self, value: #field_type) {
                        self.0[Self::#range_ident].copy_from_slice(&value.to_be_bytes());
                    }
                }
            }
            FieldSize::Array(_) => {
                let range_ident = &self.range_ident;
                quote! {
                    pub fn #set_ident<V: std::convert::AsRef<[u8]>>(&mut self, value: V) {
                        self.0[Self::#range_ident].copy_from_slice(value.as_ref());
                    }
                }
            }
        }
    }

    /// Define the code to insert the value for this field into the key array.
    pub fn sets(&self, db_key: &DBKeyStruct) -> TokenStream {
        let example_start = db_key.example_start();
        let struct_ident = &db_key.ident;
        let set_doc = format!("Set the {} in the `{}`.", &self.attr.name, struct_ident);
        let random1 = FieldValue::random(self.field_type.size);
        let random2 = {
            let mut random = FieldValue::random(self.field_type.size);
            while random == random1 {
                random = FieldValue::random(self.field_type.size);
            }
            random
        };
        let set_example = format!(r#"
{0}

assert_eq!(key.{1}(), {2});
key.{3}({4});
assert_eq!(key.{1}(), {4});"#,
            db_key.doc_init_key("mut key", &self.ident, &random1),  // 0
            &self.ident,    // 1
            random1,    // 2
            &self.set_ident,    // 3
            random2,    // 4
        );
        let docs = &self.attr.docs;
        let set_code = self.set_code();
        quote! {
            #[doc = #set_doc]
            ///
            #(#docs)*
            ///
            /// # Examples
            ///
            #[doc = #example_start]
            #[doc = #set_example]
            /// ```
            #set_code
        }
    }

    /// Define the doctest for the new() function for this field.
    pub fn new_init_doc(&self, output: &mut String, _key_name: &str, _index: usize) {
        let _ = output.write_fmt(format_args!("\t{},\n", self.random));
    }

    /// Define the doctest for the new() function for this field.
    pub fn from_init_doc(&self, output: &mut String, _key_name: &str, _index: usize) {
        let _ = output.write_fmt(format_args!("\t{}: {},\n", self.ident, self.random));
    }

    /// Define the doctest for the partial new() function for this field.
    pub fn new_init_partial(&self, output: &mut String, _key_name: &str, index: usize) {
        if 0 == (1 & index) {
            let _ = output.write_fmt(format_args!("\t{},\n", self.random));
        }
        else {
            let _ = output.write_fmt(format_args!("\t{},\n", self.attr.default));
        }
    }

    /// Define the doctest for the partial new() function for this field.
    pub fn from_init_partial(&self, output: &mut String, _key_name: &str, index: usize) {
        if 0 == (1 & index) {
            let _ = output.write_fmt(format_args!("\t{}: {},\n", self.ident, self.random));
        }
    }

    /// Define the doctest for the new() function for this field.
    pub fn verify_parts(&self, output: &mut String, key_name: &str, _index: usize) {
        let _ = output.write_fmt(format_args!("\nassert_eq!({}.{}(), {});", key_name, &self.ident,
            &self.random));
    }

    /// Define the doctest for the new() function for this field.
    pub fn verify_partial(&self, output: &mut String, key_name: &str, index: usize) {
        if 0 == (1 & index) {
            let _ = output.write_fmt(format_args!("\nassert_eq!({}.{}(), {});", key_name,
                &self.ident, &self.random));
        }
        else {
            let _ = output.write_fmt(format_args!("\nassert_eq!({}.{}(), {});", key_name,
                &self.ident, &self.attr.default));
        }
    }
}
