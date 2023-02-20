extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields};

// https://crates.io/crates/convert_case
use convert_case::{Case, Casing};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

#[proc_macro_derive(IsVariant)]
pub fn derive_is_variant(input: TokenStream) -> TokenStream {
    // See https://doc.servo.org/syn/derive/struct.DeriveInput.html
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    // get enum name
    let ref name = input.ident;
    let ref data = input.data;

    let mut variant_checker_functions;

    // data is of type syn::Data
    // See https://doc.servo.org/syn/enum.Data.html
    match data {
        // Only if data is an enum, we do parsing
        Data::Enum(data_enum) => {

            // data_enum is of type syn::DataEnum
            // https://doc.servo.org/syn/struct.DataEnum.html

            variant_checker_functions = TokenStream2::new();

            // Iterate over enum variants
            // `variants` if of type `Punctuated` which implements IntoIterator
            //
            // https://doc.servo.org/syn/punctuated/struct.Punctuated.html
            // https://doc.servo.org/syn/struct.Variant.html
            for variant in &data_enum.variants {

                // Variant's name
                let ref variant_name = variant.ident;

                // Variant can have unnamed fields like `Variant(i32, i64)`
                // Variant can have named fields like `Variant {x: i32, y: i32}`
                // Variant can be named Unit like `Variant`
                let fields_in_variant = match &variant.fields {
                    Fields::Unnamed(_) => quote_spanned! {variant.span()=> (..) },
                    Fields::Unit => quote_spanned! { variant.span()=> },
                    Fields::Named(_) => quote_spanned! {variant.span()=> {..} },
                };

                // construct an identifier named is_<variant_name> for function name
                // We convert it to snake case using `to_case(Case::Snake)`
                // For example, if variant is `HelloWorld`, it will generate `is_hello_world`
                let mut is_variant_func_name =
                    format_ident!("is_{}", variant_name.to_string().to_case(Case::Snake));
                is_variant_func_name.set_span(variant_name.span());

                // Here we construct the function for the current variant
                variant_checker_functions.extend(quote_spanned! {variant.span()=>
                    pub fn #is_variant_func_name(&self) -> bool {
                        match self {
                            #name::#variant_name #fields_in_variant => true,
                            _ => false,
                        }
                    }
                });

                // Above we are making a TokenStream using extend()
                // This is because TokenStream is an Iterator,
                // so we can keep extending it.
                //
                // proc_macro2::TokenStream:- https://docs.rs/proc-macro2/1.0.24/proc_macro2/struct.TokenStream.html

                // Read about
                // quote:- https://docs.rs/quote/1.0.7/quote/
                // quote_spanned:- https://docs.rs/quote/1.0.7/quote/macro.quote_spanned.html
                // spans:- https://docs.rs/syn/1.0.54/syn/spanned/index.html
            }
        }
        _ => return derive_error!("IsVariant is only implemented for enums"),
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            // variant_checker_functions gets replaced by all the functions
            // that were constructed above
            #variant_checker_functions
        }
    };

    TokenStream::from(expanded)
}