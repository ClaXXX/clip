// Copyright © 2024 Claire Bts
//
// This file is part of CLIP
//
// CLIP is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// CLIP is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    /// Try to parse an additional argument where there is no more
    TooFewArguments,
    /// could not parse a value into the expected type
    BadType,
    /// For an enumeration, Error if no value matched the input
    VariantNotFound,
}

/// Generic container. No constraint exists for this type expect for its field number.
/// It's mainly a conceptual container for parsed value associated with the iterator leftovers
#[derive(Debug, PartialEq)]
pub struct Parsed<T, I>(pub T, pub I);

/// Simple and safe type conversions that may fail in a controlled way under some circumstances.
/// It takes an iterator and return what's left once all values have been parsed
/// It's very similar to and inspired by TryFrom from the std::convert library. It just is adapted to
/// browse an iterator
pub trait TryParse<I>: Sized {
    type Error;

    /// Required method
    fn try_parse(value: I) -> Result<Parsed<Self, I>, Self::Error>;
}
