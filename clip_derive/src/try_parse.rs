// Copyright © 2024 Claire Bts
//
// This file is part of CLIP
//
// CLIP is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// CLIP is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::attribute;
use proc_macro::TokenStream;
use quote::quote;

struct ParsingMacro {
    recursion_attr: &'static str,
}

impl ParsingMacro {
    /// Implements parsing for all fields and supports either it's named or not
    ///
    /// if a #[try_parse] attribute is associated with the field, it will uses the TryParse::try_parse
    /// method for the field, otherwise and by default, str.parse::<ty> method is used.
    ///
    /// Since there no way to know if a certain trait has been implemented (TryStr or TryParse mainly),
    /// the generated error is hard to read. However, it is the only source of error of this macro.
    /// Thus, if an error happens from the lib, it means FromStr trait has not been implemented or
    /// try_parse attribute has been forgotten or not but TryParse trait is not implemented.
    fn impl_fields(
        &self,
        fields: syn::punctuated::Iter<'_, syn::Field>,
    ) -> Result<proc_macro2::TokenStream, syn::Error> {
        let mut gen = proc_macro2::TokenStream::new();
        for syn::Field {
            ty, ident, attrs, ..
        } in fields
        {
            if let Some(name) = ident {
                gen.extend(quote! { #name: });
            }
            if let Some(&_) = attrs.iter().find(attribute::is(self.recursion_attr)) {
                gen.extend(quote! {
                    {
                        let clipv::parser::Parsed ( value, rest ) = #ty::try_parse(values)?;
                        values = rest;
                        value
                    },
                });
            } else {
                gen.extend(quote! { values.next().map_or(Err(clipv::parser::ParsingError::TooFewArguments), |value| value.parse::<#ty>().or(Err(clipv::parser::ParsingError::BadType)))?, });
            }
        }
        Ok(gen)
    }

    /// Implements the initialisation of an object (Tuple/Struct/Unit).
    ///
    /// Depending of the fields type, chooses the correct initialisation pattern
    fn impl_object_initialisation(
        &self,
        ident: &syn::Ident,
        fields: &syn::Fields,
    ) -> Result<proc_macro2::TokenStream, syn::Error> {
        match fields {
            syn::Fields::Unit => Ok(quote! { #ident }),
            syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
                let fields = self.impl_fields(named.iter())?;
                Ok(quote! { #ident { #fields } })
            }
            syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
                let fields = self.impl_fields(unnamed.iter())?;
                Ok(quote! { #ident ( #fields ) })
            }
        }
    }

    /// Implements the initialisation of an enum
    ///
    /// Consumes the next iterator value and tries to match to one of the enumeration variants
    /// It is case insensitive.
    fn impl_enum_initialization(
        &self,
        parent: &syn::Ident,
        variants: syn::punctuated::Iter<'_, syn::Variant>,
    ) -> Result<proc_macro2::TokenStream, syn::Error> {
        let mut gen = proc_macro2::TokenStream::new();
        for syn::Variant { ident, fields, .. } in variants {
            let lowercase = ident.to_string().to_lowercase();
            let value = self.impl_object_initialisation(ident, fields)?;
            gen.extend(quote! {
                #lowercase => Ok(#parent::#value),
            });
        }
        Ok(quote! {
            {
                let keyword = values.next().ok_or(clipv::parser::ParsingError::TooFewArguments)?;
                match keyword.to_lowercase().as_str() {
                    #gen
                    _ => Err(clipv::parser::ParsingError::VariantNotFound)
                }
            }?
        })
    }

    fn impl_parser(&self, ident: &syn::Ident, data: &syn::Data) -> proc_macro2::TokenStream {
        match data {
            syn::Data::Struct(syn::DataStruct { fields, .. }) => {
                self.impl_object_initialisation(ident, fields)
            }
            syn::Data::Enum(syn::DataEnum { variants, .. }) => {
                self.impl_enum_initialization(ident, variants.iter())
            }
            syn::Data::Union(syn::DataUnion { union_token, .. }) => Err(syn::Error::new_spanned(
                union_token,
                "Unsupported Union type",
            )),
        }
        .unwrap_or_else(|err| err.to_compile_error())
    }
}

/// Implements TryParse trait for any rust object with the input values being an iterator of &str
///
/// Supports Struct and Enum but not Union
pub(crate) fn impl_try_parse_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let parser = ParsingMacro {
        recursion_attr: "try_parse",
    }
    .impl_parser(name, &ast.data);
    quote! {
        impl<'a> clipv::parser::TryParse<&'a str> for #name {
            type Error = clipv::parser::ParsingError;

            fn try_parse<I: std::iter::Iterator<Item = &'a str>>(mut values: I) -> Result<clipv::parser::Parsed<Self, I>, Self::Error> {
                Ok(clipv::parser::Parsed((#parser), values))
            }
        }

        impl<'a> clipv::parser::TryParse<&'a &'a str> for #name {
            type Error = clipv::parser::ParsingError;

            fn try_parse<I: std::iter::Iterator<Item = &'a &'a str>>(mut values: I) -> Result<clipv::parser::Parsed<Self, I>, Self::Error> {
                Ok(clipv::parser::Parsed((#parser), values))
            }
        }
    }
    .into()
}
