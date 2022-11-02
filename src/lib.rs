//! Creates unique identifiers for macros using procedural macros and [UUID](https://crates.io/crates/uuid)
//! # Examples
//! ```
//!
//! macro_rules! gen_fn {
//!     ($a:ty, $b:ty) => {
//!         detsym::detsym!{ _gen_fn!{ $a, $b } }
//!     };
//! }
//!
//! macro_rules! _gen_fn {
//!     ($detsym:ident, $a:ty, $b:ty) => {
//!         fn $detsym(a: $a, b: $b) {
//!             unimplemented!()
//!         }
//!     };
//! }
//!
//! mod test {
//!     gen_fn!{ u64, u64 }
//!     gen_fn!{ u32, u32 }
//! }
//! ```
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::quote;
use syn::{parse_macro_input, parse_quote};
use uuid::Uuid;

#[proc_macro]
pub fn detsym(input: TokenStream) -> TokenStream {
    //! Generate a unique identifier with a span of `Span::call_site` and
    //! insert it as the first argument to a macro call followed by a comma.

    let mcall = parse_macro_input!(input as syn::Macro);

    proc_macro::TokenStream::from(
        alter_macro(mcall).unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    )
}


fn alter_macro(mut mcall: syn::Macro) -> Result<proc_macro2::TokenStream, syn::Error> {
    use core::iter::Extend;
    use quote::ToTokens;
    let namespace_a = Uuid::parse_str("000000001111111111d2d3d4d5d6d7d8").unwrap();
    let path = Span::call_site().source_file().path();
    let path_bytes = path.as_path().to_str().unwrap().as_bytes();
    let namespace = Uuid::new_v5(&namespace_a, path_bytes);
    let seed: String = format!("{}", mcall.tts);


    let sym = syn::Ident::new(
        &format!("detsym_{}", Uuid::new_v5(&namespace, seed.as_bytes()).to_simple()).to_uppercase(),
        Span::call_site(),
    );

    let mut inserted_detsym: proc_macro2::TokenStream = parse_quote!(#sym, );

    inserted_detsym.extend(mcall.tts);
    mcall.tts = inserted_detsym;

    Ok(mcall.into_token_stream())
}
