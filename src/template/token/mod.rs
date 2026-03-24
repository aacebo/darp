mod group;
mod ident;
mod literal;
mod punct;

pub use group::*;
pub use ident::*;
pub use literal::*;
pub use punct::*;

use crate::template::LexError;

#[derive(Debug, Clone)]
pub enum Token {
    Ident(Ident),
    Literal(Literal),
    Punct(Punct),
    Group(Group),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(v) => write!(f, "{}", v),
            Self::Punct(v) => write!(f, "{}", v),
            Self::Literal(v) => write!(f, "{}", v),
            Self::Group(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Stream {
    tokens: Vec<Token>,
}

impl std::ops::Deref for Stream {
    type Target = [Token];

    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl Iterator for Stream {
    type Item = Result<Token, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl std::fmt::Display for Stream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in self.tokens.iter() {
            write!(f, "{}", token)?;
        }

        Ok(())
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Spacing {
    #[default]
    Alone,
    Joint,
}

impl Spacing {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Alone => "alone",
            Self::Joint => "joint",
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Delim {
    #[default]
    None,
    Paren,
    Brace,
    Bracket,
}

impl Delim {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Paren => "paren",
            Self::Brace => "brace",
            Self::Bracket => "bracket",
            Self::None => "none",
        }
    }

    pub fn open(&self) -> char {
        match self {
            Self::None => ' ',
            Self::Brace => '{',
            Self::Bracket => '[',
            Self::Paren => '(',
        }
    }

    pub fn close(&self) -> char {
        match self {
            Self::None => ' ',
            Self::Brace => '}',
            Self::Bracket => ']',
            Self::Paren => ')',
        }
    }

    pub fn from_open(ch: char) -> Option<Self> {
        match ch {
            '(' => Some(Self::Paren),
            '[' => Some(Self::Bracket),
            '{' => Some(Self::Brace),
            _ => None,
        }
    }

    pub fn from_close(ch: char) -> Option<Self> {
        match ch {
            ')' => Some(Self::Paren),
            ']' => Some(Self::Bracket),
            '}' => Some(Self::Brace),
            _ => None,
        }
    }
}
