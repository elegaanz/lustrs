pub mod ast;
pub mod lexer;
pub mod location;
pub mod parser;
mod rowan_nom;

use std::ops::Range;

use crate::lexer::Token;
use crate::rowan_nom::RowanNomError;
use lexer::LustreLang;
use logos::Logos;

/// TODO remove `Debug`
#[derive(Debug)]
pub struct Error {
    pub span: Range<usize>,
    pub msg: String,
    pub cause: Option<Box<Error>>,
}

impl RowanNomError<LustreLang> for Error {
    fn from_message(message: &str) -> Self {
        Error {
            span: 0..0,
            msg: message.to_string(),
            cause: None,
        }
    }

    fn from_unexpected_eof(position: usize) -> Self {
        Error {
            span: position..position,
            msg: "unexpected eof".to_string(),
            cause: None,
        }
    }

    fn from_unexpected_token(span: Range<usize>, expected: Token, found: Token) -> Self {
        Error {
            span,
            msg: format!("expected {expected:?}, found {found:?}"),
            cause: None,
        }
    }

    fn with_context(mut self, ctx: &'static str) -> Self {
        Error {
            span: self.span.clone(),
            msg: ctx.to_string(),
            cause: Some(Box::new(self)),
        }
    }
}

pub type SyntaxNode = rowan::SyntaxNode<LustreLang>;
pub type SyntaxToken = rowan::SyntaxToken<LustreLang>;

/// A Lustre v6 parser
pub struct Parse {
    pub root: SyntaxNode,
    pub errors: Vec<Error>,
}

impl Parse {
    pub fn parse(src: &str) -> Self {
        let lexer = lexer::Token::lexer(src).spanned();
        let tokens: Vec<_> = lexer.map(|(tok, span)| (tok, &src[span])).collect();
        parser::Parser::parse(tokens)
    }
}
