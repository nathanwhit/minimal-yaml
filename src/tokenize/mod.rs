use core::fmt;
use core::iter::{Iterator, Peekable};
use core::ops::Add;
mod tests;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ByteIdx(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ByteLen(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Span {
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

    pub(crate) fn dummy() -> Self {
        Self::new(0, 0)
    }
}

impl ByteIdx {
    /// Returns a span from the current byte index
    /// to `other`.
    pub(crate) fn to(self, other: ByteIdx) -> Span {
        Span {
            start: self,
            end: other,
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
pub(crate) enum TokenKind<'a> {
    Literal(&'a str),
    Whitespace(usize),
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
    Dummy,
}

impl<'a> fmt::Display for TokenKind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenKind::*;
        write!(
            f,
            "{}",
            match self {
                Literal(val) => val,
                Whitespace(..) => "WHITESPACE",
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
                Dummy => "DUMMY",
            }
        )
    }
}

impl<'a> Default for TokenKind<'a> {
    fn default() -> Self {
        Self::Dummy
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Token<'a> {
    pub(crate) kind: TokenKind<'a>,
    pub(crate) span: Span,
}

impl<'a> Default for Token<'a> {
    fn default() -> Self {
        Self {
            kind: TokenKind::Dummy,
            span: Span::dummy(),
        }
    }
}

impl<'a> Token<'a> {
    /// Constructs a new token.
    pub(crate) fn new(kind: TokenKind<'a>, span: Span) -> Self {
        Self { kind, span }
    }

    pub(crate) fn start(&self) -> ByteIdx {
        self.span.start
    }

    pub(crate) fn end(&self) -> ByteIdx {
        self.span.end
    }
}

use core::str::CharIndices;

pub(crate) struct Tokenizer<'a> {
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

    #[allow(unused)]
    pub fn new(raw: &'a str) -> Self {
        Self::from_str(raw)
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
        Token::new(TokenKind::Whitespace(slice.len()), span)
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

    /// Tokenizes the source, consuming the Tokenizer.
    /// Returns the tokenized source.
    pub fn tokenize(mut self) -> Vec<Token<'a>> {
        use TokenKind::*;
        while let Some(chr) = self.next_char() {
            // TODO: Consider preserving comments instead of just throwing them out
            match chr.value {
                '&' => self.push_tok_with(Ampersand, chr.span()),
                '*' => self.push_tok_with(Asterisk, chr.span()),
                ':' => self.push_tok_with(Colon, chr.span()),
                ',' => self.push_tok_with(Comma, chr.span()),
                '-' => self.push_tok_with(Dash, chr.span()),
                '.' => self.push_tok_with(Dot, chr.span()),
                '\"' => self.push_tok_with(DoubleQuote, chr.span()),
                '>' => self.push_tok_with(Fold, chr.span()),
                '\n' | '\r' => self.push_tok_with(Newline, chr.span()),
                '{' => self.push_tok_with(LeftBrace, chr.span()),
                '[' => self.push_tok_with(LeftBracket, chr.span()),
                '|' => self.push_tok_with(Pipe, chr.span()),
                '+' => self.push_tok_with(Plus, chr.span()),
                '}' => self.push_tok_with(RightBrace, chr.span()),
                ']' => self.push_tok_with(RightBracket, chr.span()),
                '\'' => self.push_tok_with(SingleQuote, chr.span()),
                '\t' | ' ' => {
                    let tok = self.consume_whitespace(&chr);
                    self.push_tok(tok);
                }
                '#' => {
                    self.consume_until(&chr, |c| c == '\n');
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
