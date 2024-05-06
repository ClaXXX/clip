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

/// Implements parsing for all fields and supports either it's named or not
///
/// if a #[try_parse] attribute is associated with the field, it will uses the TryParse::try_parse
/// method for the field, otherwise and by default, str.parse::<ty> method is used.
/// 
/// Since there no way to know if a certain trait has been implemented (TryStr or TryParse mainly),
/// the generated error is hard to read. However, it is the only source of error of this macro.
/// Thus, if an error happens from the lib, it means FromStr trait has not been implemented or
/// try_parse attribute has been forgotten or not but TryParse trait is not implemented.
fn impl_fields_parsing(fields: syn::punctuated::Iter<'_, syn::Field>) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut gen = proc_macro2::TokenStream::new();
    for syn::Field { ty, ident, attrs, .. } in fields {
        if let Some(name) = ident {
            gen.extend(quote! { #name: });
        }
        if let Some(&_) = attrs.iter().find(|attr| (*attr).path().get_ident().is_some_and(|i| i.to_string() == "try_parse")) {
            gen.extend(quote! {
                {
                    let Parsed ( value, rest ) = #ty::try_parse(values)?;
                    values = rest;
                    value
                },
            });
        } else {
            gen.extend(quote! { values.next().map_or(Err(ParsingError::TooFewArguments), |value| value.parse::<#ty>().or(Err(ParsingError::BadType)))?, });
        }
    }
    Ok(gen)
}

/// Implements the initialisation of an object (Tuple/Struct/Unit).
/// 
/// Depending of the fields type, chooses the correct initialisation pattern
fn impl_object_initialisation(ident: &syn::Ident, fields: &syn::Fields) -> Result<proc_macro2::TokenStream, syn::Error> {
    match fields {
        syn::Fields::Unit => Ok(quote!{ #ident }),
        syn::Fields::Named(syn::FieldsNamed { named, .. } ) => {
            let fields = impl_fields_parsing(named.iter())?;
            Ok(quote!{ #ident { #fields } })
        },
        syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
            let fields = impl_fields_parsing(unnamed.iter())?;
            Ok(quote! { #ident ( #fields ) })
        }
    }
}

/// Implements the initialisation of an enum 
///
/// Consumes the next iterator value and tries to match to one of the enumeration variants
/// It is case insensitive.
fn impl_enum_initialization(parent: &syn::Ident, variants: syn::punctuated::Iter<'_, syn::Variant>) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut gen = proc_macro2::TokenStream::new();
    for syn::Variant { ident, fields, .. } in variants {
        let lowercase = ident.to_string().to_lowercase();
        let value = impl_object_initialisation(ident, fields)?;
        gen.extend(quote! {
            #lowercase => Ok(#parent::#value),
        });
    }
    Ok(quote! {
        {
            let keyword = values.next().ok_or(ParsingError::TooFewArguments)?;
            match keyword.to_lowercase().as_str() {
                #gen
                _ => Err(ParsingError::VariantNotFound)
            }
        }?
    })
}

/// Implements TryParse trait for any rust object with the input values being an iterator of &str
/// 
/// Supports Struct and Enum but not Union
pub(crate) fn impl_try_parse_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let parser = match &ast.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => impl_object_initialisation(&ast.ident, fields),
        syn::Data::Enum(syn::DataEnum { variants, .. }) => impl_enum_initialization(&ast.ident, variants.iter()),
        syn::Data::Union(syn::DataUnion { union_token, .. }) => Err(syn::Error::new_spanned(union_token, "TryParse macro doesn't support union")),
    }.unwrap_or_else(|err| err.to_compile_error());
    quote! {
        impl<'a, I> TryParse<I> for #name where
                I: std::iter::Iterator<Item = &'a &'a str> {
            type Error = ParsingError;

            fn try_parse(mut values: I) -> Result<Parsed<Self, I>, Self::Error> {
                Ok(Parsed((#parser), values))
            }
        }
    }.into()
}

