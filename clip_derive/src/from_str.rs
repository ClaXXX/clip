// Copyright © 2024 Claire Bts
//
// This file is part of CLIP
//
// CLIP is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// CLIP is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

use proc_macro::TokenStream;
use quote::quote;

fn impl_from_str_enum_fields(
    parent: &syn::Ident,
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut fields_gen = proc_macro2::TokenStream::new();

    for syn::Variant { ident, fields, .. } in variants.iter() {
        if let syn::Fields::Unit = fields {
            let lowercase_ident = ident.to_string().to_lowercase();

            fields_gen.extend(quote! {
                #lowercase_ident => Ok(#parent::#ident),
            });
        } else {
            return Err(syn::Error::new_spanned(
                fields,
                "TryFromStr only supports unit fields",
            ));
        }
    }
    Ok(fields_gen)
}

fn impl_from_str_trait_for_enum(
    name: &syn::Ident,
    fields: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote! {
        impl std::str::FromStr for #name {
            type Err = String;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value.to_lowercase().as_str() {
                    #fields
                    _ => Err(format!("Unexistant variant {}", value))
                }
            }
        }
    }
}

pub(crate) fn impl_from_str_macro(ast: &syn::DeriveInput) -> TokenStream {
    if let syn::Data::Enum(syn::DataEnum { variants, .. }) = &ast.data {
        match impl_from_str_enum_fields(&ast.ident, variants) {
            Ok(fields) => impl_from_str_trait_for_enum(&ast.ident, fields),
            Err(err) => err.to_compile_error(),
        }
    } else {
        syn::Error::new_spanned(ast, "expected an enum").to_compile_error()
    }
    .into()
}
