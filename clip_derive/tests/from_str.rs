// Copyright © 2024 Claire Bts
//
// This file is part of CLIP
//
// CLIP is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// 
// CLIP is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

use clip_derive::*;
#[derive(Debug, PartialEq, FromStr)]
enum Unit {
    One,
    Two,
    Three,
}

#[test]
fn it_should_parse_the_enumeration_unit_value() {
    assert_eq!("One".parse::<Unit>(), Ok(Unit::One));
    assert_eq!("two".parse::<Unit>(), Ok(Unit::Two));
    assert_eq!("THREE".parse::<Unit>(), Ok(Unit::Three));
}

#[test]
fn it_should_raise_variant_not_found() {
    assert_eq!(
        "unexistant".parse::<Unit>(),
        Err(String::from("Unexistant variant unexistant"))
    );
    assert_eq!("".parse::<Unit>(), Err(String::from("Unexistant variant ")));
}

