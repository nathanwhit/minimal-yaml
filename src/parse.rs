use crate::tokenize::{Token, TokenKind};
use crate::{Entry, Yaml, YamlParseError};
use core::iter::{Enumerate, Iterator, Peekable};
use core::slice::Iter;

use crate::Result;

// Implementation lifted from std, as it's currently only on Nightly. It's such a simple macro that it's low risk to duplicate it here (and better than writing one myself)
macro_rules! matches {
    ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )?) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => true,
            _ => false
        }
    }
}

pub(crate) struct Parser<'a, 'b> {
    token: &'b Token<'a>,
    stream: Peekable<Enumerate<Iter<'b, Token<'a>>>>,
    tok_stream: &'b [Token<'a>],
    source: &'a str,
    tok_idx: usize,
    indent: usize,
    expected: Vec<TokenKind<'a>>,
}

impl<'a, 'b> Parser<'a, 'b> {
    pub(crate) fn new(source: &'a str, tok_stream: &'b [Token<'a>]) -> Result<Self> {
        let mut stream = tok_stream.iter().enumerate().peekable();
        let first = stream.next().ok_or_else(|| YamlParseError {
            line: 0,
            col: 0,
            msg: Some("expected input".into()),
            source: None,
        })?;
        Ok(Self {
            token: &first.1,
            stream,
            tok_stream,
            source,
            tok_idx: first.0,
            indent: 0,
            expected: Vec::new(),
        })
    }

    fn bump(&mut self) -> bool {
        match self.stream.next() {
            Some(tok) => {
                self.tok_idx = tok.0;
                self.token = tok.1;
                true
            }
            None => false,
        }
    }

    fn advance(&mut self) -> Result<()> {
        if self.bump() {
            Ok(())
        } else {
            self.parse_error_with_msg("unexpected end of input")
        }
    }

    fn peek(&mut self) -> Option<&Token<'a>> {
        self.stream.peek().map(|t| t.1)
    }

    fn at_end(&self) -> bool {
        self.tok_idx == self.tok_stream.len() - 1
    }

    fn parse_mapping_maybe(&mut self, node: Yaml<'a>) -> Result<Yaml<'a>> {
        use TokenKind::*;
        self.chomp_whitespace();
        match self.token.kind {
            Colon
                if match self.expected.last() {
                    Some(RightBrace) | Some(Colon) => false,
                    _ => true,
                } =>
            {
                self.parse_mapping_block(node)
            }
            _ => Ok(node),
        }
    }

    pub(crate) fn parse(&mut self) -> Result<Yaml<'a>> {
        use TokenKind::*;
        let res = match self.token.kind {
            DoubleQuote | SingleQuote | Literal(..) => {
                let node = self.parse_scalar()?;
                self.parse_mapping_maybe(node)?
            }
            LeftBrace => {
                self.expected.push(RightBrace);
                let res = self.parse_mapping_flow()?;
                if let Some(RightBrace) = self.expected.last() {
                    self.pop_if_match(&RightBrace)?;
                }
                self.parse_mapping_maybe(res)?
            }
            LeftBracket => {
                let node = self.parse_sequence_flow()?;
                self.parse_mapping_maybe(node)?
            }
            Dash => match self.peek() {
                Some(Token { kind: Dash, .. }) => {
                    if self.check_ahead_n(2, |tk| matches!(tk, Dash)) {
                        self.bump();
                        self.bump();
                        self.bump();
                        self.parse()?
                    } else {
                        // TODO: Provide error message
                        return self.parse_error();
                    }
                }
                _ => self.parse_sequence_block()?,
            },
            RightBrace | RightBracket => {
                return self
                    .parse_error_with_msg(format!(r#"unexpected symbol '{}'"#, self.token.kind))
            }
            Whitespace(amt) => {
                self.indent = amt;
                self.advance()?;
                self.parse()?
            }
            Newline => {
                self.indent = 0;
                self.advance()?;
                self.parse()?
            }
            // TODO: Provide error message
            _ => return self.parse_error(),
        };
        Ok(res)
    }

    pub(crate) fn parse_scalar(&mut self) -> Result<Yaml<'a>> {
        use TakeUntilCond::*;
        use TokenKind::*;
        match self.token.kind {
            // TODO: currently qouble quote/single quote scalars are handled identically. maybe handle as defined
            // by the YAML spec?
            DoubleQuote => {
                let scal_start = self.tok_idx;
                self.advance()?;
                self.take_until(MatchOrErr, |tok, _| matches!(tok, DoubleQuote))?;
                let scal_end = if self.bump() {
                    self.tok_idx
                } else {
                    self.tok_stream.len()
                };
                let entire_literal = self.slice_tok_range((scal_start, scal_end));
                Ok(Yaml::Scalar(entire_literal))
            }
            SingleQuote => {
                let scal_start = self.tok_idx;
                self.advance()?;
                self.take_until(MatchOrErr, |tok, _| matches!(tok, SingleQuote))?;
                let scal_end = if self.bump() {
                    self.tok_idx
                } else {
                    self.tok_stream.len()
                };
                let entire_literal = self.slice_tok_range((scal_start, scal_end));
                Ok(Yaml::Scalar(entire_literal))
            }
            Literal(..) => {
                let stop = |tok: &TokenKind<'_>| {
                    matches!(tok, Comma | Colon | RightBrace | RightBracket | Newline)
                };
                let scal_range = self.take_until(MatchOrEnd, |tok, nxt| {
                    stop(tok) || (matches!(tok, Whitespace(..)) && stop(nxt))
                })?;
                let entire_literal = self.slice_tok_range(scal_range);
                Ok(Yaml::Scalar(entire_literal))
            }
            // TODO: Provide error message
            _ => self.parse_error(),
        }
    }

