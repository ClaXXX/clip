//SPDX-FileCopyrightText: 2024 Claire Bts <claxxx.bts@gmail.com>
//SPDX-License-Identifier: GPL-3.0-or-later

// clipv aims to simplify writing cli and/or parser in general

//Copyright (C) 2024 Claire Bts claxxx.bts@gmail.com

//This program is free software: you can redistribute it and/or modify it under the terms of the
//GNU General Public License as published by the Free Software Foundation, either version 3 of the
//License, or (at your option) any later version.

//even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
//General Public License for more details.

//You should have received a copy of the GNU General Public License along with this program. If
//not, see <https://www.gnu.org/licenses/>.

#[cfg(feature = "derive")]
mod derive_test {
use clipv::describe::command::AsCommand;
use clipv::{AsCommand, AsArg};

#[allow(dead_code)]
#[derive(AsCommand)]
/// description
enum SimpleEnum {
    Variant1,
    /// provides some documentation
    Variant2,
}
#[test]
fn it_should_display_the_usage() {
    assert_eq!(SimpleEnum::help(), r#"description

Usage: SimpleEnum <SimpleEnum>

Arguments:
  SimpleEnum
    - Variant1
    - Variant2provides some documentation
"#);
}

#[derive(Debug,AsArg)]
struct EmptyArg;

#[allow(dead_code)]
#[derive(Debug, AsCommand)]
enum NestedEnum {
    Tuple (u8, String),
    Struct { a: u8, b: String },
    #[group]
    SubArg(EmptyArg),
    Unit
}

#[test]
fn it_should_support_nested_arguments() {
    assert_eq!(NestedEnum::help(), r#"Usage: NestedEnum <NestedEnum>

Arguments:
  NestedEnum
    - u8
      String
    - a
      b
    - EmptyArg
    - Unit
"#);
}

#[allow(dead_code)]
#[derive(AsCommand)]
#[command]
/// description
enum SimpleCommad {
    Variant1,
    /// provides some documentation
    Variant2,
}
}
