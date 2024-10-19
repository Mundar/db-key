use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{quote, ToTokens};
use syn::{
    self,
    Attribute,
    DeriveInput,
    Error,
    Meta,
    Result,
    spanned::Spanned,
    Visibility,
};
use crate::field::{
    DBKeyFields,
    value::FieldValue,
};

#[derive(Debug)]
pub enum DBKey {
    Struct(DBKeyStruct),
    Error(Error),
}

impl DBKey {
    /// Read in the token stream to process the attribute macro.
    pub fn attribute(attr: TokenStream, input: DeriveInput) -> Self {
        match DBKeyStruct::try_attribute(attr, input) {
            Ok(db_key_struct) => Self::Struct(db_key_struct),
            Err(err) => Self::Error(err),
        }
    }

    /// Read in the token stream to process the derive macro.
    pub fn derive(input: DeriveInput) -> Self {
        match DBKeyStruct::try_derive(input) {
            Ok(db_key_struct) => Self::Struct(db_key_struct),
            Err(err) => Self::Error(err),
        }
    }

    /// Generate the source code from the db_key macros.
    pub fn generate(&self) -> TokenStream {
        match self {
            Self::Struct(db_key_struct) => db_key_struct.generate(),
            Self::Error(err) => {
                let error = err.to_compile_error();
                quote! {
                    #error
                }
            }
        }
    }
}

/// Indicates the attribute that we are defining in the state machine that is parsing the attribute
/// parameters.
#[derive(Copy, Clone, Debug)]
enum ParseAttrParam {
    Crate,
    Path,
    RawDebug,
    AltName,
}

/// Indicates the state of what we are expecting when walking through the token stream reading the
/// parameters of the attribute macro.
#[derive(Copy, Clone, Debug)]
enum ParseAttrExpect {
    Param,
    IdentOrLit(ParseAttrParam),
    Equals(ParseAttrParam),
    Comma,
}

/// The output format of the raw array for the Debug trait.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RawDebugFormat {
    /// A compact display of the raw key data that also demarcates the parts of the key.
    Compact,
    /// Display the raw key in Debug that standard way you display any array.
    Standard,
    /// Display the raw key in Debug that standard way you display any array except in lowercase
    /// hexadecimal.
    LowerHex,
    /// Display the raw key in Debug that standard way you display any array except in uppercase
    /// hexadecimal.
    UpperHex,
    /// Display the raw key value in Debug as lowercase hexadecimal values with the leading '0x'.
    PrettyLowerHex,
    /// Display the raw key value in Debug as uppercase hexadecimal values with the leading '0x'.
    PrettyUpperHex,
}

impl Default for RawDebugFormat {
    fn default() -> Self {
        RawDebugFormat::Compact
    }
}

impl TryFrom<&str> for RawDebugFormat {
    type Error = ();

    fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
        match s {
            "compact" => Ok(RawDebugFormat::Compact),
            "lower_hex" => Ok(RawDebugFormat::LowerHex),
            "upper_hex" => Ok(RawDebugFormat::UpperHex),
            "pretty_lower_hex" => Ok(RawDebugFormat::PrettyLowerHex),
            "pretty_upper_hex" => Ok(RawDebugFormat::PrettyUpperHex),
            "std" => Ok(RawDebugFormat::Standard),
            _ => Err(()),
        }
    }
}

/// Stores the status of the parameters read from the attribute macro.
#[derive(Clone, Debug, Default)]
pub struct DBKeyAttributes {
    crate_name: String,
    use_path: String,
    new: bool,
    debug: bool,
    raw_fmt: RawDebugFormat,
    alt_name: Option<Ident>,
}

impl TryFrom<TokenStream> for DBKeyAttributes {
    type Error = Error;