    fn lookup_line_col(&self) -> (usize, usize) {
        let err_off: usize = usize::from(self.token.start()) + 1;
        let mut off = 0;
        let mut line_len = 0;
        let mut chars = self.source.chars().map(|c| (c, c.len_utf8()));
        let mut line_lens = Vec::new();
        while let Some((chr, len)) = chars.next() {
            match chr {
                '\r' => {
                    if let Some(('\n', nxtlen)) = chars.next() {
                        line_lens.push(line_len + nxtlen + len);
                        line_len = 0;
                        continue;
                    }
                }
                '\n' => {
                    line_lens.push(line_len + len);
                    line_len = 0;
                    continue;
                }
                _ => line_len += len,
            }
        }
        let mut line_num = 0;
        for ((line_no, _), len) in self.source.lines().enumerate().zip(line_lens) {
            if err_off >= off && err_off < off + len {
                return (line_no + 1, err_off - off + 1);
            }
            line_num = line_no;
            off += len;
        }
        if err_off >= off {
            return (line_num + 1, err_off - off + 1);
        }
        eprintln!("Couldn't find error location, please report this bug");
        (0, 0)
    }

    fn parse_error<T>(&self) -> Result<T> {
        let (line, col) = self.lookup_line_col();
        Err(YamlParseError {
            line,
            col,
            msg: None,
            source: None,
        })
    }

    fn parse_error_with_msg<T, S: Into<String>>(&self, msg: S) -> Result<T> {
        let (line, col) = self.lookup_line_col();
        Err(YamlParseError {
            line,
            col,
            msg: Some(msg.into()),
            source: None,
        })
    }

