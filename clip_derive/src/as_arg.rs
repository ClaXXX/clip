//SPDX-FileCopyrightText: 2024 Claire Bts <claxxx.bts@gmail.com>
//SPDX-License-Identifier: GPL-3.0-or-later

// clip_derive aims to simplify writing cli and/or parser in general

//Copyright (C) 2024 Claire Bts claxxx.bts@gmail.com

//This program is free software: you can redistribute it and/or modify it under the terms of the
//GNU General Public License as published by the Free Software Foundation, either version 3 of the
//License, or (at your option) any later version.

//This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
//even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
//General Public License for more details.

//You should have received a copy of the GNU General Public License along with this program. If
//not, see <https://www.gnu.org/licenses/>.

use crate::attribute;
use proc_macro::TokenStream;
use quote::quote;

pub(crate) fn impl_description(attrs: std::slice::Iter<'_, syn::Attribute>) -> proc_macro2::TokenStream {
    attrs
        .filter(attribute::is("doc"))
        .filter_map(attribute::extract_string)
        .reduce(|prev, nstr| format!("{}\n{}", prev, nstr))
        .map_or(quote! { None }, |val| quote! { Some(#val) })
}


fn is_subargument(attr: &syn::Attribute) -> bool {
    attribute::is("group")(&attr) || attribute::is("choices")(&attr)
}

fn impl_field_as_arg(
    syn::Field {
        ty, ident, attrs, ..
    }: &syn::Field,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    match ty {
        syn::Type::Path(syn::TypePath {
            path: syn::Path { segments, .. },
            ..
        }) if segments.last().is_some() => {
            let name = if let Some(i) = &ident {
                i
            } else {
                &(segments.last().unwrap().ident)
            };
            let description = impl_description(attrs.iter());
            Ok(if attrs.iter().any(is_subargument) {
                quote!{
                    clipv::describe::arg::Arg::with_type(
                        stringify!(#name), #description, #ty::arguments()
                    ),
                }
            } else {
                quote!{
                    clipv::describe::arg::Arg::new(
                        stringify!(#name), #description
                    ),
                }
            })
        }
        _ => Err(syn::Error::new_spanned(ty, "Unsupported argument type")),
    }
}

fn impl_fields_as_arg(
    fields: syn::punctuated::Iter<'_, syn::Field>,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut arguments = proc_macro2::TokenStream::new();
    for field in fields {
        arguments.extend(impl_field_as_arg(field)?);
    }
    Ok(arguments)
}

fn impl_struct_field_as_arg(fields: &syn::Fields) -> Result<proc_macro2::TokenStream, syn::Error> {
    match fields {
        // it has no arguments
        syn::Fields::Unit => Ok(proc_macro2::TokenStream::new()),
        syn::Fields::Named(syn::FieldsNamed { named: fields, .. })
        | syn::Fields::Unnamed(syn::FieldsUnnamed {
            unnamed: fields, ..
        }) => impl_fields_as_arg(fields.iter()),
    }
}

pub(crate) fn impl_enum_variant_as_arg(
    variants: syn::punctuated::Iter<'_, syn::Variant>,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut arguments = proc_macro2::TokenStream::new();
    for syn::Variant {
        ident,
        attrs,
        fields,
        ..
    } in variants
    {
        let description = impl_description(attrs.iter());
        if let syn::Fields::Unit = fields {
            arguments.extend(quote! {
                clipv::describe::arg::Arg::new(stringify!(#ident), #description),
            });
        } else {
            let sub_arguments = impl_struct_field_as_arg(fields)?;
            arguments.extend(quote! {
                clipv::describe::arg::Arg::with_type(stringify!(#ident), #description, clipv::describe::arg::ArgType::Group(clipv::describe::arg::ArgGroup(vec![
                    #sub_arguments
                ]))),
            })
        }
    }
    Ok(arguments)
}

pub(crate) fn impl_as_arg(ast: &syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    let name = &ast.ident;
    let inner = match &ast.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            let arguments = impl_struct_field_as_arg(fields)?;
            quote! { clipv::describe::arg::ArgType::Group(clipv::describe::arg::ArgGroup(vec![#arguments])) }
        }
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let arguments = impl_enum_variant_as_arg(variants.iter())?;
            quote! { clipv::describe::arg::ArgType::Choices(clipv::describe::arg::Choices(vec![#arguments])) }
        }
        syn::Data::Union(syn::DataUnion { union_token, .. }) => {
            return Err(syn::Error::new_spanned(
                union_token,
                "union aren't yet supported",
            ))
        }
    };
    Ok(quote! {
        impl clipv::describe::arg::AsArg for #name {
            fn arguments() -> clipv::describe::arg::ArgType {
                #inner
            }
        }
    })
}

pub(crate) fn impl_as_arg_macro(ast: &syn::DeriveInput) -> TokenStream {
    impl_as_arg(ast)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