    fn try_from(attr: TokenStream) -> Result<Self> {
        let mut crate_name: Option<String> = None;
        let mut use_path: Option<String> = None;
        let mut alt_name: Option<Ident> = None;
        let mut new = true;
        let mut debug = true;
        let mut raw_fmt = RawDebugFormat::default();
        let mut waiting_for = ParseAttrExpect::Param;
        for thing in attr {
            match thing {
                proc_macro2::TokenTree::Ident(id) => {
                    match waiting_for {
                        ParseAttrExpect::Param => {
                            let span = id.span(); // Save span info
                            let text = id.to_string();
                            match text.as_str() {
                                "alt_name" => {
                                    waiting_for = ParseAttrExpect::Equals(ParseAttrParam::AltName);
                                }
                                "crate_name" => {
                                    waiting_for = ParseAttrExpect::Equals(ParseAttrParam::Crate);
                                }
                                "custom_debug" => {
                                    debug = false;
                                    waiting_for = ParseAttrExpect::Comma;
                                }
                                "raw_debug" => {
                                    waiting_for = ParseAttrExpect::Equals(ParseAttrParam::RawDebug);
                                }
                                "no_new" => {
                                    new = false;
                                    waiting_for = ParseAttrExpect::Comma;
                                }
                                "path" => {
                                    waiting_for = ParseAttrExpect::Equals(ParseAttrParam::Path);
                                }
                                _ => {
                                    return Err(Error::new(span.into(), "Unknown parameter"));
                                }
                            }
                        }
                        ParseAttrExpect::IdentOrLit(ParseAttrParam::Crate) => {
                            crate_name = Some(id.to_string());
                            waiting_for = ParseAttrExpect::Comma;
                        }
                        ParseAttrExpect::IdentOrLit(ParseAttrParam::Path) => {
                            use_path = Some(id.to_string());
                            waiting_for = ParseAttrExpect::Comma;
                        }
                        ParseAttrExpect::IdentOrLit(ParseAttrParam::RawDebug) => {
                            let fmt_str = id.to_string();
                            raw_fmt = RawDebugFormat::try_from(fmt_str.as_str()).map_err(|_| {
                                Error::new(id.span().into(), "Unknown raw_debug format")
                            })?;
                            waiting_for = ParseAttrExpect::Comma;
                        }
                        ParseAttrExpect::IdentOrLit(ParseAttrParam::AltName) => {
                            alt_name = Some(id.clone());
                            waiting_for = ParseAttrExpect::Comma;
                        }
                        _ => {}
                    }
                }
                proc_macro2::TokenTree::Literal(lit) => {
                    match waiting_for {
                        ParseAttrExpect::IdentOrLit(ParseAttrParam::Crate) => {
                            crate_name = Some(lit.to_string().trim_matches('"').to_string());
                            waiting_for = ParseAttrExpect::Comma;
                        }
                        ParseAttrExpect::IdentOrLit(ParseAttrParam::Path) => {
                            use_path = Some(lit.to_string().trim_matches('"').to_string());
                            waiting_for = ParseAttrExpect::Comma;
                        }
                        ParseAttrExpect::IdentOrLit(ParseAttrParam::RawDebug) => {
                            let fmt_str = lit.to_string();
                            raw_fmt = RawDebugFormat::try_from(fmt_str.trim_matches('"'))
                                .map_err(|_| {
                                    Error::new(lit.span().into(), "Unknown raw_debug format")
                            })?;
                            waiting_for = ParseAttrExpect::Comma;
                        }
                        ParseAttrExpect::IdentOrLit(ParseAttrParam::AltName) => {
                            alt_name = Some(Ident::new(lit.to_string().trim_matches('"'),
                                lit.span()));
                            waiting_for = ParseAttrExpect::Comma;
                        }
                        _ => {
                            return Err(Error::new(lit.span().into(),
                                "Unexpected literal encountered"));
                        }
                    }
                }
                proc_macro2::TokenTree::Punct(punct) => {
                    match waiting_for {
                        ParseAttrExpect::Equals(next) => {
                            if '=' == punct.as_char() {
                                waiting_for = ParseAttrExpect::IdentOrLit(next);
                            }
                            else {
                                return Err(Error::new(punct.span().into(),
                                    "Unexpected punctuation (not '=')"));
                            }
                        }
                        ParseAttrExpect::Comma => {
                            if ',' == punct.as_char() {
                                waiting_for = ParseAttrExpect::Param;
                            }
                            else {
                                return Err(Error::new(punct.span().into(),
                                    "Unexpected punctuation (not ',')"));
                            }
                        }
                        _ => {
                            return Err(Error::new(punct.span().into(),
                                "Unexpected punctuation"));
                        }
                    }
                }
                _ => {}
            }
        }
        let crate_name = crate_name.unwrap_or(std::env::var("CARGO_PKG_NAME").unwrap()
                // If the CARGO_PKG_NAME has '-', convert them to '_'.
                .replace('-', "_"));
        let use_path = use_path.unwrap_or_default();
        Ok(DBKeyAttributes {
            crate_name,
            use_path,
            new,
            debug,
            raw_fmt,
            alt_name,
        })
    }
}

