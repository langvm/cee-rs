// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use crate::scanner::BasicToken::IntFormat;
use crate::scanner::PosRange::PosRange;

#[derive(Clone)]
pub enum TokenKind {
    None,

    Ident,
    Operator,

    Int(IntFormat),
    String,
    Char,

    Delimiter,

    Comment,

    // Keywords

    LPAREN,
    LBRACK,
    LBRACE,
    RPAREN,
    RBRACK,
    RBRACE,
    COMMA,
    SEMICOLON,
    COLON,
    NEWLINE,
}

pub struct Token {
    pub Pos: PosRange,
    pub Kind: TokenKind,
    pub Literal: Vec<char>,
}

impl Token {
    pub fn clone(&self) -> Token {
        Token {
            Pos: self.Pos.clone(),
            Kind: self.Kind.clone(),
            Literal: self.Literal.clone(),
        }
    }
}

pub fn KeywordLookup() -> HashMap<String, TokenKind> {
    HashMap::from([
        ("(".to_string(), TokenKind::LPAREN),
        ("[".to_string(), TokenKind::LBRACK),
        ("{".to_string(), TokenKind::LBRACE),
        (")".to_string(), TokenKind::RPAREN),
        ("]".to_string(), TokenKind::RBRACK),
        ("}".to_string(), TokenKind::RBRACE),
        (",".to_string(), TokenKind::COMMA),
        (";".to_string(), TokenKind::SEMICOLON),
        (":".to_string(), TokenKind::COLON),
        ("\n".to_string(), TokenKind::NEWLINE),
    ])
}