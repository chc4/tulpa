use luther;
use luther_derive;

#[derive(Lexer, Debug)]
pub enum Token {
    #[luther(regex = "function")]
    Function,
    #[luther(regex = "if")]
    If,
    #[luther(regex = "let")]
    Let,
    #[luther(regex = "do")]
    Do,
    #[luther(regex = "in")]
    In,
    #[luther(regex = "=")]
    Tis,
    #[luther(regex = "\\(")]
    Pel,
    #[luther(regex = "\\)")]
    Per,
    #[luther(regex = "\\[")]
    Sel,
    #[luther(regex = "\\]")]
    Ser,
    #[luther(regex = "::.*$")]
    Comment,
    #[luther(regex = "[ \n]+")]
    Gap, //  No switching between "tall mode" and "wide mode"
    #[luther(regex = "[0-9]+")]
    Num(String),
    #[luther(regex = "[a-zA-Z][a-zA-Z0-9]*")]
    Ident(String),
    #[luther(regex = "\"[^\"]*\"")]
    Str(String),
}

//  Spans?
#[derive(Debug,PartialEq)]
pub enum Ast {
    Func { body: Box<Ast> },
    If(Box<Ast>, Box<Ast>, Box<Ast>),
    Let(String, Box<Ast>, Box<Ast>),
    Within(Box<Ast>, Box<Ast>),
    Core(Vec<(String, Ast)>),
    Var(String),
    Str(String),
    Num(i32),
}

#[cfg(test)]
mod test {
    use super::{Ast,Token};
    use crate::parser;
    use luther::Lexer;
    use luther::spanned::StrExt;

    fn parse(input: &str) -> Option<Ast> {
        let spanned = input.spanned_chars();
        let tokens = Token::lexer(spanned).map_span(|s| s.into_inner());

        let parser = parser::TopParser::new();
        let out = parser.parse(tokens).ok();
        out
    }


    #[test]
    fn parse_num() {
        assert_eq!(parse("1"), Some(Ast::Num(1)));
    }

    #[test]
    fn parse_str() {
        assert_eq!(parse("\"AAA\""), Some(Ast::Str("AAA".to_string())));
        assert_eq!(parse("\"\""), Some(Ast::Str("".to_string())));
    }

    #[test]
    fn parse_var() {
        assert_eq!(parse("a"), Some(Ast::Var("a".to_string())));
        assert_eq!(parse("abc1"), Some(Ast::Var("abc1".to_string())));
    }

    #[test]
    fn parse_core() {
        assert_eq!(parse("[a=1]"), Some(Ast::Core(vec![("a".to_string(),Ast::Num(1))])));
        assert_eq!(parse("[a=1 b=2]"),
            Some(Ast::Core(
                vec![
                    ("a".to_string(),Ast::Num(1)),
                    ("b".to_string(),Ast::Num(2))
                ]
            )
        ));
        assert_eq!(parse("[   a =          1  b   = 2]"),
            Some(Ast::Core(
                vec![
                    ("a".to_string(),Ast::Num(1)),
                    ("b".to_string(),Ast::Num(2))
                ]
            )
        ));
    }

    #[test]
    fn parse_invalid_core() {
        assert_eq!(parse("[a=1b=2]"), None);
        assert_eq!(parse("[]"), None);
        assert_eq!(parse("[a b c]"), None);

    }

    #[test]
    fn parse_within() {
        assert_eq!(parse("do 1 in 2"),Some(Ast::Within(Box::new(Ast::Num(1)),Box::new(Ast::Num(2)))));
        assert_eq!(parse("in 2 do 1"),Some(Ast::Within(Box::new(Ast::Num(1)),Box::new(Ast::Num(2)))));
    }

    #[test]
    fn parse_let() {
        assert_eq!(parse("let a = 1 2"), Some(Ast::Let("a".to_string(), Box::new(Ast::Num(1)), Box::new(Ast::Num(2)))));
        assert_eq!(parse("let a=1 let b=2 3"), Some(Ast::Let("a".to_string(), Box::new(Ast::Num(1)), Box::new(Ast::Let("b".to_string(), Box::new(Ast::Num(2)), Box::new(Ast::Num(3)))))));
        assert_eq!(parse("let a = 1"), None);
        assert_eq!(parse("leta=1 2"), None);
    }
}