    pub(crate) fn parse_mapping_flow(&mut self) -> Result<Yaml<'a>> {
        use TokenKind::*;
        match self.token.kind {
            LeftBrace => (),
            _ => return self.parse_error_with_msg("expected left brace"),
        }
        self.advance()?;
        let mut entries: Vec<Entry<'a>> = Vec::new();
        loop {
            match self.token.kind {
                RightBrace => {
                    self.bump();
                    return Ok(Yaml::Mapping(entries));
                }
                Comma => {
                    self.advance()?;
                }
                _ => {
                    self.expected.push(Colon);
                    let key = self.parse()?;
                    self.chomp_whitespace();
                    match self.token.kind {
                        Colon => {
                            self.pop_if_match(&Colon)?;
                            self.advance()?;
                            self.chomp_whitespace();
                            let value = self.parse()?;
                            self.chomp_whitespace();
                            entries.push(Entry { key, value })
                        }
                        // TODO: Provide error message
                        _ => return self.parse_error(),
                    }
                }
            }
        }
    }

    pub(crate) fn parse_mapping_block(&mut self, start_key: Yaml<'a>) -> Result<Yaml<'a>> {
        use TokenKind::*;
        let indent = self.indent;
        match self.token.kind {
            Colon => {
                self.advance()?;
                let mut entries = Vec::new();
                self.chomp_whitespace();
                let value = self.parse()?;
                entries.push(Entry::new(start_key, value));
                loop {
                    match self.token.kind {
                        Newline => {
                            self.indent = 0;
                            if self.bump() {
                                continue;
                            } else {
                                break;
                            }
                        }
                        Whitespace(idt) => {
                            self.indent = idt;
                            if !self.bump() {
                                break;
                            }
                        }
                        _ if self.indent < indent || self.at_end() => break,
                        _ => {
                            self.expected.push(Colon);
                            let key = self.parse()?;
                            self.chomp_whitespace();
                            if let Colon = self.token.kind {
                                self.pop_if_match(&Colon)?;
                                self.advance()?;
                                self.chomp_whitespace();
                                let value = self.parse()?;
                                entries.push(Entry::new(key, value));
                            } else {
                                // TODO: Provide error message
                                return self.parse_error();
                            }
                        }
                    }
                }
                Ok(Yaml::Mapping(entries))
            }
            // TODO: Provide error message
            _ => self.parse_error(),
        }
    }

    fn slice_tok_range(&self, range: (usize, usize)) -> &'a str {
        let start = self.tok_stream[range.0].start();
        let end = match self.tok_stream.get(range.1) {
            Some(tok) => tok.start(),
            None => self.tok_stream.last().unwrap().end(),
        };
        &self.source[start.into()..end.into()]
    }

    fn chomp_whitespace(&mut self) {
        while let TokenKind::Whitespace(..) = self.token.kind {
            if !self.bump() {
                break;
            }
        }
    }

    pub(crate) fn parse_sequence_flow(&mut self) -> Result<Yaml<'a>> {
        use TokenKind::*;
        match self.token.kind {
            LeftBracket => {
                self.advance()?;
                let mut elements = Vec::new();
                loop {
                    match self.token.kind {
                        RightBracket => {
                            self.bump();
                            return Ok(Yaml::Sequence(elements));
                        }
                        Whitespace(..) => {
                            self.bump();
                        }
                        _ => {
                            let elem = self.parse()?;
                            elements.push(elem);
                            self.chomp_whitespace();
                            match self.token.kind {
                                Comma => {
                                    self.advance()?;
                                    continue;
                                }
                                RightBracket => {
                                    self.bump();
                                    return Ok(Yaml::Sequence(elements));
                                }
                                // TODO: Provide error message
                                _ => return self.parse_error(),
                            }
                        }
                    }
                }
            }
            // TODO: Provide error message
            _ => self.parse_error(),
        }
    }

    fn check_ahead_1(&mut self, stop: impl Fn(&TokenKind<'a>) -> bool) -> bool {
        match self.peek() {
            Some(tok) => stop(&tok.kind),
            None => false,
        }
    }

    pub(crate) fn parse_sequence_block(&mut self) -> Result<Yaml<'a>> {
        use TokenKind::*;
        let indent = self.indent;
        match self.token.kind {
            Dash => {
                let mut seq = Vec::new();
                loop {
                    match self.token.kind {
                        Newline => {
                            self.indent = 0;
                            if self.bump() {
                                continue;
                            } else {
                                break;
                            }
                        }
                        Whitespace(idt) => {
                            self.indent = idt;
                            if !self.bump() {
                                break;
                            }
                        }
                        _ if self.indent < indent || self.at_end() => break,
                        Dash => {
                            if self.check_ahead_1(|t| matches!(t, Newline)) {
                                self.advance()?;
                                self.advance()?;
                                self.indent = 0;
                                if let Whitespace(idt) = self.token.kind {
                                    if idt < indent {
                                        break;
                                    } else {
                                        let node = self.parse()?;
                                        seq.push(node);
                                    }
                                } else if 0 < indent {
                                    break;
                                } else {
                                    let node = self.parse()?;
                                    seq.push(node);
                                }
                            } else {
                                self.advance()?;
                                let node = self.parse()?;
                                seq.push(node);
                            }
                        }
                        _ => return self.parse_error(),
                    }
                }
                Ok(Yaml::Sequence(seq))
            }
            // TODO: Provide error message
            _ => self.parse_error(),
        }
    }

    fn check_ahead_n(&self, n: usize, stop: impl Fn(&TokenKind<'a>) -> bool) -> bool {
        match self.tok_stream.get(self.tok_idx + n) {
            Some(Token { kind: tok_kind, .. }) => stop(tok_kind),
            None => false,
        }
    }

    fn peekahead_n(&self, n: usize) -> Option<&TokenKind<'a>> {
        match self.tok_stream.get(self.tok_idx + n) {
            Some(Token { kind: tok_kind, .. }) => Some(tok_kind),
            None => None,
        }
    }

    fn take_until(
        &mut self,
        cond: TakeUntilCond,
        stop: impl Fn(&TokenKind<'a>, &TokenKind<'a>) -> bool,
    ) -> Result<(usize, usize)> {
        let start = self.tok_idx;
        let mut end = start;
        loop {
            let peeked = self.peekahead_n(1);
            if match peeked {
                Some(tok_kind) => stop(&self.token.kind, tok_kind),
                None => stop(&self.token.kind, &TokenKind::default()),
            } {
                break;
            } else if !self.bump() {
                return match cond {
                    TakeUntilCond::MatchOrEnd => Ok((start, self.tok_stream.len())),
                    // TODO: Provide error message
                    TakeUntilCond::MatchOrErr => self.parse_error(),
                };
            }
            end += 1;
        }
        Ok((start, end))
    }

    fn pop_if_match(&mut self, expect: &TokenKind<'a>) -> Result<()> {
        match self.expected.last() {
            Some(tk) if tk == expect => {
                self.expected.pop();
                Ok(())
            }
            // TODO: Provide error message
            _ => self.parse_error(),
        }
    }
}
#[derive(Clone, Copy)]
enum TakeUntilCond {
    MatchOrEnd,
    MatchOrErr,
}
