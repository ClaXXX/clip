//SPDX-FileCopyrightText: 2024 Claire Bts <claxxx.bts@gmail.com>
//SPDX-License-Identifier: GPL-3.0-or-later

// clip_core aims to simplify writing cli and/or parser in general

//Copyright (C) 2024 Claire Bts claxxx.bts@gmail.com

//This program is free software: you can redistribute it and/or modify it under the terms of the
//GNU General Public License as published by the Free Software Foundation, either version 3 of the
//License, or (at your option) any later version.

//This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
//even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
//General Public License for more details.

//You should have received a copy of the GNU General Public License along with this program. If
//not, see <https://www.gnu.org/licenses/>.

use super::arg::{Arg, ArgDetails, ArgGroup, ArgSummarize, DetailsFormatter, GetArgs};
use super::value::Value;
use super::formatter::start_with;

#[derive(Debug, PartialEq)]
pub struct Command {
    pub value: Value<'static>,
    pub subcommands: Option<Vec<Command>>,
    pub arguments: ArgGroup,
}

impl DetailsFormatter for Command {}

impl Command {
    pub fn new(name: &'static str, description: Option<&'static str>) -> Self {
        Self {
            value: Value {
                name,
                description,
            },
            subcommands: None,
            arguments: ArgGroup(Vec::new()),
        }
    }
    pub fn set_subcommands(&mut self, subcommands: Vec<Command>) {
        self.subcommands = Some(subcommands);
    }
    pub fn set_arguments(&mut self, arguments: Vec<Arg>) {
        match &mut self.arguments { ArgGroup(args) => args }.extend(arguments);
    }

    fn summarize(&self) -> String {
        let mut result = format!("{}", self.value);
        if !self.arguments.get_args().is_empty() {
            result.push_str(format!(" {}", self.arguments.summarize()).as_str());
        }
        if self.subcommands.is_some() {
            result.push_str(" [COMMAND] ..");
        }
        result
    }

    fn arguments_details(&self) -> String {
        format!("Arguments:\n{}", start_with(self.arguments.details(), "  "))
    }

    fn command_details(&self, command: &[Command]) -> String {
        format!("Commands:\n{}", start_with(
            Self::get_details_formatter().fmt(command.iter(), |cmd| Some(format!("{:#}\n", cmd.value))), "  "
        ))
    }
    fn details(&self) -> String {
        let mut result = String::new();
        if !self.arguments.get_args().is_empty() {
            result.push_str(
                self.arguments_details().as_str(),
            );
            if self.subcommands.is_some() { result.push('\n'); }
        }
        if let Some(commands) = &self.subcommands {
            result.push_str(
                self.command_details(commands).as_str(),
            );
        }
        result
    }
}

/// provides helper functions to describe a command
pub trait AsCommand {
    /// Required methods
    fn command() -> Command;
    /// Optional methods
    fn help() -> String {
        let command = Self::command();
        format!(
            "{}Usage: {}\n\n{}",
            if let Some(description) = command.value.description {
                format!("{}\n\n", description)
            } else {
                String::new()
            },
            command.summarize(),
            command.details()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::describe::arg::ArgGroup;

    #[derive(Debug)]
    enum Number {
        // One,
        // Two,
        // Three,
    }

    impl AsCommand for Number {
        fn command() -> Command {
            Command {
                value: Value {
                    name: "cli",
                    description: None,
                },
                subcommands: Some(vec![
                    Command::new("One", None),
                    Command::new("Two", Some("Second command")),
                    Command::new("Three", None),
                ]),
                arguments: ArgGroup::default(),
            }
        }
    }

    #[test]
    fn it_should_format_one_layer_commands() {
        assert_eq!(
            Number::command().summarize(),
            "cli [COMMAND] ..".to_string()
        );

        assert_eq!(
            Number::command().details(),
            format!(
                "Commands:\n  One\n  Two{ws:5}Second command\n  Three\n",
                ws = ' '
            )
        );
    }

    #[test]
    fn it_should_display_description() {
        assert_eq!(
            Number::help(),
            r#"Usage: cli [COMMAND] ..

Commands:
  One
  Two     Second command
  Three
"#
        );
    }

    #[derive(Debug)]
    struct Complexe {
        // arg1: String,
        // arg2: u8,
        // subcommand: Number,
    }

    impl AsCommand for Complexe {
        fn command() -> Command {
            Command {
                value: Value {
                    name: "complexe",
                    description: Some("Complexified cli test"),
                },
                subcommands: Some(vec![
                    Command::new("One", None),
                    Command::new("Two", Some("Second command")),
                    Command::new("Three", None),
                ]),
                arguments: ArgGroup(vec![
                    Arg::new("arg1", None),
                    Arg::new("arg2", Some("Second argument")),
                ]),
            }
        }
    }

    #[test]
    fn arg_and_command_summary() {
        assert_eq!(
            Complexe::command().summarize(),
            "complexe <arg1> <arg2> [COMMAND] ..".to_string()
        );
    }

    #[test]
    fn arg_and_command_details() {
        assert_eq!(
            Complexe::command().details(),
            r#"Arguments:
  arg1
  arg2    Second argument

Commands:
  One
  Two     Second command
  Three
"#
        );
    }

    #[test]
    fn arg_and_command_help() {
        assert_eq!(
            Complexe::help(),
            r#"Complexified cli test

Usage: complexe <arg1> <arg2> [COMMAND] ..

Arguments:
  arg1
  arg2    Second argument

Commands:
  One
  Two     Second command
  Three
"#
        );
    }
}
