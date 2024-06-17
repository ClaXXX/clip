//SPDX-FileCopyrightText: 2024 Claire Bts <claxxx.bts@gmail.com>
//SPDX-License-Identifier: GPL-3.0-or-later

// clipv aims to simplify writing cli and/or parser in general

//Copyright (C) 2024 Claire Bts claxxx.bts@gmail.com

//This program is free software: you can redistribute it and/or modify it under the terms of the
//GNU General Public License as published by the Free Software Foundation, either version 3 of the
//License, or (at your option) any later version.

//This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
//even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
//General Public License for more details.

//You should have received a copy of the GNU General Public License along with this program. If
//not, see <https://www.gnu.org/licenses/>.


#[cfg(feature = "derive")]
mod derive_test {
use clipv::describe::arg::{Arg, ArgGroup, ArgType, AsArg, Choices};
use clipv::AsArg;

#[allow(dead_code)]
#[derive(AsArg)]
struct SimpleNamed {
    /// first structure field
    var1: u8,
    var2: String,
    /// This is a more
    /// complexe documentation
    var3: usize,
}

#[test]
fn it_should_support_struct_with_named_and_single_depth_fields() {
    assert_eq!(
        SimpleNamed::arguments(),
        ArgType::Group(ArgGroup(vec![
            Arg::new("var1", Some("first structure field")),
            Arg::new("var2", None),
            Arg::new("var3", Some("This is a more\ncomplexe documentation"))
        ]))
    );
}

#[allow(dead_code)]
#[derive(AsArg)]
enum SimpleEnum {
    Variant1,
    /// provides some documentation
    Variant2,
}

fn simple_enum_arguments() -> ArgType {
    ArgType::Choices(Choices(vec![
        Arg::new("Variant1", None),
        Arg::new("Variant2", Some("provides some documentation")),
    ]))
}

#[test]
fn it_should_support_enum_with_a_single_depth_field() {
    assert_eq!(SimpleEnum::arguments(), simple_enum_arguments());
}

#[allow(dead_code)]
#[derive(AsArg)]
struct SimpleUnnamed(
    /// first structure field
    u8,
    std::string::String,
    /// This is a more
    /// complexe documentation
    usize,
);

#[test]
fn it_should_support_struct_with_unnamed_and_single_depth_fields() {
    assert_eq!(
        SimpleUnnamed::arguments(),
        ArgType::Group(ArgGroup(vec![
            Arg::new("u8", Some("first structure field")),
            Arg::new("String", None),
            Arg::new("usize", Some("This is a more\ncomplexe documentation"))
        ]))
    );
}

#[allow(dead_code)]
#[derive(AsArg)]
enum NestedStruct {
    /// this is a nested structure variable
    Struct {
        a: u8,
        /// b is a subvariable of a struct
        b: u8,
    },
    Tuple(u8, u8),
    Unit,
}

fn nested_struct_arguments() -> ArgType {
    ArgType::Choices(Choices(vec![
        Arg::with_type(
            "Struct",
            Some("this is a nested structure variable"),
            ArgType::Group(ArgGroup(vec![
                Arg::new("a", None),
                Arg::new("b", Some("b is a subvariable of a struct")),
            ])),
        ),
        Arg::with_type(
            "Tuple",
            None,
            ArgType::Group(ArgGroup(vec![Arg::new("u8", None), Arg::new("u8", None)])),
        ),
        Arg::new("Unit", None),
    ]))
}

#[test]
fn it_should_support_nested_struct() {
    assert_eq!(NestedStruct::arguments(), nested_struct_arguments());
}

#[allow(dead_code)]
#[derive(AsArg)]
struct NestedStructWithAttributes {
    /// subgroup searching for the nestedstruct arguments
    #[group]
    nested_struct: NestedStruct,
    #[choices]
    /// sub enum
    subenum: SimpleEnum,
}

#[test]
fn it_should_support_arg_attribute() {
    assert_eq!(
        NestedStructWithAttributes::arguments(),
        ArgType::Group(ArgGroup(vec![
            Arg::with_type(
                "nested_struct",
                Some("subgroup searching for the nestedstruct arguments"),
                nested_struct_arguments()
            ),
            Arg::with_type("subenum", Some("sub enum"), simple_enum_arguments())
        ]))
    );
}
}
