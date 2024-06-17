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

use proc_macro::TokenStream;
use quote::quote;

fn impl_as_command_from_arg(syn::DeriveInput {
    ident, attrs, ..
}: &syn::DeriveInput, arguments: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let description = crate::as_arg::impl_description(attrs.iter());
    quote! {
        impl clipv::describe::command::AsCommand for #ident {
            fn command() -> clipv::describe::command::Command {
                let mut cmd = clipv::describe::command::Command::new(
                    stringify!(#ident),
                    #description
                );
                cmd.set_arguments(vec![
                    clipv::describe::arg::Arg::with_type(
                        stringify!(#ident), None,
                        clipv::describe::arg::ArgType::Choices(
                            clipv::describe::arg::Choices(vec![#arguments])
                        )
                    )
                ]);
                cmd
            }
        }
    }
}

fn impl_as_command(ast: &syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
            // fn name() -> &'static str { stringify!(#ident) }
    match &ast.data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let as_command = impl_as_command_from_arg(ast, crate::as_arg::impl_enum_variant_as_arg(variants.iter())?);
            // let as_arg = crate::as_arg::impl_as_arg(ast)?;
            Ok(quote!{ #as_command })
        },
        syn::Data::Struct(syn::DataStruct { struct_token, .. }) => Err(
            syn::Error::new_spanned(struct_token, "only enum can be defined as command")
        ),
        syn::Data::Union(syn::DataUnion { union_token, .. }) => Err(
            syn::Error::new_spanned(union_token, "Union aren't supported as commands")
        )
    }
}

pub(crate) fn impl_as_command_macro(ast: &syn::DeriveInput) -> TokenStream {
    impl_as_command(ast)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
