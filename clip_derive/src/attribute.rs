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

/// Creates a closure to identify an attribute by its name
///
/// The created closure only supports one path attribute
/// For instance, `#[this::is::an::example]` won't work
pub(crate) fn is(name: &'static str) -> Box<dyn Fn(&&syn::Attribute) -> bool> {
    return Box::new(move |attr: &&syn::Attribute| {
        (*attr)
            .path()
            .get_ident()
            .is_some_and(|ident| *ident == name)
    });
}

/// From a syn::Attribute TokenStream, try to retrieve a Literal String
///
/// Returns None, if it cannot be parsed or doesn't correspond to a literal string
pub(crate) fn extract_string(attr: &syn::Attribute) -> Option<String> {
    if let syn::Meta::NameValue(syn::MetaNameValue {
        value:
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }),
        ..
    }) = &attr.meta
    {
        Some(String::from(
            lit_str
                .value()
                .trim_matches(|c| c == '"' || c == '\'' || c == ' '),
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {}
