//! Procedural macros used by `HiTool` crates.

#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input, parse_quote};

/// Derives `Debug` while replacing fields marked `#[redact]` with
/// `[REDACTED]`.
///
/// This macro supports structs. Unredacted field types must implement
/// `Debug`; redacted field types have no such bound.
///
/// Enums and tuple structs are rejected at compile time:
///
/// ```compile_fail
/// use hitool_macros::RedactedDebug;
///
/// #[derive(RedactedDebug)]
/// enum SecretState {
///     Ready,
/// }
/// ```
#[proc_macro_derive(RedactedDebug, attributes(redact))]
pub fn derive_redacted_debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_redacted_debug(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand_redacted_debug(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = input.ident;
    let mut generics = input.generics;
    let body = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                let mut statements = Vec::with_capacity(fields.named.len());
                for field in fields.named {
                    let identifier = field.ident.expect("named field has an identifier");
                    let field_name = identifier.to_string();
                    let is_redacted = field
                        .attrs
                        .iter()
                        .any(|attribute| attribute.path().is_ident("redact"));
                    if is_redacted {
                        statements.push(quote! {
                            debug.field(#field_name, &"[REDACTED]");
                        });
                    } else {
                        let field_type = field.ty;
                        generics
                            .make_where_clause()
                            .predicates
                            .push(parse_quote!(#field_type: ::core::fmt::Debug));
                        statements.push(quote! {
                            debug.field(#field_name, &self.#identifier);
                        });
                    }
                }
                quote! {
                    let mut debug = formatter.debug_struct(stringify!(#name));
                    #(#statements)*
                    debug.finish()
                }
            }
            Fields::Unit => quote! {
                formatter.write_str(stringify!(#name))
            },
            Fields::Unnamed(_) => {
                return Err(syn::Error::new_spanned(
                    name,
                    "RedactedDebug requires a named-field or unit struct",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "RedactedDebug can only be derived for structs",
            ));
        }
    };

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics ::core::fmt::Debug for #name #type_generics #where_clause {
            fn fmt(
                &self,
                formatter: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                #body
            }
        }
    })
}
