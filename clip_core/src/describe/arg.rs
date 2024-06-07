// Copyright © 2024 Claire Bts
//
// This file is part of CLIP
//
// CLIP is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// CLIP is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

/// module to manage a command line interface description and its parameters
/// Tries to facilitate displaying a help message for a CLI
///
/// For this, first, it describes each rust data object as a tree of argument with a certain type
/// Each type being either a leaf (Value) or a node to subtrees.
///
/// It's composed of two types of arguments' description:
///  - summary: displays the list of argument required (and their order). Choices parent name is
///  used
///  - details: displays each arguments with its description. Here, order doesn't matter. Choices
///  are also details.
///
///  Both type of description are complementary to describe command line arguments
use super::formatter::Formatter;
use super::value::Value;
use std::vec::Vec;

#[derive(Debug, PartialEq, Default)]
pub struct ArgGroup(pub Vec<Arg>);

#[derive(Debug, PartialEq)]
pub struct Choices(pub Vec<Arg>);

pub trait GetArgs {
    /// Required method
    fn get_args(&self) -> &Vec<Arg>;
}

trait ArgTree: GetArgs {
    /// Get the maximum depth of all the arguments it contains
    fn max_depth(&self) -> usize {
        self.get_args()
            .iter()
            .map(|a| a.max_depth)
            .reduce(|a, b| if a > b { a } else { b })
            .unwrap_or(1)
    }
}

pub trait SummaryFormatter {
    /// Optional methods
    fn get_summary_formatter<'a>() -> Formatter<'a> {
        Formatter::default()
    }
}
pub trait ArgSummarize: GetArgs + SummaryFormatter {
    fn summarize(&self) -> String {
        Self::get_summary_formatter().fmt(self.get_args().iter(), |arg: &Arg| Some(arg.summarize()))
    }
}

pub trait DetailsFormatter {
    fn get_details_formatter<'a>() -> Formatter<'a> {
        Formatter::default()
    }
}

pub trait ArgDetails: GetArgs + DetailsFormatter {
    fn details(&self) -> String {
        Self::get_details_formatter().fmt(self.get_args().iter(), |arg: &Arg| Some(arg.details()))
    }
}

impl GetArgs for ArgGroup {
    fn get_args(&self) -> &Vec<Arg> {
        match self {
            ArgGroup(args) => args,
        }
    }
}
impl ArgTree for ArgGroup {}

impl SummaryFormatter for ArgGroup {
    fn get_summary_formatter<'a>() -> Formatter<'a> {
        Formatter {
            middle: Some(" "),
            start: Some("<"),
            end: Some(">"),
            ..Default::default()
        }
    }
}
impl ArgSummarize for ArgGroup {}
impl DetailsFormatter for ArgGroup {}
impl ArgDetails for ArgGroup {}

impl GetArgs for Choices {
    fn get_args(&self) -> &Vec<Arg> {
        match self {
            Choices(args) => args,
        }
    }
}
impl ArgTree for Choices {}

impl SummaryFormatter for Choices {
    fn get_summary_formatter<'a>() -> Formatter<'a> {
        Formatter {
            very_start: Some("<"),
            very_end: Some(">"),
            middle: Some("|"),
            ..Default::default()
        }
    }
}
impl ArgSummarize for Choices {}
impl DetailsFormatter for Choices {
    fn get_details_formatter<'a>() -> Formatter<'a> {
        Formatter {
            start: Some("- "),
            new_line_chars: Some("  "),
            ..Default::default()
        }
    }
}
impl ArgDetails for Choices {}

/// All argument type supporting a formatting
/// Either a leaf or subtree's holder
#[derive(Default, Debug, PartialEq)]
pub enum ArgType {
    /// default type holds no additional value
    #[default]
    Value,
    /// enumeration argument type
    Choices(Choices),
    /// struct argument type
    Group(ArgGroup),
}
/// Argument tree root: a single field with its associated type (if needed, recursivly display all argument)
/// It contains the description of the argument itself and its type. It is a node of the tree. It
/// being a leaf is determined by it type.
///
#[derive(Debug, PartialEq)]
pub struct Arg {
    pub value: Value<'static>,
    /// type of argument determining when and what to display
    pub r#type: ArgType,
    max_depth: usize,
}

impl Arg {
    /// creates an argument with its inner type
    pub fn with_type(
        name: &'static str,
        description: Option<&'static str>,
        r#type: ArgType,
    ) -> Arg {
        let max_depth = match &r#type {
            ArgType::Choices(choices) => choices.max_depth() + 1,
            ArgType::Group(group) => group.max_depth(),
            ArgType::Value => 1,
        };
        Arg {
            value: Value { name, description },
            r#type,
            max_depth,
        }
    }

    /// default constructor, by default we expect a single simple value as a field
    pub fn new(name: &'static str, description: Option<&'static str>) -> Arg {
        Arg {
            value: Value { name, description },
            r#type: ArgType::Value,
            max_depth: 1,
        }
    }

    /// Summarize argument order and name to details afterwards
    pub fn summarize(&self) -> String {
        match &self.r#type {
            ArgType::Value => self.value.name.to_string(),
            // displays only the name if the commands or choices depth are not more than two.
            // since it would mean a complicated argument description
            ArgType::Choices(_) if self.max_depth <= 2 => self.value.name.to_string(),
            ArgType::Choices(choices) => choices.summarize(),
            ArgType::Group(group) => group.summarize(),
        }
    }

    pub fn details(&self) -> String {
        match &self.r#type {
            ArgType::Value => format!("{:#}\n", self.value),
            ArgType::Choices(choices) if self.max_depth <= 2 => {
                format!("{:#}\n{}", self.value, super::formatter::start_with(choices.details(), "  "))
            }
            ArgType::Choices(choices) => choices.details(),
            ArgType::Group(group) => group.details(),
        }
    }
}

