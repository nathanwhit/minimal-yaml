#![warn(clippy::all, clippy::pedantic)]
pub mod errors;
mod parse;
mod tests;
mod tokenize;

pub use crate::errors::MiniYamlError;

pub(crate) type Result<T> = std::result::Result<T, MiniYamlError>;

use parse::Parser;
use tokenize::Tokenizer;

use std::{fmt, fmt::Display};

#[derive(Clone, Debug, PartialEq, Eq)]
/// A Yaml Element
pub enum Yaml<'a> {
    /// A literal value, losslessly interpreted as a string
    Scalar(&'a str),

    /// A sequence of values in flow style
    /// `[x, y, z]`
    /// or in block style
    /// ```yaml
    ///     - x
    ///     - y
    ///     - z
    /// ```
    Sequence(Vec<Yaml<'a>>),

    /// A mapping from key to value in flow style
    /// `{x: X, y: Y, z: Z}`
    /// or in block style
    /// ```yaml
    ///     x: X
    ///     y: Y
    ///     z: Z
    /// ```
    Mapping(Vec<Entry<'a>>),
}

impl<'a> Display for Yaml<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Yaml::Scalar(slice) => {
                let escaped: String = slice.escape_debug().collect();
                write!(f, "\"{}\"", escaped)
            }
            Yaml::Sequence(seq) => {
                write!(f, "[ ")?;
                let last_idx = seq.len() - 1;
                for (idx, elem) in seq.iter().enumerate() {
                    if idx == last_idx {
                        write!(f, "{}", elem)?;
                    } else {
                        write!(f, "{}, ", elem)?;
                    }
                }
                write!(f, " ]")
            }
            Yaml::Mapping(map) => {
                write!(f, "{{")?;
                let last_idx = map.len() - 1;
                for (idx, entry) in map.iter().enumerate() {
                    if idx == last_idx {
                        write!(f, "{}", entry)?;
                    } else {
                        write!(f, "{}, ", entry)?;
                    }
                }
                write!(f, " }}")
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A Yaml map entry
pub struct Entry<'a> {
    /// The key associated with the entry
    pub key: Yaml<'a>,
    /// The value which the key maps to
    pub value: Yaml<'a>,
}

impl<'a> Entry<'a> {
    pub fn new(key: Yaml<'a>, value: Yaml<'a>) -> Self {
        Self { key, value }
    }
}

impl<'a> Display for Entry<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} : {}", self.key, self.value)
    }
}

/// Parse Yaml input. Returns the top level Yaml element on success,
/// or a ```MiniYamlError``` on failure
pub fn parse<'a>(input: &'a str) -> Result<Yaml<'a>> {
    let tokenizer = Tokenizer::from_str(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(input, &tokens);
    parser.parse()
}
