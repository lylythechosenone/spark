use logos::{Lexer, Logos};

fn parse_int<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> u64 {
    let s = lexer.slice();
    let mut base = 10;
    let mut start = 0;
    if s.starts_with("0x") {
        base = 16;
        start = 2;
    } else if s.starts_with("0b") {
        base = 2;
        start = 2;
    } else if s.starts_with("0o") {
        base = 8;
        start = 2;
    }
    let mut num = 0;
    for c in s[start..].chars() {
        if c == '_' {
            continue;
        }
        num *= base;
        num += c.to_digit(base as u32).unwrap() as u64;
    }
    num
}

fn parse_float<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> f64 {
    let s = lexer.slice();
    let mut num = 0.0;
    let mut denom = 1.0;
    let mut in_decimal = false;
    for c in s.chars() {
        if c == '_' {
            continue;
        }
        if c == '.' {
            in_decimal = true;
            continue;
        }
        if in_decimal {
            denom *= 10.0;
            num += c.to_digit(10).unwrap() as f64 / denom;
        } else {
            num *= 10.0;
            num += c.to_digit(10).unwrap() as f64;
        }
    }
    num
}

#[derive(Logos, Clone, Copy, PartialEq)]
pub enum Token<'a> {
    // keywords
    #[token("include")]
    KwInclude,
    #[token("package")]
    KwPackage,
    #[token("channel")]
    KwChannel,
    #[token("json")]
    KwJson,
    #[token("toml")]
    KwToml,
    // delimiters
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    // separators
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token(";")]
    Semi,
    // arithmetic
    #[token("+")]
    Plus,
    #[token("+=")]
    PlusEquals,
    #[token("-")]
    Minus,
    #[token("-=")]
    MinusEquals,
    #[token("*")]
    Mul,
    #[token("*=")]
    MulEquals,
    #[token("/")]
    Div,
    #[token("/=")]
    DivEquals,
    #[token("%")]
    Mod,
    #[token("%=")]
    ModEquals,
    #[token("!")]
    Not,
    // bitwise
    #[token("&")]
    BitAnd,
    #[token("&=")]
    BitAndEquals,
    #[token("|")]
    BitOr,
    #[token("|=")]
    BitOrEquals,
    // logical
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    // comparison
    #[token("==")]
    Eq,
    #[token("!=")]
    Neq,
    #[token("<")]
    Lt,
    #[token("<=")]
    Lte,
    #[token(">")]
    Gt,
    #[token(">=")]
    Gte,
    // assignment
    #[token("=")]
    Equals,
    // literals
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident(&'a str),
    #[regex(r"(0x|0b|0o)?[0-9][0-9_]*", parse_int)]
    Int(u64),
    #[regex(r"(\.[0-9][0-9_]*|[0-9][0-9_]*\.[0-9][0-9_]*)", parse_float)]
    Float(f64),
    #[regex(r#""([^"]|\\")+""#, |lex| &lex.slice()[1..lex.slice().len() - 1])]
    String(&'a str),
}
