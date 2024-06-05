//SPDX-FileCopyrightText: 2024 Claire Bts <claxxx.bts@gmail.com>
//SPDX-License-Identifier: GPL-3.0-or-later

// clip_derive aims to simplify writing cli and/or parser in general

//Copyright (C) 2024 Claire Bts claxxx.bts@gmail.com

//Clipv is free software: you can redistribute it and/or modify it under the terms of the
//GNU General Public License as published by the Free Software Foundation, either version 3 of the
//License, or (at your option) any later version.

//Clipv is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
//even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
//General Public License for more details.

//You should have received a copy of the GNU General Public License along with Clipv. If
//not, see <https://www.gnu.org/licenses/>.

#[derive(Default)]
pub struct Formatter<'a> {
    pub very_start: Option<&'a str>,
    pub very_end: Option<&'a str>,
    pub start: Option<&'a str>,
    pub end: Option<&'a str>,
    pub middle: Option<&'a str>,
}

impl<'a> Formatter<'a> {
    pub fn fmt<'b, Item: 'b, I: Iterator<Item = &'b Item>, F: FnMut(I::Item) -> Option<String>>(
        &self,
        args: I,
        format_function: F,
    ) -> String {
        format!(
            "{very_start}{description}{very_end}",
            very_start = self.very_start.unwrap_or(""),
            very_end = self.very_end.unwrap_or(""),
            description = args.filter_map(format_function).fold(
                "".to_string(),
                |string: String, item: String| {
                    format!(
                        "{string}{middle}{start}{item}{end}",
                        start = self.start.unwrap_or(""),
                        middle = if string.len() == 0 {
                            ""
                        } else {
                            self.middle.unwrap_or("")
                        },
                        end = self.end.unwrap_or("")
                    )
                }
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_formatter() {
        assert_eq!(
            Formatter::default().fmt([1, 2, 3].iter(), |item| Some(item.to_string())),
            String::from("123")
        );
    }

    #[test]
    fn it_should_fmt_with_start_and_end() {
        assert_eq!(
            Formatter {
                start: Some("<"),
                end: Some(">"),
                ..Default::default()
            }
            .fmt([1, 2, 3].iter(), |item| Some(item.to_string())),
            "<1><2><3>"
        );
    }

    #[test]
    fn it_should_fmt_with_middle_char() {
        assert_eq!(
            Formatter {
                middle: Some(" "),
                ..Default::default()
            }
            .fmt([1, 2, 3].iter(), |item| Some(item.to_string())),
            "1 2 3"
        );
    }

    #[test]
    fn it_should_fmt_with_start_end_middle_char() {
        assert_eq!(
            Formatter {
                start: Some("<"),
                end: Some(">"),
                middle: Some(" "),
                very_start: Some("Result: "),
                very_end: Some(".")
            }
            .fmt([1, 2, 3].iter(), |item| Some(item.to_string())),
            "Result: <1> <2> <3>."
        );
    }

    #[test]
    fn it_should_fmt_and_filter_None_values() {
        assert_eq!(
            Formatter {
                start: Some("<"),
                end: Some(">"),
                middle: Some(","),
                ..Default::default()
            }
            .fmt([1, 2, 3, 4, 5, 4, 3, 5].iter(), |item| {
                if item % 2 == 0 {
                    Some(item.to_string())
                } else {
                    None
                }
            }),
            "<2>,<4>,<4>"
        );
        assert_eq!(
            Formatter {
                start: Some("<"),
                end: Some(">"),
                middle: Some(","),
                ..Default::default()
            }
            .fmt([1, 2, 3, 4, 5, 4, 3, 5].iter(), |item| {
                if item % 2 != 0 {
                    Some(item.to_string())
                } else {
                    None
                }
            }),
            "<1>,<3>,<5>,<3>,<5>"
        );
    }
}
