use super::parse::ParseContext;

pub(crate) trait ByteExt {
    fn is_indicator(self) -> bool;
    fn is_linebreak(self) -> bool;
    fn is_ws(self) -> bool;
    fn is_ns_char(self) -> bool;
    fn is_ns_plain_safe(self, ctx: Option<ParseContext>) -> bool;
    fn is_flow_indicator(self) -> bool;
    fn is_scalar_start(self, next: Option<u8>, ctx: Option<ParseContext>) -> bool;
    fn is_safe(&self, group: CharGroup) -> bool;
}

impl ByteExt for u8 {
    fn is_indicator(self) -> bool {
        matches!(
            self,
            b'-' | b'?'
                | b':'
                | b','
                | b'['
                | b']'
                | b'{'
                | b'}'
                | b'&'
                | b'*'
                | b'!'
                | b'|'
                | b'#'
                | b'>'
                | b'\"'
                | b'\''
                | b'%'
                | b'@'
                | b'`'
        )
    }
    fn is_linebreak(self) -> bool {
        matches!(self, b'\n' | b'\r')
    }

    fn is_ws(self) -> bool {
        matches!(self, b' ' | b'\t')
    }

    fn is_ns_char(self) -> bool {
        !self.is_ws()
    }

    fn is_flow_indicator(self) -> bool {
        matches!(self, b'[' | b']' | b'{' | b'}' | b',')
    }

    fn is_ns_plain_safe(self, ctx: Option<ParseContext>) -> bool {
        match ctx {
            Some(ParseContext::FlowIn) | Some(ParseContext::FlowKey) => {
                self.is_ns_char() && !self.is_flow_indicator() && !self.is_linebreak()
            }
            _ => self.is_ns_char() && !self.is_linebreak(),
        }
    }

    fn is_scalar_start(self, next: Option<u8>, ctx: Option<ParseContext>) -> bool {
        if self.is_linebreak() {
            return false;
        }
        if self.is_ns_char() && !self.is_indicator() {
            return true;
        }
        match self {
            b'\"' | b'\'' => true,
            b'?' | b':' | b'-' => match next {
                Some(byt) => byt.is_ns_plain_safe(ctx),
                None => false,
            },
            _ => false,
        }
    }

    fn is_safe(&self, group: CharGroup) -> bool {
        match group {
            CharGroup::NSPlainIn => {
                self.is_ns_char() && !self.is_flow_indicator() && !self.is_linebreak()
            }
            CharGroup::NSPlainOut => self.is_ns_char() && !self.is_linebreak(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub(crate) enum CharGroup {
    NSPlainOut,
    NSPlainIn,
}