impl std::fmt::Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_str(self.summarize().as_str())
        } else {
            f.write_str(self.details().as_str())
        }
    }
}

/// Provides a method returning the object as a list of displayable arguments
pub trait AsArg {
    /// Required method
    /// Return either choices for an enum or a group for a struct
    fn arguments() -> ArgType;
}

#[cfg(test)]
mod tests {
    use super::*;

    enum Number {
        // One,
        // Two,
        // Three,
    }

    fn number_argument() -> Vec<Arg> {
        vec![
            Arg::new("One", None),
            Arg::new("Two", Some("Second argument")),
            Arg::new("Three", None),
        ]
    }

    impl AsArg for Number {
        fn arguments() -> ArgType {
            ArgType::Choices(Choices(number_argument()))
        }
    }

    #[test]
    fn it_should_format_one_layer_arguments_nominale() {
        // group of argument
        assert_eq!(
            ArgGroup(number_argument()).summarize(),
            "<One> <Two> <Three>".to_string()
        );
        // a list of possible choices
        assert_eq!(
            Choices(number_argument()).summarize(),
            "<One|Two|Three>".to_string()
        );
    }

    #[test]
    fn it_should_format_one_layer_arguments_alternate() {
        // group of argument
        assert_eq!(
            ArgGroup(number_argument()).details(),
            format!("One\nTwo{ws:5}Second argument\nThree\n", ws = ' ')
        );
        // a list of possible choices
        assert_eq!(
            Choices(number_argument()).details(),
            format!("- One\n- Two{ws:5}Second argument\n- Three\n", ws = ' ')
        );
    }

    struct Tata {
        // titi: u8,
        // tutu: Number,
    }

    enum Complexe {
        // One(Tata),
        // Two { test: u8, tata: Tata },
        // Three,
    }

    impl AsArg for Tata {
        fn arguments() -> ArgType {
            ArgType::Group(ArgGroup(vec![
                Arg::new(
                    "titi",
                    Some("This titi belongs to the Tata struct and is an unsigned integer"),
                ),
                Arg::with_type(
                    "tutu",
                    Some("tutu is the second argument"),
                    Number::arguments(),
                ),
            ]))
        }
    }

    /// expects
    /// {One,Two,Three} {args...}
    ///
    /// - One <titi> <tutu>
    /// - Two <test> <tata>
    /// - Three
    impl AsArg for Complexe {
        fn arguments() -> ArgType {
            ArgType::Choices(Choices(vec![
                Arg::with_type("One", None, Tata::arguments()),
                Arg::with_type(
                    "Two",
                    None,
                    ArgType::Group(ArgGroup(vec![
                        Arg::new(
                            "test",
                            Some("This test belongs to the Two value of complexe"),
                        ),
                        Arg::with_type("tata", None, Tata::arguments()),
                    ])),
                ),
                Arg::new("Three", None),
            ]))
        }
    }

    /// Test to display a command description
    struct Test {
        // cmd: Number,
        // tata: Number,
        // toto: u8,
        // complexe: Complexe,
        // titi: Tata,
    }

    impl AsArg for Test {
        fn arguments() -> ArgType {
            ArgType::Group(ArgGroup(vec![
                Arg::with_type("tata", Some("a list of possibilities"), Number::arguments()),
                Arg::new("toto", Some("number of something")),
                Arg::with_type(
                    "titi",
                    Some("This titi is part of the whole Test structure and is a struct"),
                    Tata::arguments(),
                ),
                Arg::with_type("complexe", None, Complexe::arguments()),
            ]))
        }
    }

    #[test]
    fn it_should_display_recusion_behavior() {
        let test = match Test::arguments() {
            ArgType::Group(group) => group.details(),
            _ => unreachable!(),
        };
        assert_eq!(
            test,
            r#"tata    a list of possibilities
  - One
  - Two     Second argument
  - Three
toto    number of something
titi    This titi belongs to the Tata struct and is an unsigned integer
tutu    tutu is the second argument
  - One
  - Two     Second argument
  - Three
- titi    This titi belongs to the Tata struct and is an unsigned integer
  tutu    tutu is the second argument
    - One
    - Two     Second argument
    - Three
- test    This test belongs to the Two value of complexe
  titi    This titi belongs to the Tata struct and is an unsigned integer
  tutu    tutu is the second argument
    - One
    - Two     Second argument
    - Three
- Three
"#
            .to_string()
        );
    }
}
