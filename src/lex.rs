use super::syntax;

#[derive(Debug, Clone, Copy)]
pub struct Token<'source> {
    pub kind: syntax::SyntaxKind,
    pub text: &'source str,
}

impl From<Token<'_>> for rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken> {
    fn from(token: Token<'_>) -> Self {
        use rowan::Language;
        rowan::GreenToken::new(syntax::Language::kind_to_raw(token.kind), token.text.into()).into()
    }
}

pub struct Lexer<'source> {
    lexer: logos::Lexer<'source, syntax::SyntaxKind>,
}

impl<'source> Lexer<'source> {
    pub fn new(input: &'source str) -> Self {
        use logos::Logos;
        Self {
            lexer: syntax::SyntaxKind::lexer(input),
        }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = (usize, Token<'source>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.lexer.next()?;
        let text = self.lexer.slice();
        let span = self.lexer.span();
        Some((span.start, Token { kind, text }, span.end))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NoError {}
