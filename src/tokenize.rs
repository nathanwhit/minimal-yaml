#![allow(unused)]
use super::Result;
use crate::MiniYamlError;
use core::fmt;
use core::iter::{Iterator, Peekable};
use core::ops::{Add, AddAssign};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ByteIdx(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ByteLen(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Span {
    start: ByteIdx,
    end: ByteIdx,
}

impl Span {
    fn new<T: Into<ByteIdx>>(start: T, end: T) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }
}

impl ByteIdx {
    /// Returns a span from the current byte index
    /// to `other`.
    fn to(self, other: ByteIdx) -> Span {
        Span {
            start: self,
            end: other,
        }
    }
    /// Returns a span that begins at the current byte index
    /// and has a length of `len`.
    fn spans(self, len: ByteLen) -> Span {
        Span {
            start: self,
            end: self + len,
        }
    }
}

impl From<usize> for ByteIdx {
    fn from(other: usize) -> Self {
        Self(other)
    }
}

impl From<usize> for ByteLen {
    fn from(other: usize) -> Self {
        Self(other)
    }
}

impl From<ByteIdx> for usize {
    fn from(other: ByteIdx) -> Self {
        other.0
    }
}

impl From<ByteLen> for usize {
    fn from(other: ByteLen) -> Self {
        other.0
    }
}

impl Add<ByteLen> for ByteIdx {
    type Output = ByteIdx;

    fn add(self, rhs: ByteLen) -> Self::Output {
        ByteIdx(self.0 + rhs.0)
    }
}

impl Add<ByteIdx> for ByteIdx {
    type Output = ByteIdx;

    fn add(self, rhs: ByteIdx) -> Self::Output {
        ByteIdx(self.0 + rhs.0)
    }
}

impl Add<ByteIdx> for ByteLen {
    type Output = ByteIdx;

    fn add(self, rhs: ByteIdx) -> Self::Output {
        ByteIdx(self.0 + rhs.0)
    }
}

impl Add<ByteLen> for ByteLen {
    type Output = ByteLen;

    fn add(self, rhs: ByteLen) -> Self::Output {
        ByteLen(self.0 + rhs.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenKind<'a> {
    Literal(&'a str),
    Whitespace(&'a str),
    Ampersand,
    Asterisk,
    Colon,
    Comma,
    Dash,
    Dot,
    Fold,
    Newline,
    LeftBrace,
    LeftBracket,
    Pipe,
    Plus,
    RightBrace,
    RightBracket,
    SingleQuote,
    DoubleQuote,
}

impl<'a> fmt::Display for TokenKind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenKind::*;
        write!(
            f,
            "{}",
            match self {
                Literal(val) => val,
                Whitespace(space) => space,
                Ampersand => "&",
                Asterisk => "*",
                Colon => ":",
                Comma => ",",
                Dash => "-",
                Dot => ".",
                Fold => ">",
                LeftBrace => "{",
                LeftBracket => "[",
                Newline => "\n",
                Pipe => "|",
                Plus => "+",
                RightBrace => "}",
                RightBracket => "]",
                SingleQuote => r"'",
                DoubleQuote => r#"""#,
            }
        )
    }
}

impl<'a> TokenKind<'a> {}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Token<'a> {
    kind: TokenKind<'a>,
    span: Span,
}

impl<'a> Token<'a> {
    /// Constructs a new token.
    fn new(kind: TokenKind<'a>, span: Span) -> Self {
        Self { kind, span }
    }
}

use core::str::CharIndices;

struct Tokenizer<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
    tokens: Vec<Token<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn from_str(raw: &'a str) -> Self {
        Self {
            chars: raw.char_indices().peekable(),
            source: raw,
            tokens: Vec::new(),
        }
    }

    pub fn try_from_bytes(raw: &'a [u8]) -> std::result::Result<Self, std::str::Utf8Error> {
        Ok(Self::from_str(std::str::from_utf8(raw)?))
    }
}
#[derive(Debug, Clone)]
struct SourceChar {
    idx: ByteIdx,
    value: char,
}

impl SourceChar {
    /// Returns the length, in bytes, of the character.
    fn len(&self) -> ByteLen {
        ByteLen(self.value.len_utf8())
    }

    /// Returns the span of the character. The span begins at
    /// the characters byte index and ends at the byte index + byte length.
    fn span(&self) -> Span {
        Span {
            start: self.idx,
            end: self.idx + self.len(),
        }
    }

    fn end(&self) -> ByteIdx {
        self.idx + self.len()
    }
}

impl From<&(usize, char)> for SourceChar {
    fn from(other: &(usize, char)) -> Self {
        Self {
            idx: ByteIdx(other.0),
            value: other.1,
        }
    }
}

struct SourceStr<'a> {
    slice: &'a str,
    span: Span,
}

impl<'a> SourceStr<'a> {
    pub fn new(slice: &'a str, span: Span) -> Self {
        Self { slice, span }
    }
}

impl<'a> Tokenizer<'a> {
    /// Returns the next character, if one exists, without advancing
    /// the source position.
    fn peek(&mut self) -> Option<SourceChar> {
        self.chars.peek().map(SourceChar::from)
    }

