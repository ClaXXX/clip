// Copyright © 2024 Claire Bts
//
// This file is part of CLIP
//
// CLIP is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// 
// CLIP is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.



mod from_str;
mod try_parse;
use proc_macro::TokenStream;

/// FromStr Derive attribute
///
/// It is the derive macro for the FromStr trait
///
/// Only for Unit enum, any other type is unsupported and will an error at compile time.
///
/// This macro is just a conveniant way to parse a string into the derived enumeration. It is case
/// insensitive. The behavior is actually the same as for the TryParse trait.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate clip_derive;
/// use clip_derive::FromStr;
/// use std::str::FromStr;
/// 
/// ##[derive(Debug, PartialEq, FromStr)]
/// enum Random { One, Two, Three }
///
/// # fn main() {
/// assert_eq!(Random::from_str("one"), Ok(Random::One));
/// assert_eq!(Random::from_str("THREE"), Ok(Random::Three));
/// assert!(Random::from_str("Four").is_err());
/// # }
/// ```
#[proc_macro_derive(FromStr)]
pub fn from_str_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    crate::from_str::impl_from_str_macro(&ast)
}

/// TryParse devive attribute
///
/// It is the Derive macro for the TryParse trait
///
/// Supports Struct and Enum but not Union
///
/// The try_parse helper attribute may be associated to a field to use the TryParse::try_parse
/// method instead of the default str.parse one.
///
/// Since the FromStr and TryParse trait cannot be checked for a certain trait, generated errors
/// can lead to crypted error messages. However since there is a few number of errors' sources:
///  - trying to derive an union, which should give an explicit error
///  - having an attributeless fields for which the type doesn't implement FromStr
///  - having a `#[try_parse]` attributed field for which the type doesn't implement TryParse
/// If the error seems hard to decrypt, chances are high that the problem is one of the last two.
///
/// # Struct
/// Only positional arguments are supported, meaning there is no concept of optional or flags for
/// now. The position of a field within itself will determine the expected position within a parsed
/// line.
///
/// For instance the following structure will expect `<titi> <tata> <toto>` in this order only.
/// ```
/// # #[macro_use] extern crate clip_derive;
/// # extern crate clip_core;
/// use clip_core::parser::{Parsed, ParsingError, TryParse};
/// use clip_derive::TryParse;
///
/// ##[derive(TryParse)]
/// struct Toto {
///     titi: String,
///     tata: u8,
///     toto: u8,
/// }
/// ```
/// 
///
/// # Enum
/// For an enumeration, the first positional parameter corresponds to the Variant (case insensitive
/// match) that should be initialized and the following value are used if for the Variant
/// initialisation.
///
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate clip_derive;
/// # extern crate clip_core;
/// use clip_core::parser::{Parsed, ParsingError, TryParse};
/// use clip_derive::TryParse;
///
/// ##[derive(Debug, PartialEq, TryParse)]
/// enum Tata { One, Two, Three }
/// ##[derive(TryParse)]
/// struct Toto {
///     ##[try_parse] tata: Tata,
///     titi: u8
/// }
///
/// fn main() {
///     let arguments = [ "one", "32" ];
///     let result = Toto::try_parse(arguments.iter());
///     assert!(result.is_ok());
///     let Parsed (parsed, _) = result.unwrap();
///     assert_eq!(parsed.tata, Tata::One);
///     assert_eq!(parsed.titi, 32);
/// }
/// ```
///
#[proc_macro_derive(TryParse, attributes(try_parse))]
pub fn try_parse_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    crate::try_parse::impl_try_parse_macro(&ast)
}
