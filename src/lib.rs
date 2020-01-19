pub mod errors;
mod parse;
mod tests;
mod tokenize;

pub use crate::errors::MiniYamlError;

pub(crate) type Result<T> = std::result::Result<T, MiniYamlError>;

use parse::Parser;
use tokenize::Tokenizer;
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
#[derive(Clone, Debug, PartialEq, Eq)]
/// A Yaml map entry
pub struct Entry<'a> {
    /// The key associated with the entry
    key: Yaml<'a>,
    /// The value which the key maps to
    value: Yaml<'a>,
}

/// Parse Yaml input. Returns the top level Yaml element on success,
/// or a ```MiniYamlError``` on failure
pub fn parse<'a>(input: &'a str) -> Result<Yaml<'a>> {
    let tokenizer = Tokenizer::from_str(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(input, &tokens);
    parser.parse()
}