impl DBKeyAttributes {
    fn try_derive(attrs: &[Attribute], struct_attrs: &mut Vec<Attribute>) -> Result<Self> {
        let mut result = None;
        for attr in attrs {
            if attr.path().is_ident("doc") {
                struct_attrs.push(attr.clone());
            }
            else if attr.path().is_ident("key") {
                /*
                attr.parse_nested_meta(|meta| {
                    dbg!(&meta.path);
                    dbg!(meta.value());
                    Ok(())
                })?;
                */
                if let Meta::List(list) = &attr.meta {
                    result = Some(DBKeyAttributes::try_from(list.tokens.clone())?);
                }
            }
            else {
                return Err(Error::new(attr.path().get_ident().span(), "Unexpected attribute"));
            }
        }
        if result.is_none() {
            DBKeyAttributes::try_from(TokenStream::default())
        }
        else {
            Ok(result.unwrap())
        }
    }
}

#[derive(Debug)]
pub struct DBKeyStruct {
    pub attr: DBKeyAttributes,
    pub vis: Visibility,
    pub ident: Ident,
    pub args_ident: Ident,
    pub struct_attrs: Vec<Attribute>,
    pub fields: DBKeyFields,
    pub define_args: bool,
}

impl DBKeyStruct {
    pub fn try_attribute(attr: TokenStream, input: DeriveInput) -> Result<Self> {
        let attr = DBKeyAttributes::try_from(attr)?;
        // We are not a drive macro, because we are replacing the input structure with a new
        // definition, but we need the same data as a derive function, so we use the DeriveInput.
        let vis = input.vis.clone();
        let ident = input.ident.clone();
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
        if !impl_generics.into_token_stream().is_empty()
            || !ty_generics.into_token_stream().is_empty()
            || where_clause.is_some()
        {
            return Err(Error::new(ident.span(),
                "db_key attribute macro doesn't support generics."));
        }
        let args_ident = match &attr.alt_name {
            None => Ident::new(&format!("{}Args", ident), ident.span()),
            Some(alt_name) => alt_name.clone(),
        };
        let struct_attrs = input.attrs.clone();
        let fields = DBKeyFields::try_from(&input)?;
        Ok(Self {
            attr,
            vis,
            ident,
            args_ident,
            struct_attrs,
            fields,
            define_args: true,
        })
    }

    pub fn try_derive(input: DeriveInput) -> Result<Self> {
        let mut struct_attrs = Vec::new();
        let attr = DBKeyAttributes::try_derive(&input.attrs, &mut struct_attrs)?;
        let vis = input.vis.clone();
        let args_ident = input.ident.clone();
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
        if !impl_generics.into_token_stream().is_empty()
            || !ty_generics.into_token_stream().is_empty()
            || where_clause.is_some()
        {
            return Err(Error::new(args_ident.span(),
                "db_key attribute macro doesn't support generics."));
        }
        let ident = match &attr.alt_name {
            None => Ident::new(&format!("{}Key", args_ident), args_ident.span()),
            Some(alt_name) => alt_name.clone(),
        };
        let fields = DBKeyFields::try_from(&input)?;
        Ok(Self {
            attr,
            vis,
            ident,
            args_ident,
            struct_attrs,
            fields,
            define_args: false,
        })
    }

