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
/// For this, first, it describes each rust data object as a list of argument with a certain type

use std::vec::Vec;

/// All argument type supporting a formatting
#[derive(Default)]
pub enum ArgType {
    /// default type holds no additional value
    #[default]
    Value,
    /// enumeration argument type
    Choices(Vec<Arg>),
    /// struct argument type
    Group(Vec<Arg>),
}

/// a single field with its associated type (if needed, recursivly display all argument)
pub struct Arg {
    /// Argument field name
    pub name: &'static str,
    /// description associated to the argument and displayed beside its name
    pub description: Option<&'static str>,
    /// type of argument determining when and what to display
    pub r#type: ArgType,
}

impl Arg {
    /// default constructor, by default we expect a single simple value as a field
    pub fn new(name: &'static str, description: Option<&'static str>) -> Arg {
        Arg {
            name,
            description,
            r#type: ArgType::Value,
        }
    }

    pub fn group(name: &'static str, description: Option<&'static str>, subargs: Vec<Arg>) -> Arg {
        Arg {
            name,
            description,
            r#type: ArgType::Group(subargs),
        }
    }

    pub fn choices(
        name: &'static str,
        description: Option<&'static str>,
        choices: Vec<Arg>,
    ) -> Arg {
        Arg {
            name,
            description,
            r#type: ArgType::Choices(choices),
        }
    }
}

impl std::fmt::Display for ArgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgType::Choices(choices) => {
                for choice in choices.iter() {
                    write!(f, "  - {}", choice)?;
                }
            }
            ArgType::Group(args) => {
                for arg in args.iter() {
                    write!(f, "  {}", arg)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl std::fmt::Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(description) = self.description {
            writeln!(f, "{:8}{}", self.name, description)
        } else {
            writeln!(f, "{}", self.name)
        }?;
        if let ArgType::Group(_) | ArgType::Choices(_) = self.r#type {
            for line in self.r#type.to_string().lines() {
                writeln!(f, "  {}", line)?;
            }
        }
        Ok(())
    }
}

/// Provides a method returning the object as a list of displayable arguments
pub trait AsArg<const N: usize> {
    /// Required method
    fn arguments() -> [Arg; N];
}

#[cfg(test)]
mod tests {
    use super::*;

    enum Number {
        One,
        Two,
        Three,
    }

    impl AsArg<3> for Number {
        fn arguments() -> [Arg; 3] {
            [
                Arg::new("One", None),
                Arg::new("Two", Some("Second argument")),
                Arg::new("Three", None),
            ]
        }
    }

    #[test]
    fn it_should_format_one_layer_arguments() {
        // group of argument
        assert_eq!(
            format!("{}", ArgType::Group(Vec::from(Number::arguments()))),
            format!(
                "{ws:2}One\n{ws:2}Two{ws:5}Second argument\n{ws:2}Three\n",
                ws = ' '
            )
        );
        // a list of possible choices
        assert_eq!(
            format!("{}", ArgType::Choices(Vec::from(Number::arguments()))),
            format!(
                "{ws:2}- One\n{ws:2}- Two{ws:5}Second argument\n{ws:2}- Three\n",
                ws = ' '
            )
        );
    }

    struct Tata {
        titi: u8,
        tutu: Number,
    }

    impl AsArg<2> for Tata {
        fn arguments() -> [Arg; 2] {
            [
                Arg::new("titi", None),
                Arg::choices(
                    "tutu",
                    Some("tutu is the second argument"),
                    Vec::from(Number::arguments()),
                ),
            ]
        }
    }

    /// Test to display a command description
    struct Test {
        tata: Number,
        toto: u8,
        titi: Tata,
    }

    impl AsArg<3> for Test {
        fn arguments() -> [Arg; 3] {
            [
                Arg::choices(
                    "tata",
                    Some("a list of possibilities"),
                    Vec::from(Number::arguments()),
                ),
                Arg::new("toto", Some("number of something")),
                Arg::group("titi", None, Vec::from(Tata::arguments())),
            ]
        }
    }

    #[test]
    fn it_should_support_recusion() {
        assert_eq!(format!("{}", ArgType::Group(Vec::from(Test::arguments()))), format!("{ws:2}tata{ws:4}a list of possibilities\n{ws:4}- One\n{ws:4}- Two{ws:5}Second argument\n{ws:4}- Three\n{ws:2}toto{ws:4}number of something\n{ws:2}titi\n{ws:4}titi\n{ws:4}tutu{ws:4}tutu is the second argument\n{ws:6}- One\n{ws:6}- Two{ws:5}Second argument\n{ws:6}- Three\n", ws = ' '));
    }

}
