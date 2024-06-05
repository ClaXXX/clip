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
use std::vec::Vec;

#[derive(Debug, PartialEq)]
pub struct ArgGroup(pub Vec<Arg>);

#[derive(Debug, PartialEq)]
pub struct Choices(pub Vec<Arg>);

#[derive(Debug, PartialEq)]
pub struct Commands(Vec<Arg>);

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
    /// enumeration, displayed within command list
    Commands(Commands),
}

impl Into<u8> for &ArgType {
    fn into(self) -> u8 {
        match self {
            ArgType::Value => 0,
            ArgType::Choices(_) => ArgType::Choices as u8,
            ArgType::Group(_) => ArgType::Group as u8,
            ArgType::Commands(_) => ArgType::Commands as u8,
        }
    }
}

impl ArgType {
    /// Gets the inner arguments Choices, Group and Commands hold
    /// Gets subtree or None
    fn get_args(&self) -> Option<&Vec<Arg>> {
        match self {
            ArgType::Choices(Choices(args))
            | ArgType::Group(ArgGroup(args))
            | ArgType::Commands(Commands(args)) => Some(args),
            _ => None,
        }
    }

    /// Get the maximum depth of all the arguments it contains
    fn max_depth(&self) -> usize {
        if let Some(args) = self.get_args() {
            args.iter()
                .map(|a| a.max_depth)
                .reduce(|a, b| if a > b { a } else { b })
                .unwrap_or(1)
        } else {
            1
        }
    }

    /// Tries to give a description in only one line, to summaries all arguments it contains
    /// Used to display the usage
    pub fn summarize(&self, fmts: &std::collections::HashMap<u8, Formatter<'_>>) -> Option<String> {
        if let Some(args) = self.get_args() {
            let index: u8 = self.into();
            Some(
                fmts.get(&index)
                    .unwrap_or(&Formatter::default())
                    .fmt(args.iter(), |arg: &Arg| Some(arg.summarize(fmts))),
            )
        } else {
            None
        }
    }

    /// Gives all argument details, including their description
    /// don't support much recursion
    pub fn details(&self, fmts: &std::collections::HashMap<u8, Formatter<'_>>) -> Option<String> {
        if let Some(args) = self.get_args() {
            let index: u8 = self.into();
            Some(
                fmts.get(&index)
                    .unwrap_or(&Formatter::default())
                    .fmt(args.iter(), |arg: &Arg| arg.details(fmts)),
            )
        } else {
            None
        }
    }
}

/// Adds characters to each line of a string
fn start_with(string: String, chars: &'static str) -> String {
    let mut result = String::new();
    for line in string.lines() {
        result.push_str(format!("{chars}{line}\n").as_str());
    }
    result
}

/// Argument tree root: a single field with its associated type (if needed, recursivly display all argument)
/// It contains the description of the argument itself and its type. It is a node of the tree. It
/// being a leaf is determined by it type.
///
#[derive(Debug, PartialEq)]
pub struct Arg {
    /// Argument field name
    pub name: &'static str,
    /// description associated to the argument and displayed beside its name
    pub description: Option<&'static str>,
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
        let max_depth = r#type.max_depth() + 1;
        Arg {
            name,
            description,
            r#type,
            max_depth,
        }
    }

    /// default constructor, by default we expect a single simple value as a field
    pub fn new(name: &'static str, description: Option<&'static str>) -> Arg {
        Arg {
            name,
            description,
            r#type: ArgType::Value,
            max_depth: 1,
        }
    }

    /// When displaying the usage, only leafs should be showed and explained.
    /// Depending on their type, it could make sens to have them gathered together as an
    /// enum for example or even a group.
    /// This function serves to determine whether the argument is considered an leaf or not
    fn can_be_explained(&self) -> bool {
        match self.r#type {
            ArgType::Value | ArgType::Commands(_) | ArgType::Choices(_) if self.max_depth <= 2 => {
                true
            }
            _ => false,
        }
    }

    /// Summarize argument order and name to details afterwards
    pub fn summarize(&self, fmts: &std::collections::HashMap<u8, Formatter<'_>>) -> String {
        if self.can_be_explained() {
            // displays only the name if the commands or choices depth are not more than two.
            // since it would mean a complicated argument description
            String::from(self.name)
        } else {
            /// since values are filtered with the condition above, the return type cannot be none
            self.r#type.summarize(fmts).unwrap()
        }
    }

    pub fn details(&self, fmts: &std::collections::HashMap<u8, Formatter<'_>>) -> Option<String> {
        if self.max_depth <= 2 {
            let test = if let Some(description) = self.description {
                format!("{:8}{}", self.name, description)
            } else {
                String::from(self.name)
            };
            if let ArgType::Value = self.r#type {
                Some(format!("{test}\n"))
            } else if let Some(details) = self.r#type.details(fmts) {
                Some(format!("{}\n{}", test, start_with(details, "  ")))
            } else {
                None
            }
        } else {
            self.r#type.details(fmts)
        }
    }
}

