//SPDX-FileCopyrightText: 2024 Claire Bts <claxxx.bts@gmail.com>
//SPDX-License-Identifier: GPL-3.0-or-later

// clip aims to simplify writing cli and/or parser in general

//Copyright (C) 2024 Claire Bts claxxx.bts@gmail.com

//This program is free software: you can redistribute it and/or modify it under the terms of the
//GNU General Public License as published by the Free Software Foundation, either version 3 of the
//License, or (at your option) any later version.

//This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
//even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
//General Public License for more details.

//You should have received a copy of the GNU General Public License along with this program. If
//not, see <https://www.gnu.org/licenses/>.

#[derive(Debug, PartialEq)]
pub struct Value<'a> {
    pub(crate) name: &'a str,
    pub(crate) description: Option<&'a str>,
}

impl std::fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() && self.description.is_some() {
            write!(f, "{:8}{}", self.name, self.description.unwrap())
        } else {
            write!(f, "{}", self.name)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_nominale() {
        assert_eq!(
            format!(
                "{}",
                Value {
                    name: "name",
                    description: Some("description")
                }
            ),
            "name"
        );
        assert_eq!(
            format!(
                "{}",
                Value {
                    name: "name",
                    description: None
                }
            ),
            "name"
        );
    }

    #[test]
    fn display_alternate() {
        assert_eq!(
            format!(
                "{:#}",
                Value {
                    name: "name",
                    description: Some("description")
                }
            ),
            "name    description"
        );
        assert_eq!(
            format!(
                "{:#}",
                Value {
                    name: "name",
                    description: None
                }
            ),
            "name"
        );
    }
}