    /// Create the use line for documentation.
    pub fn example_start(&self) -> String {
        let (start_line, crate_name) = match self.vis {
            Visibility::Public(_) => {
                ("```rust", self.attr.crate_name.clone())
            }
            _ => {
                ("```text", "crate".to_string())
            }
        };
        if self.attr.use_path.is_empty() {
            format!("{0}\nuse {1}::{{{2}, {3}}};", start_line, crate_name, self.ident,
                self.args_ident)
        }
        else {
            format!("{0}\nuse {1}::{2}::{{{3}, {4}}};", start_line, crate_name,
                self.attr.use_path, self.ident, self.args_ident)
        }
    }

    /// Create a source code documentation string for a single field.
    pub fn doc_init_key(
        &self,
        let_str: &str,
        field: &Ident,
        value: &FieldValue,
    ) -> String {
        format!("let {0} = {1}::from({2} {{\n    {3}: {4},\n    ..Default::default()\n}});",
            let_str, &self.ident, &self.args_ident, field, value)
    }

    pub fn generate(&self) -> TokenStream {
        match self.try_generate() {
            Ok(stream) => stream,
            Err(error) => {
                let error = error.to_compile_error();
                quote! {
                    #error
                }
            }
        }
    }

    fn try_generate(&self) -> Result<TokenStream> {
        let example_start = self.example_start();
        let consts = self.fields.consts();
        let sizes = self.fields.sizes();
        let params = self.fields.params();
        let gets = self.gets();
        let sets = self.sets();
        let defines = self.fields.defines();
        let from_args = self.fields.from_args();
        let debug = self.fields.debug();
        let arg_defaults = self.fields.arg_defaults();
        let defaults = self.fields.defaults();
        let new_init_doc = self.new_init_doc();
        let new_init_partial = self.new_init_partial();
        let verify_new_parts = self.verify_new_parts();
        let verify_new_partial = self.verify_new_partial();
        let from_init_doc = self.from_init_doc();
        let from_init_partial = self.from_init_partial();
        let verify_from_parts = self.verify_from_parts();
        let verify_from_partial = self.verify_from_partial();
        let ident = &self.ident;
        let attrs = &self.struct_attrs;
        let vis = &self.vis;
        let args_ident = &self.args_ident;
        let raw_debug_impl = self.raw_debug_impl();
        let args_doc_header = format!("Argument structure used to create [{}] structures.", ident);
        let from_doc_header = format!("Create a `{}` from a [{}].", ident, args_ident);
        let mut optional_new_docs = Vec::new();
        let mut optional_new_partial_docs = Vec::new();
        let mut optional_functions = Vec::new();
        let from_docs = if self.attr.new {
            let new_doc_header = format!("Create a new `{}` from the individual values.", self.ident);
            optional_functions.push(quote! {
                #[doc = #new_doc_header]
                ///
                /// # Examples
                ///
                #[doc = #example_start]
                ///
                #[doc = #new_init_doc]
                ///
                #[doc = #verify_new_parts]
                ///
                #[doc = #from_init_doc]
                ///
                /// assert_eq!(new_key, from_key);
                /// ```
                pub fn new(#(#params)*) -> Self {
                    let mut buf = [0_u8; #ident::KEY_LENGTH];
                    #(#defines)*
                    Self(buf)
                }
            });
            optional_new_docs.push(quote! {
                ///
                #[doc = #new_init_doc]
                ///
                #[doc = #verify_new_parts]
                ///
                /// assert_eq!(from_key, new_key);
            });
            optional_new_partial_docs.push(quote! {
                ///
                #[doc = #new_init_partial]
                ///
                #[doc = #verify_new_partial]
                ///
                /// assert_eq!(partial_from_key, partial_new_key);
            });
            quote! {
                #[doc = #from_doc_header]
                ///
                /// # Examples
                ///
                #[doc = #example_start]
                ///
                #[doc = #from_init_doc]
                ///
                #[doc = #verify_from_parts]
                ///
                #[doc = #new_init_doc]
                ///
                /// assert_eq!(from_key, new_key);
                ///
                #[doc = #from_init_partial]
                ///
                #[doc = #verify_from_partial]
                ///
                #[doc = #new_init_partial]
                ///
                #[doc = #verify_new_partial]
                ///
                /// assert_eq!(partial_from_key, partial_new_key);
                /// ```
            }
        }
        else {
            quote! {
                #[doc = #from_doc_header]
                ///
                /// # Examples
                ///
                #[doc = #example_start]
                ///
                #[doc = #from_init_doc]
                ///
                #[doc = #verify_from_parts]
                ///
                #[doc = #from_init_partial]
                ///
                #[doc = #verify_from_partial]
                /// ```
            }
        };
        let mut optional_traits = Vec::new();
        if self.attr.debug {
            optional_traits.push(quote! {
                impl ::std::fmt::Debug for #ident {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.debug_struct(stringify!(#ident))
                            #(#debug)*
                            .field("raw", &self.raw_debug())
                            .finish()
                    }
                }
            });
        }
        let args_definition = if self.define_args {
            quote! {
                #[doc = #args_doc_header]
                ///
                /// # Examples
                ///
                #[doc = #example_start]
                ///
                #[doc = #from_init_doc]
                ///
                #[doc = #verify_from_parts]
                #(#optional_new_docs)*
                ///
                #[doc = #from_init_partial]
                ///
                #[doc = #verify_from_partial]
                #(#optional_new_partial_docs)*
                /// ```
                #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
                #vis struct #args_ident {
                    #(pub #params)*
                }
            }
        }
        else {
            quote!{}
        };

        Ok(quote! {
            #args_definition

            impl Default for #args_ident {
                fn default() -> Self {
                    Self {
                        #(#arg_defaults)*
                    }
                }
            }

            #(#attrs)*
            ///
            /// # Examples
            ///
            #[doc = #example_start]
            ///
            #[doc = #from_init_doc]
            ///
            #[doc = #verify_from_parts]
            ///
            #(#optional_new_docs)*
            /// ```
            #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
            #vis struct #ident([u8; #ident::KEY_LENGTH]);

            impl Default for #ident {
                fn default() -> Self {
                    const DEFAULT: [u8; #ident::KEY_LENGTH] = {
                        let mut def_buf = [0_u8; #ident::KEY_LENGTH];
                        let mut def_i = 0;
                        #(#defaults)*
                        def_buf
                    };
                    Self(DEFAULT)
                }
            }

            impl #ident {
                /// The size in bytes of the key data.
                pub const KEY_LENGTH: usize = 0 #(+ #sizes)*;
                /// The sizes of the individual fields in order of definition.
                ///
                /// This is used by the default Debug implementation to split the raw output with
                /// underscores.
                const FIELD_SIZES: &'static [usize] = &[#(#sizes, )*];
                #(#consts)*
                /// The maximum value of the key.
                pub const MAX_KEY: #ident = #ident([0xFF_u8; #ident::KEY_LENGTH]);

                #(#optional_functions)*

                #(#gets)*

                #(#sets)*

            }

            impl AsRef<[u8]> for #ident {
                fn as_ref(&self) -> &[u8] {
                    &self.0[..]
                }
            }

            #from_docs
            impl From<#args_ident> for #ident {
                fn from(args: #args_ident) -> Self {
                    let mut buf = [0_u8; #ident::KEY_LENGTH];
                    #(#from_args)*
                    Self(buf)
                }
            }

            impl From<#ident> for [u8; #ident::KEY_LENGTH] {
                fn from(key: #ident) -> Self {
                    key.0
                }
            }

            impl From<[u8; #ident::KEY_LENGTH]> for #ident {
                fn from(ary: [u8; #ident::KEY_LENGTH]) -> Self {
                    Self(ary)
                }
            }

            impl From<&[u8]> for #ident {
                fn from(slice: &[u8]) -> Self {
                    let size = ::std::cmp::min(#ident::KEY_LENGTH, slice.len());
                    let mut output = #ident::default(); // Fills with zeros
                    output.0[..size].copy_from_slice(&slice[..size]);
                    output
                }
            }

            impl From<#ident> for Vec<u8> {
                fn from(key: #ident) -> Self {
                    key.0.to_vec()
                }
            }

            impl PartialEq<[u8]> for #ident {
                fn eq(&self, other: &[u8]) -> bool {
                    self.as_ref() == other
                }
            }

            impl PartialOrd<[u8]> for #ident {
                fn partial_cmp(&self, other: &[u8]) -> Option<::std::cmp::Ordering> {
                    Some(self.as_ref().cmp(other))
                }
            }

            impl PartialEq<#ident> for [u8] {
                fn eq(&self, other: &#ident) -> bool {
                    other.as_ref() == self
                }
            }

            impl PartialOrd<#ident> for [u8] {
                fn partial_cmp(&self, other: &#ident) -> Option<::std::cmp::Ordering> {
                    Some(self.cmp(other.as_ref()))
                }
            }

            #(#optional_traits)*

            #raw_debug_impl
        })
    }

    fn gets(&self) -> Vec<TokenStream> {
        self.fields.gets(self)
    }

    fn sets(&self) -> Vec<TokenStream> {
        self.fields.sets(self)
    }

    fn new_init_doc(&self) -> String {
        self.fields.new_init_doc(
            format_args!("// Create a new {0} from the individual values.\nlet new_key = {0}::new(\n", self.ident),
            ");")
    }

    fn new_init_partial(&self) -> String {
        self.fields.new_init_partial(
            format_args!("// Create a new {0} from the individual values.\nlet partial_new_key = {0}::new(\n", self.ident),
            ");")
    }

    fn verify_new_parts(&self) -> String {
        self.fields.verify_new_parts(
            format_args!("// Verify the contents of new_key."),
            "")
    }

    fn verify_new_partial(&self) -> String {
        self.fields.verify_new_partial(
            format_args!("// Verify the contents of partial_new_key."),
            "")
    }

    fn from_init_doc(&self) -> String {
        self.fields.from_init_doc(
            format_args!("// Create a {0} from a {1} structure.\nlet from_key = {0}::from( {1} {{\n", self.ident, self.args_ident),
            "});")
    }

    fn from_init_partial(&self) -> String {
        self.fields.from_init_partial(
            format_args!("// Create a {0} from a partially defined {1} structure.\nlet partial_from_key = {0}::from( {1} {{\n", self.ident, self.args_ident),
            "\t..Default::default()\n});")
    }

    fn verify_from_parts(&self) -> String {
        self.fields.verify_from_parts(
            format_args!("// Verify the contents of from_key."),
            "")
    }

    fn verify_from_partial(&self) -> String {
        self.fields.verify_from_partial(
            format_args!("// Verify the contents of partial_from_key."),
            "")
    }

    fn raw_debug_format(&self) -> TokenStream {
        match self.attr.raw_fmt {
            RawDebugFormat::Compact => {
                let ident = &self.ident;
                quote! {
                    f.write_str("0x")?;
                    let mut field_sizes = #ident::FIELD_SIZES.iter();
                    let mut field_remain: usize = *field_sizes.next().unwrap_or(&usize::MAX);
                    // Display a leading underscore before the next byte.
                    let mut leading_ = false;
                    for byte in self.0.0.iter() {
                        if leading_ { f.write_str("_")?; leading_ = false; }
                        f.write_fmt(format_args!("{:02X}", byte))?;
                        field_remain -= 1;
                        if 0 == field_remain {
                            leading_ = true;
                            field_remain = *field_sizes.next().unwrap_or(&usize::MAX);
                        }
                    }
                    Ok(())
                }
            }
            RawDebugFormat::PrettyUpperHex => quote! {
                f.write_str("[")?;
                let mut byte_iter = self.0.0.iter();
                f.write_fmt(format_args!("{:#04X}", byte_iter.next().unwrap()))?;
                for byte in byte_iter {
                    f.write_fmt(format_args!(", {:#04X}", byte))?;
                }
                f.write_str("]")
            },
            RawDebugFormat::PrettyLowerHex => quote! {
                f.write_str("[")?;
                let mut byte_iter = self.0.0.iter();
                f.write_fmt(format_args!("{:#04X}", byte_iter.next().unwrap()))?;
                for byte in byte_iter {
                    f.write_fmt(format_args!(", {:#04x}", byte))?;
                }
                f.write_str("]")
            },
            RawDebugFormat::UpperHex => quote! {
                write!(f, "{:X?}", self.0.0)
            },
            RawDebugFormat::LowerHex => quote! {
                write!(f, "{:x?}", self.0.0)
            },
            _ => quote! {
                write!(f, "{:?}", self.0.0)
            },
        }
    }

    fn raw_debug_impl(&self) -> TokenStream {
        let ident = &self.ident;
        let raw_debug_ident = Ident::new(&format!("{}RawDebug", ident), ident.span());
        let raw_debug_format = self.raw_debug_format();

        quote! {
            impl #ident {
                /// Format the raw key value the same way as the generated default Debug
                /// implementation. This can be used in custom Debug implementations.
                ///
                /// ```rust
                /// use db_key_macro::db_key;
                ///
                /// #[db_key(custom_debug)]
                /// struct LocationInfoKey {
                ///     player_id: u64,
                ///     board_location: u32,
                /// }
                ///
                /// // We want the Debug output to display the board_id, x, and y instead of the
                /// // board location value.
                /// impl std::fmt::Debug for LocationInfoKey {
                ///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                ///         let board_location = self.board_location();
                ///         let board_id = (board_location >> 16) as u16;
                ///         let x = (board_location >> 8) as u8;
                ///         let y = board_location as u8;
                ///         f.debug_struct("LocationInfoKey")
                ///             .field("player_id", &format_args!("{:#018X}", self.player_id()))
                ///             .field("board_id", &board_id)
                ///             .field("x", &x)
                ///             .field("y", &y)
                ///             .field("raw", &self.raw_debug())
                ///             .finish()
                ///     }
                /// }
                ///
                /// let key = LocationInfoKey::new(0x123456789ABCDEF0, 0x0009_05_0A);
                /// assert_eq!(&format!("{:?}", key),
                ///     concat!("LocationInfoKey { player_id: 0x123456789ABCDEF0, board_id: 9, ",
                ///         "x: 5, y: 10, raw: 0x123456789ABCDEF0_0009050A }"));
                /// ```
                #[allow(dead_code)]
                fn raw_debug<'a>(&'a self) -> #raw_debug_ident<'a> {
                    #raw_debug_ident::from(self)
                }
            }

            /// A structure to implement the custom debug formatting for the raw key value.
            struct #raw_debug_ident<'a>(&'a #ident);

            impl<'a> From<&'a #ident> for #raw_debug_ident<'a> {
                fn from(key: &'a #ident) -> Self {
                    Self(key)
                }
            }

            impl<'a> ::std::fmt::Debug for #raw_debug_ident<'a> {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    #raw_debug_format
                }
            }

            impl<'a> ::std::fmt::Display for #raw_debug_ident<'a> {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    ::std::fmt::Debug::fmt(self, f)
                }
            }
        }
    }
}