    /// Returns the index of the next character, if one exists,
    /// without advancing the source position.
    fn peek_idx(&mut self) -> Option<ByteIdx> {
        self.chars.peek().map(|tup| ByteIdx(tup.0))
    }

    /// Returns the next character, if one exists, and advances
    /// the source position.
    fn next_char(&mut self) -> Option<SourceChar> {
        self.chars.next().map(|ref tup| SourceChar::from(tup))
    }

    /// Advances the source position by one character. Returns `false`
    /// if the source is exhausted, `true` otherwise.
    fn advance(&mut self) -> bool {
        self.chars.next().is_some()
    }

    fn consume_whitespace(&mut self, start: &SourceChar) -> Token<'a> {
        let SourceStr { slice, span } =
            self.consume_matches(start, |chr| chr == ' ' || chr == '\t');
        Token::new(TokenKind::Whitespace(slice), span)
    }

    fn consume_literal(&mut self, start: &SourceChar) -> Token<'a> {
        let SourceStr { slice, span } = self.consume_matches(start, |chr| {
            chr.is_alphanumeric() || chr == '.' || chr == '_'
        });
        Token::new(TokenKind::Literal(slice), span)
    }

    fn consume_matches(
        &mut self,
        start: &SourceChar,
        matches: impl Fn(char) -> bool,
    ) -> SourceStr<'a> {
        let tok_start = start.idx;
        let mut tok_end = start.end();
        loop {
            match self.peek() {
                Some(chr) if matches(chr.value) => {
                    tok_end = chr.end();
                    self.advance();
                }
                _ => {
                    return SourceStr::new(
                        &self.source[tok_start.into()..tok_end.into()],
                        tok_start.to(tok_end),
                    )
                }
            }
        }
    }

    fn consume_until(
        &mut self,
        start: &SourceChar,
        end_cond: impl Fn(char) -> bool,
    ) -> SourceStr<'a> {
        let tok_start = start.end();
        let mut tok_end = start.end();
        loop {
            match self.peek() {
                Some(chr) => {
                    tok_end = chr.end();
                    self.advance();
                    if chr.value == '\\' {
                        if let Some(next_chr) = self.peek() {
                            tok_end = next_chr.end();
                            self.advance();
                        }
                    }
                    if end_cond(chr.value) {
                        break;
                    }
                }
                _ => break,
            }
        }
        SourceStr::new(
            &self.source[tok_start.into()..tok_end.into()],
            tok_start.to(tok_end),
        )
    }

    fn chomp_whitespace(&mut self, start: &SourceChar) {
        self.consume_matches(start, |chr| chr == ' ' || chr == '\t');
    }

    /// Tokenizes the source, consuming the Tokenizer.
    /// Returns the tokenized source.
    pub fn tokenize(mut self) -> Vec<Token<'a>> {
        use TokenKind::*;
        while let Some(chr) = self.next_char() {
            match chr.value {
                '&' => self.push_tok_with(Ampersand, chr.span()),
                '*' => self.push_tok_with(Asterisk, chr.span()),
                ':' => {
                    self.push_tok_with(Colon, chr.span());
                    self.chomp_whitespace(&chr);
                }
                ',' => {
                    self.push_tok_with(Comma, chr.span());
                    self.chomp_whitespace(&chr);
                }
                '-' => {
                    self.push_tok_with(Dash, chr.span());
                    self.chomp_whitespace(&chr);
                }
                '.' => self.push_tok_with(Dot, chr.span()),
                '\"' => {
                    let lit = self.consume_until(&chr, |c| c == '\"');
                    self.push_tok_with(Literal(lit.slice), lit.span);
                }
                '>' => self.push_tok_with(Fold, chr.span()),
                '\n' | '\r' => self.push_tok_with(Newline, chr.span()),
                '{' => self.push_tok_with(LeftBrace, chr.span()),
                '[' => self.push_tok_with(LeftBracket, chr.span()),
                '|' => self.push_tok_with(Pipe, chr.span()),
                '+' => self.push_tok_with(Plus, chr.span()),
                '}' => self.push_tok_with(RightBrace, chr.span()),
                ']' => self.push_tok_with(RightBracket, chr.span()),
                '\'' => {
                    let SourceStr { slice, span } = self.consume_until(&chr, |c| c == '\'');
                    self.push_tok_with(Literal(slice), span);
                }
                '\t' | ' ' => {
                    let tok = self.consume_whitespace(&chr);
                    self.push_tok(tok);
                }
                _ => {
                    if let Token {
                        kind: Literal(slice),
                        span,
                    } = self.consume_literal(&chr)
                    {
                        self.push_tok_with(Literal(slice), span);
                    }
                }
            }
        }
        self.tokens
    }

    fn push_tok(&mut self, tok: Token<'a>) {
        self.tokens.push(tok)
    }

    /// Constructs a token and pushes it onto the token stream
    fn push_tok_with(&mut self, kind: TokenKind<'a>, span: Span) {
        self.tokens.push(Token::new(kind, span))
    }
}
