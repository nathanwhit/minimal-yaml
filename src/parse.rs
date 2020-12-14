use crate::bytes::ByteExt;
use crate::{Entry, Yaml, YamlParseError};
use core::iter::{Iterator, Peekable};
use std::str::Bytes;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParseContext {
    FlowIn,
    FlowOut,
    FlowKey,
    BlockIn,
    BlockOut,
    BlockKey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseContextKind {
    FlowMapping,
    Flow,
    BlockMapping,
    Block,
}

pub(crate) struct Parser<'a> {
    current: u8,
    stream: Peekable<Bytes<'a>>,
    bytes: &'a [u8],
    source: &'a str,
    idx: usize,
    indent: usize,
    expected: Vec<u8>,
    contexts: Vec<ParseContext>,
}

impl<'a, 'b> Parser<'a> {
    pub(crate) fn new(source: &'a str) -> Result<Self> {
        let mut stream = source.bytes().peekable();
        let first = stream.next().ok_or_else(|| YamlParseError {
            line: 0,
            col: 0,
            msg: Some("expected input".into()),
            source: None,
        })?;
        Ok(Self {
            current: first,
            bytes: source.as_bytes(),
            stream,
            source,
            idx: 0,
            indent: 0,
            expected: Vec::new(),
            contexts: Vec::new(),
        })
    }

    fn start_context(&mut self, context_kind: ParseContextKind) -> Result<()> {
        let context = match self.context() {
            Some(ctx) => match context_kind {
                ParseContextKind::Flow => match ctx {
                    ParseContext::FlowKey | ParseContext::FlowIn | ParseContext::FlowOut => {
                        ParseContext::FlowIn
                    }
                    ParseContext::BlockKey | ParseContext::BlockIn | ParseContext::BlockOut => {
                        ParseContext::FlowIn
                    }
                },
                ParseContextKind::FlowMapping => ParseContext::FlowKey,
                ParseContextKind::Block => match ctx {
                    ParseContext::FlowIn | ParseContext::FlowOut | ParseContext::FlowKey => {
                        return self.parse_error_with_msg(
                            "block collections cannot be values in flow collections",
                        )
                    }
                    ParseContext::BlockIn | ParseContext::BlockOut | ParseContext::BlockKey => {
                        ParseContext::BlockIn
                    }
                },
                ParseContextKind::BlockMapping => ParseContext::BlockKey,
            },
            None => match context_kind {
                ParseContextKind::Flow => ParseContext::FlowIn,
                ParseContextKind::FlowMapping => ParseContext::FlowKey,
                ParseContextKind::Block => ParseContext::BlockOut,
                ParseContextKind::BlockMapping => ParseContext::BlockKey,
            },
        };
        self.contexts.push(context);
        Ok(())
    }

    fn end_context(&mut self, expect: ParseContextKind) -> Result<()> {
        if let Some(actual) = self.contexts.pop() {
            let ctx_matches = match expect {
                ParseContextKind::Flow => {
                    matches!(actual, ParseContext::FlowIn | ParseContext::FlowOut)
                }
                ParseContextKind::FlowMapping => matches!(actual, ParseContext::FlowKey),
                ParseContextKind::Block => {
                    matches!(actual, ParseContext::BlockIn | ParseContext::BlockOut)
                }
                ParseContextKind::BlockMapping => matches!(actual, ParseContext::BlockKey),
            };
            if ctx_matches {
                Ok(())
            } else {
                self.parse_error_with_msg(format!(
                    "expected but failed to end context {:?}, instead found {:?}",
                    expect, actual
                ))
            }
        } else {
            self.parse_error_with_msg(format!(
                "expected context {:?} but no contexts remained",
                expect
            ))
        }
    }

    fn context(&self) -> Option<ParseContext> {
        self.contexts.last().map(|&c| c)
    }

    fn bump(&mut self) -> bool {
        match self.stream.next() {
            Some(byte) => {
                self.idx += 1;
                self.current = byte;
                true
            }
            None => false,
        }
    }

    fn bump_newline(&mut self) -> bool {
        match self.stream.next() {
            Some(b'\n') | Some(b'\r') => self.bump(),
            Some(byte) => {
                self.idx += 1;
                self.current = byte;
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

    fn peek(&mut self) -> Option<u8> {
        self.stream.peek().copied()
    }

    fn at_end(&self) -> bool {
        self.idx == self.bytes.len() - 1
    }

    fn parse_mapping_maybe(&mut self, node: Yaml<'a>) -> Result<Yaml<'a>> {
        self.chomp_whitespace();
        self.chomp_comment();
        match self.current {
            b':' if match self.expected.last() {
                Some(b'}') | Some(b':') => false,
                _ => true,
            } =>
            {
                self.parse_mapping_block(node)
            }
            _ => Ok(node),
        }
    }

    pub(crate) fn parse(&mut self) -> Result<Yaml<'a>> {
        let context = self.context();
        let peeked = self.peek();
        dbg!(char::from(self.current));
        let res = match self.current {
            b'#' => {
                self.chomp_comment();
                self.parse()?
            }
            b'-' if self.check_ahead_1(|val| val == b'-')
                && self.check_ahead_n(2, |val| val == b'-') =>
            {
                self.bump();
                self.bump();
                self.bump();
                self.parse()?
            }
            b'\n' | b'\r' => {
                self.chomp_newlines()?;
                self.indent = 0;
                self.parse()?
            }
            byt if byt.is_scalar_start(peeked, context) => self.parse_maybe_scalar()?,
            b'{' => {
                self.expected.push(b'}');
                let res = self.parse_mapping_flow()?;
                if let Some(b'}') = self.expected.last() {
                    self.pop_if_match(b'}')?;
                }
                self.parse_mapping_maybe(res)?
            }
            b'[' => {
                let node = self.parse_sequence_flow()?;
                self.parse_mapping_maybe(node)?
            }
            b'-' => match context {
                _ => match self.peek() {
                    Some(byt) if byt.is_linebreak() || byt.is_ws() => {
                        self.parse_sequence_block()?
                    }
                    byt => unreachable!(format!("unexpected {:?}", byt.map(char::from))),
                },
            },
            b'}' | b']' => {
                return self.parse_error_with_msg(format!(
                    r#"unexpected symbol '{}'"#,
                    char::from(self.current)
                ))
            }
            b if b.is_ws() => {
                self.chomp_indent();
                if self.at_end() {
                    return self.parse_error_with_msg("unexpected end of input");
                }
                self.parse()?
            }
            // TODO: Provide error message
            _ => return self.parse_error_with_msg("failed to parse at top level"),
        };
        Ok(res)
    }
    pub(crate) fn parse_maybe_scalar(&mut self) -> Result<Yaml<'a>> {
        match self.context() {
            None => {
                self.start_context(ParseContextKind::BlockMapping)?;
                let node = self.parse_scalar()?;
                self.end_context(ParseContextKind::BlockMapping)?;
                self.parse_mapping_maybe(node)
            }
            Some(ctx) => match ctx {
                ParseContext::FlowIn | ParseContext::FlowOut | ParseContext::FlowKey => {
                    self.parse_scalar()
                }
                _ => {
                    self.start_context(ParseContextKind::BlockMapping)?;
                    let node = self.parse_scalar()?;
                    self.end_context(ParseContextKind::BlockMapping)?;
                    self.parse_mapping_maybe(node)
                }
            },
        }
    }

    pub(crate) fn parse_scalar(&mut self) -> Result<Yaml<'a>> {
        let context = self.context();
        match self.current {
            // TODO: currently qouble quote/single quote scalars are handled identically. maybe handle as defined
            // by the YAML spec?
            b'\"' => {
                let scal_start = self.idx;
                self.advance()?;
                let _ = self
                    .take_while(|tok, _| !matches!(tok, b'\"'))
                    .map_err(|_| {
                        self.make_parse_error_with_msg("unexpected end of input; expected '\"'")
                    })?;
                let scal_end = if self.bump() {
                    self.idx
                } else {
                    self.bytes.len()
                };
                let entire_literal = self.slice_range((scal_start, scal_end));

                Ok(Yaml::Scalar(entire_literal))
            }
            b'\'' => {
                let scal_start = self.idx;
                self.advance()?;
                self.take_while(|tok, _| !matches!(tok, b'\''))
                    .map_err(|_| {
                        self.make_parse_error_with_msg("unexpected end of input; expected '\''")
                    })?;
                let scal_end = if self.bump() {
                    self.idx
                } else {
                    self.bytes.len()
                };
                let entire_literal = self.slice_range((scal_start, scal_end));
                Ok(Yaml::Scalar(entire_literal))
            }
            _ => {
                let accept = |tok: u8, nxt: Option<u8>| tok.is_ns_plain(nxt, context);
                let (start, mut end) = self.take_while(&accept).unwrap_or_else(|val| val);
                loop {
                    self.chomp_whitespace();
                    self.chomp_comment();
                    let (s, e) = self.take_while(&accept).unwrap_or_else(|val| val);
                    if s == e {
                        break;
                    } else {
                        end = e;
                    }
                    if self.at_end() {
                        break;
                    }
                }
                let entire_literal = self.slice_range((start, end));
                Ok(Yaml::Scalar(entire_literal))
            }
        }
    }

    fn lookup_line_col(&self) -> (usize, usize) {
        let err_off: usize = usize::from(self.idx) + 1;
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

    #[allow(unused)]
    fn parse_error<T>(&self) -> Result<T> {
        let (line, col) = self.lookup_line_col();
        Err(YamlParseError {
            line,
            col,
            msg: Some(format!(
                r#"unexpectedly found "{}" while parsing"#,
                self.current
            )),
            source: None,
        })
    }

    fn make_parse_error_with_msg<S: Into<String>>(&self, msg: S) -> YamlParseError {
        let (line, col) = self.lookup_line_col();
        YamlParseError {
            line,
            col,
            msg: Some(msg.into()),
            source: None,
        }
    }

    fn parse_error_with_msg<T, S: Into<String>>(&self, msg: S) -> Result<T> {
        Err(self.make_parse_error_with_msg(msg))
    }

    pub(crate) fn parse_mapping_flow(&mut self) -> Result<Yaml<'a>> {
        match self.current {
            b'{' => (),
            _ => return self.parse_error_with_msg("expected left brace"),
        }
        self.advance()?;
        let mut entries: Vec<Entry<'a>> = Vec::new();
        loop {
            match &self.current {
                b'}' => {
                    self.bump();
                    return Ok(Yaml::Mapping(entries));
                }
                b',' => {
                    self.advance()?;
                }
                _ => {
                    self.expected.push(b':');
                    self.start_context(ParseContextKind::FlowMapping)?;
                    let key = self.parse()?;
                    self.end_context(ParseContextKind::FlowMapping)?;
                    self.chomp_whitespace();
                    self.chomp_comment();
                    match self.current {
                        b':' => {
                            self.pop_if_match(b':')?;
                            self.advance()?;
                            self.chomp_whitespace();
                            self.start_context(ParseContextKind::Flow)?;
                            let value = self.parse()?;
                            self.end_context(ParseContextKind::Flow)?;
                            self.chomp_whitespace();
                            self.chomp_comment();
                            entries.push(Entry { key, value })
                        }
                        // TODO: Provide error message
                        _ => return self.parse_error_with_msg("failed to parse flow mapping"),
                    }
                }
            }
        }
    }

    pub(crate) fn parse_mapping_block(&mut self, start_key: Yaml<'a>) -> Result<Yaml<'a>> {
        match self.context() {
            Some(ParseContext::FlowIn)
            | Some(ParseContext::FlowKey)
            | Some(ParseContext::FlowOut) => {
                return self
                    .parse_error_with_msg("block mappings may not appear in flow collections")
            }
            _ => {}
        }
        let indent = self.indent;
        match self.current {
            b':' => {
                self.advance()?;
                let mut entries = Vec::new();
                self.chomp_whitespace();
                self.chomp_comment();
                let value = self.parse()?;
                dbg!(&value);
                entries.push(Entry::new(start_key, value));
                loop {
                    match self.current {
                        _ if self.at_end() => break,
                        byt if byt.is_linebreak() => {
                            self.indent = 0;
                            if self.bump_newline() {
                                continue;
                            } else {
                                break;
                            }
                        }
                        byt if byt.is_ws() => {
                            self.chomp_indent();
                        }
                        b'#' => self.chomp_comment(),
                        _ if self.indent < indent => break,
                        _ => {
                            self.expected.push(b':');
                            let key = self.parse()?;
                            self.chomp_whitespace();
                            self.chomp_comment();
                            if let b':' = self.current {
                                self.pop_if_match(b':')?;
                                self.advance()?;
                                self.chomp_whitespace();
                                let value = self.parse()?;
                                entries.push(Entry::new(key, value));
                            } else {
                                // TODO: Provide error message
                                return self.parse_error_with_msg("failed to parse block mapping");
                            }
                        }
                    }
                }
                Ok(Yaml::Mapping(entries))
            }
            // TODO: Provide error message
            _ => self.parse_error_with_msg("failed to parse block mapping, expected ':'"),
        }
    }

    fn slice_range(&self, (start, end): (usize, usize)) -> &'a str {
        let end = usize::min(end, self.bytes.len());
        &self.source[start.into()..end.into()]
    }

    fn chomp_comment(&mut self) {
        if self.current == b'#' {
            self.bump();
            while !self.current.is_linebreak() {
                if !self.bump() {
                    break;
                }
            }
            self.bump_newline();
        }
    }

    fn chomp_whitespace(&mut self) {
        while let b' ' | b'\t' = self.current {
            if !self.bump() {
                break;
            }
        }
    }

    fn chomp_indent(&mut self) {
        let mut idt = 0;
        while let b' ' | b'\t' = self.current {
            if !self.bump() {
                break;
            }
            idt += 1;
        }
        self.indent = idt;
    }

    fn chomp_newlines(&mut self) -> Result<()> {
        while let b'\r' | b'\n' = self.current {
            self.advance()?;
        }
        Ok(())
    }

    pub(crate) fn parse_sequence_flow(&mut self) -> Result<Yaml<'a>> {
        self.start_context(ParseContextKind::Flow)?;
        match self.current {
            b'[' => {
                self.advance()?;
                let mut elements = Vec::new();
                loop {
                    match self.current {
                        b']' => {
                            self.bump();
                            self.end_context(ParseContextKind::Flow)?;
                            return Ok(Yaml::Sequence(elements));
                        }
                        b' ' | b'\t' => self.chomp_whitespace(),

                        b'#' => self.chomp_comment(),
                        _ => {
                            let elem = self.parse()?;
                            elements.push(elem);
                            self.chomp_whitespace();

                            match self.current {
                                b',' => {
                                    self.advance()?;
                                }
                                b'#' => self.chomp_comment(),
                                b']' => {
                                    self.bump();
                                    self.end_context(ParseContextKind::Flow)?;
                                    return Ok(Yaml::Sequence(elements));
                                }
                                // TODO: Provide error message
                                _ => {
                                    return self
                                        .parse_error_with_msg("failed to parse flow sequence")
                                }
                            }
                        }
                    }
                }
            }
            // TODO: Provide error message
            _ => self.parse_error_with_msg("failed to parse flow sequence"),
        }
    }

    fn check_ahead_1(&self, stop: impl Fn(u8) -> bool) -> bool {
        match self.bytes.get(self.idx + 1) {
            Some(&b) => stop(b),
            None => false,
        }
    }

    pub(crate) fn parse_sequence_block(&mut self) -> Result<Yaml<'a>> {
        match self.context() {
            Some(ParseContext::FlowIn)
            | Some(ParseContext::FlowKey)
            | Some(ParseContext::FlowOut) => {
                return self
                    .parse_error_with_msg("block sequences may not appear in flow collections")
            }
            _ => {}
        }
        self.start_context(ParseContextKind::Block)?;
        let indent = self.indent;
        match self.current {
            b'-' => {
                let mut seq = Vec::new();
                loop {
                    match self.current {
                        _ if self.at_end() => break,
                        b'#' => self.chomp_comment(),
                        byt if byt.is_linebreak() => {
                            self.indent = 0;
                            if self.bump_newline() {
                                continue;
                            } else {
                                break;
                            }
                        }
                        byt if byt.is_ws() => {
                            self.chomp_indent();
                        }
                        _ if self.indent < indent => break,
                        b'-' => {
                            if self.check_ahead_1(|t| t.is_linebreak()) {
                                self.advance()?;
                                self.advance()?;
                                self.indent = 0;
                                if self.current.is_ws() {
                                    self.chomp_indent();
                                    if self.indent < indent {
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
                            } else if self.check_ahead_1(|t| t.is_ws()) {
                                self.advance()?;
                                self.advance()?;
                                let node = self.parse()?;
                                seq.push(node);
                            } else {
                                return self.parse_error_with_msg("unexpected '-'");
                            }
                        }
                        _ if self.indent == indent => break,
                        _ => return self.parse_error_with_msg("expected sequence item"),
                    }
                }
                self.end_context(ParseContextKind::Block)?;
                Ok(Yaml::Sequence(seq))
            }
            // TODO: Provide error message
            _ => self.parse_error_with_msg("failed to parse block sequence"),
        }
    }

    fn check_ahead_n(&self, n: usize, stop: impl Fn(u8) -> bool) -> bool {
        match self.bytes.get(self.idx + n) {
            Some(&b) => stop(b),
            None => false,
        }
    }

    fn take_while(
        &mut self,
        accept: impl Fn(u8, Option<u8>) -> bool,
    ) -> std::result::Result<(usize, usize), (usize, usize)> {
        let start = self.idx;
        let mut end = start;
        loop {
            let peeked = self.peek();
            if !accept(self.current, peeked) {
                break;
            } else if !self.bump() {
                end += 1;
                return Err((start, end));
            }
            end += 1;
        }
        Ok((start, end))
    }

    fn pop_if_match(&mut self, expect: u8) -> Result<()> {
        match self.expected.last() {
            Some(&val) if val == expect => {
                self.expected.pop();
                Ok(())
            }
            // TODO: Provide error message
            _ => self.parse_error_with_msg("token was not expected"),
        }
    }
}