pub fn default_summary_formatters<'a>() -> std::collections::HashMap<u8, Formatter<'a>> {
    std::collections::HashMap::from([
        (
            ArgType::Group as u8,
            Formatter {
                middle: Some(" "),
                start: Some("<"),
                end: Some(">"),
                ..Default::default()
            },
        ),
        (
            ArgType::Choices as u8,
            Formatter {
                very_start: Some("<"),
                very_end: Some(">"),
                middle: Some("|"),
                ..Default::default()
            },
        ),
        (
            ArgType::Commands as u8,
            Formatter {
                very_start: Some("{"),
                very_end: Some("}"),
                middle: Some("|"),
                ..Default::default()
            },
        ),
    ])
}

pub fn default_details_formatters<'a>() -> std::collections::HashMap<u8, Formatter<'a>> {
    std::collections::HashMap::from([(
        ArgType::Choices as u8,
        Formatter {
            start: Some("- "),
            ..Default::default()
        },
    )])
}

impl std::fmt::Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_str(&self.summarize(&default_summary_formatters()).as_str())
        } else {
            if let Some(description) = self.details(&default_details_formatters()) {
                f.write_str(&description.as_str())
            } else {
                Ok(())
            }
        }
    }
}

/// Provides a method returning the object as a list of displayable arguments
pub trait AsArg {
    /// Required method
    /// Return either choices for an enum or a group for a struct
    fn arguments() -> ArgType;
}

/// provides helper functions to describe a command
pub trait AsCommand: AsArg {
    /// Required methods
    fn name() -> &'static str;
    fn commands() -> Commands;
    /// summarizes expected types and/or values
    fn summary() -> &'static str;
    /// Optional methods
    fn usage() -> String {
        format!(
            "{} {} {} ...",
            Self::name(),
            Self::arguments()
                .summarize(&default_summary_formatters())
                .expect("hello"),
            ArgType::Commands(Self::commands())
                .summarize(&default_summary_formatters())
                .expect("hola")
        )
    }
    fn help() -> String {
        let arguments = start_with(
            Self::arguments()
                .details(&default_details_formatters())
                .expect("expected some value there"),
            "  ",
        );
        let commands = start_with(
            ArgType::Commands(Self::commands())
                .details(&default_details_formatters())
                .expect("hello"),
            "  ",
        );
        format!(
            r#"
{}

Usage: {}

Arguments:
{}
Commands:
{}
"#,
            Self::summary(),
            Self::usage(),
            arguments,
            commands
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl From<Choices> for Commands {
        fn from(choices: Choices) -> Self {
            Commands(match choices {
                Choices(list) => list,
            })
        }
    }

    enum Number {
        One,
        Two,
        Three,
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
            ArgType::Group(ArgGroup(number_argument())).summarize(&default_summary_formatters()),
            Some("<One> <Two> <Three>".to_string()),
        );
        // a list of possible choices
        assert_eq!(
            Number::arguments().summarize(&default_summary_formatters()),
            Some("<One|Two|Three>".to_string())
        );

        assert_eq!(
            ArgType::Commands(Commands(number_argument())).summarize(&default_summary_formatters()),
            Some("{One|Two|Three}".to_string())
        );
    }

    #[test]
    fn it_should_format_one_layer_arguments_alternate() {
        // group of argument
        assert_eq!(
            ArgType::Group(ArgGroup(number_argument())).details(&default_details_formatters()),
            Some(format!("One\nTwo{ws:5}Second argument\nThree\n", ws = ' '))
        );
        // a list of possible choices
        assert_eq!(
            Number::arguments().details(&default_details_formatters()),
            Some(format!(
                "- One\n- Two{ws:5}Second argument\n- Three\n",
                ws = ' '
            ))
        );

        assert_eq!(
            ArgType::Commands(Commands(number_argument())).details(&default_details_formatters()),
            Some(format!("One\nTwo{ws:5}Second argument\nThree\n", ws = ' '))
        );
    }

    struct Tata {
        titi: u8,
        tutu: Number,
    }

    enum Complexe {
        One(Tata),
        Two { test: u8, tata: Tata },
        Three,
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
        cmd: Number,
        tata: Number,
        toto: u8,
        complexe: Complexe,
        titi: Tata,
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
        assert_eq!(
            Test::arguments().details(&default_details_formatters()),
            Some(
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
            )
        );
    }

    impl AsCommand for Test {
        fn name() -> &'static str {
            "test"
        }
        fn summary() -> &'static str {
            "Test to display a command description"
        }
        fn commands() -> Commands {
            if let ArgType::Choices(choices) = Number::arguments() {
                Commands::from(choices)
            } else {
                unreachable!();
            }
        }
    }
}
