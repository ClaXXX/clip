// Copyright © 2024 Claire Bts
//
// This file is part of CLIP
//
// CLIP is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// CLIP is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

#[cfg(feature = "derive")]
mod test {
use clipv::parser::{Parsed, ParsingError, TryParse};
use clipv::{FromStr, TryParse};

#[derive(Debug, PartialEq, TryParse)]
struct Empty;

#[derive(Debug, PartialEq, FromStr)]
enum Unit {
    One,
    Two,
    Three,
}

#[derive(Debug, PartialEq, TryParse)]
struct Leaf {
    a: u8,
    b: String,
}

#[derive(Debug, PartialEq, TryParse)]
enum Command {
    Tuple(u8, #[try_parse] Leaf),
    Struct { unit: Unit, other: u8 },
    Unit,
}
#[derive(Debug, PartialEq, TryParse)]
struct Parent {
    #[try_parse]
    parent_arg: Leaf,
    #[try_parse]
    command: Command,
}

#[test]
fn it_parses_a_simple_struct() {
    let arguments = ["32", "Hello, world"];
    let result = Leaf::try_parse(arguments.iter());
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap().0,
        Leaf {
            a: 32,
            b: String::from("Hello, world")
        }
    );
}

#[test]
fn it_should_raise_too_few_argument() {
    let arguments = ["32"];
    let result = Leaf::try_parse(arguments.iter());
    assert_eq!(result.err(), Some(ParsingError::TooFewArguments));
}

#[test]
fn it_should_raise_bad_argument_type() {
    let arguments = ["", "Hello, world"];
    let result = Leaf::try_parse(arguments.iter());
    assert_eq!(result.err(), Some(ParsingError::BadType));
}

#[test]
fn it_should_parse_the_enumeration_group() {
    {
        let arguments = ["tuple", "32", "32", "Hello, world", "following"];
        let result = Command::try_parse(arguments.iter());
        assert!(result.is_ok());
        let Parsed(parsed, mut rest) = result.ok().unwrap();
        assert_eq!(
            parsed,
            Command::Tuple(
                32,
                Leaf {
                    a: 32,
                    b: String::from("Hello, world")
                }
            )
        );
        assert_eq!(rest.next(), Some("following").as_ref());
    }
    {
        let arguments = ["struct", "three", "42"];
        let result = Command::try_parse(arguments.iter());
        assert!(result.is_ok());
        let Parsed(parsed, _) = result.ok().unwrap();
        assert_eq!(
            parsed,
            Command::Struct {
                unit: Unit::Three,
                other: 42
            }
        );
    }
    {
        let arguments = ["unit"];
        let result = Command::try_parse(arguments.iter());
        assert!(result.is_ok());
        let Parsed(parsed, _) = result.ok().unwrap();
        assert_eq!(parsed, Command::Unit);
    }
}

#[test]
fn it_should_raise_variant_not_found_command() {
    let arguments = ["unexistant"];
    let result = Command::try_parse(arguments.iter());
    assert_eq!(result.err(), Some(ParsingError::VariantNotFound));
}

#[test]
fn it_should_raise_too_few_argument_command() {
    {
        let arguments = ["tuple"];
        let result = Command::try_parse(arguments.iter());
        assert_eq!(result.err(), Some(ParsingError::TooFewArguments));
    }
    {
        let arguments: [&'static str; 0] = [];
        let result = Command::try_parse(arguments.iter());
        assert_eq!(result.err(), Some(ParsingError::TooFewArguments));
    }
}

#[test]
fn it_should_raise_bad_argument_type_command() {
    let arguments = ["tuple", "test", "43", "Hello"];
    assert_eq!(
        Command::try_parse(arguments.iter()).err(),
        Some(ParsingError::BadType)
    );
}

#[test]
fn it_should_parse_the_parent() {
    let arguments = ["42", "Thank", "tuple", "32", "32", "Hello, world", "end"];
    let result = Parent::try_parse(arguments.iter());
    assert!(result.is_ok());
    let Parsed(parsed, mut rest) = result.unwrap();
    assert_eq!(
        parsed,
        Parent {
            parent_arg: Leaf {
                a: 42,
                b: String::from("Thank")
            },
            command: Command::Tuple(
                32,
                Leaf {
                    a: 32,
                    b: String::from("Hello, world")
                }
            )
        }
    );
    assert_eq!(rest.next(), Some("end").as_ref());
}
}
